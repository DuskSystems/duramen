use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

use duramen_ast as ast;
use duramen_cst::{self as cst, CstNode as _};
use duramen_diagnostic::{Diagnostic, Diagnostics, Suggestion};
use duramen_escape::Escaper;
use duramen_syntax::Syntax;

use crate::{EXTENSION_FUNCTIONS, LowerContext};

/// Policy lowerer for CST-to-AST transformation.
pub struct PolicyLowerer<'a, 'src> {
    ctx: LowerContext<'a, 'src>,
}

impl<'a, 'src> PolicyLowerer<'a, 'src> {
    /// Creates a new policy lowerer.
    #[must_use]
    pub const fn new(source: &'src str, diagnostics: &'a mut Diagnostics) -> Self {
        Self {
            ctx: LowerContext::new(source, diagnostics),
        }
    }

    /// Lowers a CST policies node to an AST.
    #[must_use]
    pub fn lower(mut self, policies: cst::Policies<'src>) -> ast::Policies<'src> {
        let mut result = Vec::new();

        for policy in policies.policies() {
            if let Some(lowered) = self.lower_policy(&policy) {
                result.push(lowered);
            }
        }

        ast::Policies::new(result)
    }

    /// Lowers a single policy.
    fn lower_policy(&mut self, policy: &cst::Policy<'src>) -> Option<ast::Policy<'src>> {
        let node = policy.syntax();

        if let Some(effect_token) = policy.effect_token() {
            let effect_start = effect_token.range().start;

            for child in node.children() {
                if child.kind().is_error() && child.range().start < effect_start {
                    if let Some(diagnostic) = LowerContext::recognize_error(child) {
                        self.ctx.diagnostic(diagnostic);
                    } else {
                        let span = child
                            .children()
                            .find(|c| !c.kind().is_trivial())
                            .map_or_else(|| child.range(), |first| first.range());

                        let text = &self.ctx.source[span.clone()];
                        self.ctx.diagnostic(
                            Diagnostic::error(format!("unexpected `{text}`"))
                                .with_label(span, "expected `permit`, `forbid`, or `@`"),
                        );
                    }

                    return None;
                }
            }
        }

        let mut scope_has_errors = false;

        if let Some(scope) = policy.scope() {
            let node = scope.syntax();

            if node.child(Syntax::OpenParenthesis).is_some()
                && node.child(Syntax::CloseParenthesis).is_none()
            {
                if let Some(diagnostic) = node
                    .descendants()
                    .filter(|child| child.kind().is_error())
                    .find_map(LowerContext::recognize_error)
                {
                    self.ctx.diagnostic(diagnostic);
                    return None;
                }

                let span = node
                    .after(Syntax::OpenParenthesis)
                    .find(|child| !child.kind().is_trivial())
                    .map_or_else(
                        || {
                            let end = node.range().end;
                            end..end
                        },
                        |child| child.first().range(),
                    );

                self.ctx
                    .diagnostic(Diagnostic::error("expected `)`").with_label(span, "expected `)`"));

                return None;
            }

            for child in node
                .after(Syntax::OpenParenthesis)
                .take_while(|child| child.kind() != Syntax::CloseParenthesis)
            {
                if child.kind().is_error() {
                    scope_has_errors = true;

                    if let Some(diagnostic) = LowerContext::recognize_error(child) {
                        self.ctx.diagnostic(diagnostic);
                    } else {
                        self.ctx.diagnostic(
                            Diagnostic::error("expected scope variable")
                                .with_label(child.range(), "not valid in scope")
                                .with_note(
                                    "`principal`, `action`, and `resource` are the only scope variables",
                                ),
                        );
                    }
                }
            }
        }

        let annotations = self.ctx.lower_annotations(policy.annotations())?;

        let effect = policy.effect().map(|effect| match effect {
            cst::Effect::Permit => ast::Effect::Permit,
            cst::Effect::Forbid => ast::Effect::Forbid,
        });

        let Some(effect) = effect else {
            for child in node.children() {
                if child.kind().is_error() {
                    if let Some(diagnostic) = LowerContext::recognize_error(child) {
                        self.ctx.diagnostic(diagnostic);
                        return None;
                    }

                    let span = child
                        .children()
                        .find(|c| !c.kind().is_trivial())
                        .map_or_else(|| child.range(), |first| first.range());

                    let text = &self.ctx.source[span.clone()];
                    self.ctx.diagnostic(
                        Diagnostic::error(format!("unexpected `{text}`"))
                            .with_label(span, "expected `permit`, `forbid`, or `@`"),
                    );

                    return None;
                }
            }

            self.ctx.diagnostic(
                Diagnostic::error("missing effect")
                    .with_label(policy.range(), "expected `permit`, `forbid`, or `@`"),
            );

            return None;
        };

        let mut principal = None;
        let mut action = None;
        let mut resource = None;

        let variable_definitions = policy
            .scope()
            .into_iter()
            .flat_map(|scope| scope.variable_definitions());

        for variable_definition in variable_definitions {
            let Some(variable) = variable_definition.variable() else {
                if let Some(token) = variable_definition.variable_token() {
                    let text = &self.ctx.source[token.range()];
                    self.ctx.diagnostic(
                        Diagnostic::error(format!("unknown scope variable `{text}`"))
                            .with_label(token.range(), "not a valid scope variable")
                            .with_note(
                                "`principal`, `action`, and `resource` are the only scope variables",
                            ),
                    );
                }

                continue;
            };

            let checkpoint = self.ctx.diagnostics.len();

            match variable {
                cst::Variable::Principal => {
                    principal = Some(
                        self.lower_scope_constraint(&variable_definition, "principal")
                            .map(ast::PrincipalConstraint::new),
                    );
                }
                cst::Variable::Action => {
                    action = Some(self.lower_action_constraint(&variable_definition));
                }
                cst::Variable::Resource => {
                    resource = Some(
                        self.lower_scope_constraint(&variable_definition, "resource")
                            .map(ast::ResourceConstraint::new),
                    );
                }
                cst::Variable::Context => {
                    self.ctx.diagnostic(
                        Diagnostic::error("`context` is not a scope variable")
                            .with_label(variable_definition.range(), "not valid in scope")
                            .with_note(
                                "`context` can only be used in conditions, not in the scope",
                            ),
                    );
                }
            }

            // When the scope has error nodes, variable definitions adjacent to those
            // errors may be incomplete (e.g. `Photo::` without a string because a
            // single-quoted string was recovered into an error node). Suppress the
            // cascading diagnostic and treat the variable as unspecified.
            if scope_has_errors {
                let failed = match variable {
                    cst::Variable::Principal => matches!(principal, Some(None)),
                    cst::Variable::Action => matches!(action, Some(None)),
                    cst::Variable::Resource => matches!(resource, Some(None)),
                    cst::Variable::Context => false,
                };

                if failed {
                    self.ctx.diagnostics.truncate(checkpoint);
                    match variable {
                        cst::Variable::Principal => principal = None,
                        cst::Variable::Action => action = None,
                        cst::Variable::Resource => resource = None,
                        cst::Variable::Context => {}
                    }
                }
            }
        }

        let principal = match principal {
            Some(Some(constraint)) => constraint,
            Some(None) => return None,
            None => ast::PrincipalConstraint::new(ast::ScopeConstraint::Any),
        };

        let action = match action {
            Some(Some(constraint)) => constraint,
            Some(None) => return None,
            None => ast::ActionConstraint::Any,
        };

        let resource = match resource {
            Some(Some(constraint)) => constraint,
            Some(None) => return None,
            None => ast::ResourceConstraint::new(ast::ScopeConstraint::Any),
        };

        let mut conditions = Vec::new();
        for condition in policy.conditions() {
            if let Some(lowered) = self.lower_condition(&condition) {
                conditions.push(lowered);
            }
        }

        // Report trailing error nodes (e.g. `advice { ... }` after conditions).
        for child in node.children() {
            if child.kind().is_error() {
                if let Some(diagnostic) = LowerContext::recognize_error(child) {
                    self.ctx.diagnostic(diagnostic);
                } else if let Some(first) = child.children().find(|c| !c.kind().is_trivial())
                    && first.kind().is_identifier()
                {
                    let text = &self.ctx.source[first.range()];
                    self.ctx.diagnostic(
                        Diagnostic::error(format!("invalid policy condition `{text}`"))
                            .with_label(first.range(), "not a valid condition")
                            .with_help("condition must be either `when` or `unless`"),
                    );
                } else {
                    let span = child
                        .children()
                        .filter(|c| !c.kind().is_trivial())
                        .last()
                        .map_or_else(
                            || child.range(),
                            |last| child.range().start..last.range().end,
                        );

                    let text = &self.ctx.source[span.clone()];
                    self.ctx.diagnostic(
                        Diagnostic::error(format!("unexpected `{text}`"))
                            .with_label(span, "not valid in a policy"),
                    );
                }
            }
        }

        Some(ast::Policy::new(
            annotations,
            effect,
            principal,
            action,
            resource,
            conditions,
        ))
    }

    /// Lowers a principal or resource scope constraint.
    fn lower_scope_constraint(
        &mut self,
        variable_definition: &cst::VariableDefinition<'_>,
        variable_name: &str,
    ) -> Option<ast::ScopeConstraint<'src>> {
        let has_type_test = variable_definition.is_token().is_some();
        let has_membership = variable_definition.in_token().is_some();
        let has_operator = variable_definition.operator_token().is_some();

        if let Some(colon) = variable_definition.colon() {
            self.ctx.diagnostic(
                Diagnostic::error("type constraints using `:` are not supported")
                    .with_label(colon.range(), "not a valid scope operator")
                    .with_help("use `is` for type constraints"),
            );

            return Some(ast::ScopeConstraint::Any);
        }

        if !has_type_test && !has_membership && !has_operator {
            return Some(ast::ScopeConstraint::Any);
        }

        if has_type_test && has_membership {
            let kind_name = variable_definition.kind()?;
            let kind = self.ctx.lower_name(&kind_name)?;
            let entity_or_slot = self.lower_entity_or_slot_expression(variable_definition)?;
            return Some(ast::ScopeConstraint::IsIn(kind, entity_or_slot));
        }

        if has_type_test {
            let kind_name = variable_definition.kind()?;
            let kind = self.ctx.lower_name(&kind_name)?;
            return Some(ast::ScopeConstraint::Is(kind));
        }

        let operator_token = variable_definition.operator_token()?;

        match operator_token.kind() {
            Syntax::Equal => {
                let entity_or_slot = self.lower_entity_or_slot_expression(variable_definition)?;
                Some(ast::ScopeConstraint::Equal(entity_or_slot))
            }
            Syntax::InKeyword => {
                let entity_or_slot = self.lower_entity_or_slot_expression(variable_definition)?;
                Some(ast::ScopeConstraint::In(entity_or_slot))
            }
            _ => {
                let label = if variable_name == "action" {
                    "expected `==` or `in`"
                } else {
                    "expected `==`, `in`, `is`, or `is ... in`"
                };

                self.ctx.diagnostic(
                    Diagnostic::error(format!("invalid scope operator for `{variable_name}`"))
                        .with_label(operator_token.range(), label),
                );

                None
            }
        }
    }

    /// Extracts an entity reference or slot from a variable definition's expression.
    fn lower_entity_or_slot_expression(
        &mut self,
        variable_definition: &cst::VariableDefinition<'_>,
    ) -> Option<ast::EntityOrSlot<'src>> {
        if let Some(slot) = variable_definition.slot() {
            return self.lower_slot_to_entity_or_slot(&slot);
        }

        let expression = variable_definition.expression()?;
        self.lower_entity_or_slot(&expression)
    }

    /// Lowers a slot to `EntityOrSlot`.
    fn lower_slot_to_entity_or_slot(
        &mut self,
        slot: &cst::Slot<'_>,
    ) -> Option<ast::EntityOrSlot<'src>> {
        let node = slot.name()?;
        let text = self.ctx.text(node);

        match ast::SlotKind::new(text) {
            Ok(_) => Some(ast::EntityOrSlot::Slot),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers an expression to `EntityOrSlot`.
    fn lower_entity_or_slot(
        &mut self,
        expression: &cst::Expression<'_>,
    ) -> Option<ast::EntityOrSlot<'src>> {
        match expression {
            cst::Expression::Slot(slot) => self.lower_slot_to_entity_or_slot(slot),
            cst::Expression::EntityReference(entity_reference) => {
                let reference = self.lower_entity_reference(entity_reference)?;
                Some(ast::EntityOrSlot::Entity(reference))
            }
            _ => {
                self.ctx.diagnostic(
                    Diagnostic::error("missing expression")
                        .with_label(expression.range(), "expected an entity reference or slot"),
                );

                None
            }
        }
    }

    /// Lowers an action scope constraint.
    fn lower_action_constraint(
        &mut self,
        variable_definition: &cst::VariableDefinition<'_>,
    ) -> Option<ast::ActionConstraint<'src>> {
        if let Some(is_token) = variable_definition.is_token() {
            self.ctx.diagnostic(
                Diagnostic::error("`is` cannot appear in the `action` scope")
                    .with_label(is_token.range(), "not valid for `action`")
                    .with_help("try moving `action is ..` into a `when` condition"),
            );

            return Some(ast::ActionConstraint::Any);
        }

        if let Some(colon) = variable_definition.colon() {
            self.ctx.diagnostic(
                Diagnostic::error("type constraints using `:` are not supported")
                    .with_label(colon.range(), "not a valid scope operator")
                    .with_help("use `is` for type constraints"),
            );

            return Some(ast::ActionConstraint::Any);
        }

        let Some(operator_token) = variable_definition.operator_token() else {
            return Some(ast::ActionConstraint::Any);
        };

        match operator_token.kind() {
            Syntax::Equal => {
                let expression = variable_definition.expression()?;
                let entity_reference = self.lower_action_entity_reference(&expression)?;
                Some(ast::ActionConstraint::Equal(entity_reference))
            }
            Syntax::InKeyword => self.lower_action_in_constraint(variable_definition),
            _ => {
                self.ctx.diagnostic(
                    Diagnostic::error("invalid scope operator for `action`")
                        .with_label(operator_token.range(), "expected `==` or `in`"),
                );

                None
            }
        }
    }

    /// Lowers `action in ...` constraint.
    fn lower_action_in_constraint(
        &mut self,
        variable_definition: &cst::VariableDefinition<'_>,
    ) -> Option<ast::ActionConstraint<'src>> {
        let expression = variable_definition.expression()?;

        if let cst::Expression::List(list) = &expression {
            let mut actions = Vec::new();
            if let Some(arguments) = list.arguments() {
                for item in arguments.expressions() {
                    if let Some(entity_reference) = self.lower_action_entity_reference(&item) {
                        actions.push(entity_reference);
                    }
                }
            }

            match ast::ActionList::new(actions) {
                Ok(list) => Some(ast::ActionConstraint::In(list)),
                Err(error) => {
                    self.ctx.diagnostic(error);
                    None
                }
            }
        } else {
            let entity_reference = self.lower_action_entity_reference(&expression)?;

            match ast::ActionList::new(vec![entity_reference]) {
                Ok(list) => Some(ast::ActionConstraint::In(list)),
                Err(error) => {
                    self.ctx.diagnostic(error);
                    None
                }
            }
        }
    }

    /// Lowers an expression to an action entity reference.
    fn lower_action_entity_reference(
        &mut self,
        expression: &cst::Expression<'_>,
    ) -> Option<ast::EntityReference<'src>> {
        if let cst::Expression::EntityReference(entity_reference) = expression {
            self.lower_entity_reference(entity_reference)
        } else {
            self.ctx.diagnostic(
                Diagnostic::error("missing expression")
                    .with_label(expression.range(), "expected an entity reference"),
            );

            None
        }
    }

    /// Lowers an entity reference.
    fn lower_entity_reference(
        &mut self,
        entity_reference: &cst::EntityReference<'_>,
    ) -> Option<ast::EntityReference<'src>> {
        let kind_name = entity_reference.kind()?;
        let kind = self.ctx.lower_name(&kind_name)?;

        let id_node = entity_reference.id()?;
        let id = self.ctx.lower_string(id_node)?;

        Some(ast::EntityReference::new(kind, id))
    }

    /// Lowers a condition.
    fn lower_condition(
        &mut self,
        condition: &cst::Condition<'src>,
    ) -> Option<ast::Condition<'src>> {
        let node = condition.syntax();
        if node.child(Syntax::OpenBrace).is_some() && node.child(Syntax::CloseBrace).is_none() {
            if let Some(diagnostic) = node
                .descendants()
                .filter(|child| child.kind().is_error())
                .find_map(LowerContext::recognize_error)
            {
                self.ctx.diagnostic(diagnostic);
                return None;
            }

            let span = node
                .after(Syntax::OpenBrace)
                .find(|child| !child.kind().is_trivial())
                .map_or_else(
                    || {
                        let end = node.range().end;
                        end..end
                    },
                    |child| child.first().range(),
                );

            self.ctx
                .diagnostic(Diagnostic::error("expected `}`").with_label(span, "expected `}`"));

            return None;
        }

        let kind = condition.kind().map(|kind| match kind {
            cst::ConditionKind::When => ast::ConditionKind::When,
            cst::ConditionKind::Unless => ast::ConditionKind::Unless,
        })?;

        let body = condition.body();

        if body.is_none() {
            self.ctx.emit_error(node, "condition");
            return None;
        }

        let body = self.lower_expression(&body?)?;

        // Check for error nodes between { and } (e.g. block comments, leftover tokens)
        if node.child(Syntax::OpenBrace).is_some()
            && node.child(Syntax::CloseBrace).is_some()
            && let Some(error_node) = node
                .after(Syntax::OpenBrace)
                .take_while(|child| child.kind() != Syntax::CloseBrace)
                .find(|child| child.kind().is_error())
        {
            if let Some(diagnostic) = LowerContext::recognize_error(error_node) {
                self.ctx.diagnostic(diagnostic);
            } else {
                let text = self.ctx.source[error_node.range()].trim_end();
                self.ctx.diagnostic(
                    Diagnostic::error(format!("unexpected token `{text}`"))
                        .with_label(error_node.range(), "not valid in condition"),
                );
            }

            return None;
        }

        Some(ast::Condition::new(kind, body))
    }

    /// Lowers an expression.
    fn lower_expression(
        &mut self,
        expression: &cst::Expression<'src>,
    ) -> Option<ast::Expression<'src>> {
        match expression {
            cst::Expression::If(if_expression) => self.lower_if(if_expression),
            cst::Expression::Or(or_expression) => self.lower_or(or_expression),
            cst::Expression::And(and_expression) => self.lower_and(and_expression),
            cst::Expression::Relation(relation) => self.lower_relation(relation),
            cst::Expression::Sum(sum_expression) => self.lower_sum(sum_expression),
            cst::Expression::Product(product) => self.lower_product(product),
            cst::Expression::Has(has_expression) => self.lower_has(has_expression),
            cst::Expression::Like(like_expression) => self.lower_like(like_expression),
            cst::Expression::Is(is_expression) => self.lower_is(is_expression),
            cst::Expression::Unary(unary_expression) => self.lower_unary(unary_expression),
            cst::Expression::Member(member_expression) => self.lower_member(member_expression),
            cst::Expression::Literal(literal) => self.lower_literal(literal),
            cst::Expression::EntityReference(entity_reference) => {
                let reference = self.lower_entity_reference(entity_reference)?;
                Some(ast::Expression::entity(reference))
            }
            cst::Expression::Slot(slot) => self.lower_slot(slot),
            cst::Expression::Parenthesized(paren) => {
                let inner = paren.expression()?;
                self.lower_expression(&inner)
            }
            cst::Expression::List(list) => Some(self.lower_list(list)),
            cst::Expression::Record(record) => self.lower_record(record),
            cst::Expression::Name(name) => self.lower_name_expression(name),
        }
    }

    /// Lowers an if expression.
    fn lower_if(&mut self, expression: &cst::IfExpression<'src>) -> Option<ast::Expression<'src>> {
        let has_test = expression.test().is_some();

        if !has_test {
            let node = expression.syntax();
            let start = expression
                .if_token()
                .map_or_else(|| node.range().start, |token| token.range().start);
            let end = node.range().start + node.text().trim_end().len();

            self.ctx.diagnostic(
                Diagnostic::error("invalid `if` expression")
                    .with_label(start..end, "expected `if <expr> then <expr> else <expr>`"),
            );

            return None;
        }

        if expression.then_token().is_none() {
            let node = expression.syntax();
            let end = node.range().start + node.text().trim_end().len();

            self.ctx.diagnostic(
                Diagnostic::error("incomplete `if` expression")
                    .with_label(node.range().start..end, "expected `then`"),
            );

            return None;
        }

        let has_consequent = expression.consequent().is_some();
        let has_else = expression.else_token().is_some();
        let has_alternate = expression.alternate().is_some();

        if !has_consequent || !has_else || !has_alternate {
            let missing = match (has_consequent, has_else, has_alternate) {
                (false, _, _) => "expected expression after `then`",
                (true, false, _) => "expected `else`",
                (true, true, false) => "expected expression after `else`",
                _ => "incomplete `if` expression",
            };

            let end = expression.range().end;
            self.ctx.diagnostic(
                Diagnostic::error("incomplete `if` expression").with_label(end..end, missing),
            );

            return None;
        }

        let test = expression.test()?;
        let consequent = expression.consequent()?;
        let alternate = expression.alternate()?;

        let test = self.lower_expression(&test)?;
        let consequent = self.lower_expression(&consequent)?;
        let alternate = self.lower_expression(&alternate)?;

        Some(ast::Expression::if_then_else(test, consequent, alternate))
    }

    /// Lowers an or expression.
    fn lower_or(&mut self, expression: &cst::OrExpression<'src>) -> Option<ast::Expression<'src>> {
        let left = expression.left()?;
        let left = self.lower_expression(&left)?;

        let right = expression.right()?;
        let right = self.lower_expression(&right)?;

        Some(ast::Expression::or(left, right))
    }

    /// Lowers an and expression.
    fn lower_and(
        &mut self,
        expression: &cst::AndExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let left = expression.left()?;
        let left = self.lower_expression(&left)?;

        let right = expression.right()?;
        let right = self.lower_expression(&right)?;

        Some(ast::Expression::and(left, right))
    }

    /// Lowers a relation expression.
    fn lower_relation(
        &mut self,
        expression: &cst::RelationExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        if let Some(diagnostic) = expression
            .syntax()
            .descendants()
            .filter(|child| child.kind().is_error())
            .find_map(LowerContext::recognize_error)
        {
            self.ctx.diagnostic(diagnostic);
            return None;
        }

        let Some(operator) = expression.operator() else {
            if let Some(token) = expression.operator_token()
                && token.kind() == Syntax::Assign
            {
                let suggestion =
                    Suggestion::fix(token.range(), "==").with_message("use `==` for equality");

                self.ctx.diagnostic(
                    Diagnostic::error("invalid operator `=`")
                        .with_label(token.range(), "not a valid operator")
                        .with_suggestion(suggestion),
                );
            }

            return None;
        };

        let left = expression.left()?;
        let left = self.lower_expression(&left)?;

        let right = expression.right()?;
        let right = self.lower_expression(&right)?;

        let operator = match operator {
            cst::RelationOperator::Less => ast::BinaryOperator::Less,
            cst::RelationOperator::LessEqual => ast::BinaryOperator::LessEqual,
            cst::RelationOperator::Greater => ast::BinaryOperator::Greater,
            cst::RelationOperator::GreaterEqual => ast::BinaryOperator::GreaterEqual,
            cst::RelationOperator::Equal => ast::BinaryOperator::Equal,
            cst::RelationOperator::NotEqual => ast::BinaryOperator::NotEqual,
            cst::RelationOperator::In => ast::BinaryOperator::In,
        };

        Some(ast::Expression::binary(operator, left, right))
    }

    /// Lowers a sum expression.
    fn lower_sum(
        &mut self,
        expression: &cst::SumExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let left = expression.left()?;
        let left = self.lower_expression(&left)?;

        let right = expression.right()?;
        let right = self.lower_expression(&right)?;

        let operator = expression.operator()?;
        let operator = match operator {
            cst::SumOperator::Add => ast::BinaryOperator::Add,
            cst::SumOperator::Subtract => ast::BinaryOperator::Subtract,
        };

        Some(ast::Expression::binary(operator, left, right))
    }

    /// Lowers a product expression.
    fn lower_product(
        &mut self,
        expression: &cst::ProductExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let operator = expression.operator()?;
        match operator {
            cst::ProductOperator::Multiply => {
                let left = expression.left()?;
                let left = self.lower_expression(&left)?;

                let right = expression.right()?;
                let right = self.lower_expression(&right)?;

                Some(ast::Expression::binary(
                    ast::BinaryOperator::Multiply,
                    left,
                    right,
                ))
            }
            cst::ProductOperator::Divide | cst::ProductOperator::Modulo => {
                self.ctx.diagnostic(
                    Diagnostic::error("division and remainder are not supported")
                        .with_label(expression.range(), "not supported")
                        .with_note("only `*` with an integer literal is allowed"),
                );

                None
            }
        }
    }

    /// Lowers a has expression.
    fn lower_has(
        &mut self,
        expression: &cst::HasExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let left = expression.expression()?;
        let left = self.lower_expression(&left)?;

        let attribute = expression.attribute()?;
        let attribute = self.extract_attribute_name(&attribute)?;

        Some(ast::Expression::has_attribute(left, attribute))
    }

    /// Extracts an attribute name from either an identifier or a string literal.
    fn extract_attribute_name(
        &mut self,
        expression: &cst::Expression<'src>,
    ) -> Option<Cow<'src, str>> {
        match expression {
            cst::Expression::Literal(literal) => {
                let token = literal.token()?;
                if token.kind() == Syntax::String {
                    self.ctx.lower_string(token)
                } else {
                    Some(Cow::Borrowed(self.ctx.text(token)))
                }
            }
            cst::Expression::Name(name) => {
                let text = name.basename(self.ctx.source)?;
                Some(Cow::Borrowed(text))
            }
            cst::Expression::Member(member) => {
                let span = member.accesses().next().map_or_else(
                    || expression.range(),
                    |access| {
                        let node = access.syntax();
                        let start = node
                            .children()
                            .find(|child| child.kind() != Syntax::Dot && !child.kind().is_trivial())
                            .map_or_else(|| node.range().start, |first| first.range().start);

                        let end = node
                            .children()
                            .filter(|child| !child.kind().is_trivial())
                            .last()
                            .map_or_else(|| node.range().end, |last| last.range().end);

                        start..end
                    },
                );

                self.ctx.diagnostic(
                    Diagnostic::error("expected an attribute name")
                        .with_label(span, "unexpected field access"),
                );

                None
            }
            _ => {
                self.ctx.diagnostic(
                    Diagnostic::error("missing expression")
                        .with_label(expression.range(), "expected an attribute name"),
                );

                None
            }
        }
    }

    /// Lowers a like expression.
    fn lower_like(
        &mut self,
        expression: &cst::LikeExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let left = expression.expression()?;
        let left = self.lower_expression(&left)?;

        let pattern = expression.pattern()?;
        let pattern = pattern.syntax();

        let raw = self.ctx.text(pattern);
        let offset = pattern.range().start;

        match Escaper::new(raw).unescape_pattern() {
            Ok(elements) => {
                let pattern = ast::Pattern::new(elements);
                Some(ast::Expression::like(left, pattern))
            }
            Err(errors) => {
                for error in errors {
                    self.ctx.diagnostic(error.offset(offset));
                }

                None
            }
        }
    }

    /// Lowers an is expression.
    fn lower_is(&mut self, expression: &cst::IsExpression<'src>) -> Option<ast::Expression<'src>> {
        let left = expression.expression()?;
        let left = self.lower_expression(&left)?;

        let kind = expression.kind()?;
        let kind = self.ctx.lower_name(&kind)?;

        if let Some(target) = expression.target() {
            let target = self.lower_expression(&target)?;
            Some(ast::Expression::is_in(left, kind, target))
        } else {
            Some(ast::Expression::is(left, kind))
        }
    }

    /// Lowers a unary expression.
    fn lower_unary(
        &mut self,
        expression: &cst::UnaryExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let operator = expression.operator()?;

        let operator_count = expression.operator_tokens().count();
        if operator_count > 4 {
            self.ctx.diagnostic(
                Diagnostic::error(format!("found {operator_count} chained unary operators"))
                    .with_label(expression.range(), "at most 4 allowed"),
            );

            return None;
        }

        let operand = expression.operand()?;
        if operator == cst::UnaryOperator::Negate
            && operator_count == 1
            && let cst::Expression::Literal(literal) = &operand
            && literal.kind() == Some(cst::LiteralKind::Integer)
            && let Some(token) = literal.token()
        {
            let text = self.ctx.text(token);

            let mut negated = String::from("-");
            negated.push_str(text);

            return if let Ok(literal) = ast::IntegerLiteral::new(&negated) {
                Some(ast::Expression::integer(literal))
            } else {
                self.ctx.diagnostic(
                    Diagnostic::error(format!("integer literal `{negated}` is out of range"))
                        .with_label(
                            expression.range().start..token.range().end,
                            "out of range for a 64-bit integer",
                        ),
                );
                None
            };
        }

        let operand = self.lower_expression(&operand)?;

        let ast_operator = match operator {
            cst::UnaryOperator::Not => ast::UnaryOperator::Not,
            cst::UnaryOperator::Negate => ast::UnaryOperator::Negate,
        };

        let mut result = ast::Expression::unary(ast_operator, operand);
        for _ in 1..operator_count {
            result = ast::Expression::unary(ast_operator, result);
        }

        Some(result)
    }

    /// Lowers a member expression (base + chain of accesses).
    fn lower_member(
        &mut self,
        expression: &cst::MemberExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let base = expression.expression()?;
        let accesses: Vec<_> = expression.accesses().collect();

        if let cst::Expression::Name(name) = &base
            && let Some(cst::MemberAccess::Call(call)) = accesses.first()
            && call.dot().is_none()
        {
            let mut result = self.lower_function_call(name, call)?;

            for access in &accesses[1..] {
                result = self.lower_member_access(result, access)?;
            }

            return Some(result);
        }

        let mut result = self.lower_expression(&base)?;

        for access in &accesses {
            result = self.lower_member_access(result, access)?;
        }

        Some(result)
    }

    /// Lowers a single member access (.field, .`method()`, [index]).
    fn lower_member_access(
        &mut self,
        base: ast::Expression<'src>,
        access: &cst::MemberAccess<'src>,
    ) -> Option<ast::Expression<'src>> {
        match access {
            cst::MemberAccess::Field(field) => {
                let name_node = field.name()?;
                let name_text = self.ctx.text(name_node);

                Some(ast::Expression::get_attribute(
                    base,
                    Cow::Borrowed(name_text),
                ))
            }
            cst::MemberAccess::Call(call) => self.lower_method_call(base, call),
            cst::MemberAccess::Index(index) => {
                self.ctx.diagnostic(
                    Diagnostic::error("indexing is not supported")
                        .with_label(index.range(), "not supported")
                        .with_help("use `has` and `.` instead"),
                );

                None
            }
        }
    }

    /// Checks that a method call has the expected number of arguments.
    fn check_argument_count(
        &mut self,
        call: &cst::Call<'_>,
        function: &str,
        arguments: &[cst::Expression<'_>],
        expected: usize,
    ) -> bool {
        let found = arguments.len();
        if found == expected {
            return true;
        }

        let span = call
            .arguments()
            .map_or_else(|| call.range(), |args| args.range());

        let expected_suffix = if expected == 1 { "" } else { "s" };
        let found_suffix = if found == 1 { "" } else { "s" };

        self.ctx.diagnostic(
            Diagnostic::error(format!(
                "`{function}` expects {expected} argument{expected_suffix}, found {found}"
            ))
            .with_label(
                span,
                format!("expected {expected} argument{expected_suffix}, found {found} argument{found_suffix}"),
            ),
        );

        false
    }

    /// Lowers a method call.
    fn lower_method_call(
        &mut self,
        receiver: ast::Expression<'src>,
        call: &cst::Call<'src>,
    ) -> Option<ast::Expression<'src>> {
        let name_node = call.name()?;
        let method_name = self.ctx.text(name_node);

        let arguments: Vec<_> = call.arguments().map_or_else(Vec::new, |argument_list| {
            argument_list.expressions().collect()
        });

        match method_name {
            "contains" => {
                if !self.check_argument_count(call, "contains", &arguments, 1) {
                    return None;
                }

                let argument = self.lower_expression(&arguments[0])?;
                Some(ast::Expression::binary(
                    ast::BinaryOperator::Contains,
                    receiver,
                    argument,
                ))
            }
            "containsAll" => {
                if !self.check_argument_count(call, "containsAll", &arguments, 1) {
                    return None;
                }

                let argument = self.lower_expression(&arguments[0])?;
                Some(ast::Expression::binary(
                    ast::BinaryOperator::ContainsAll,
                    receiver,
                    argument,
                ))
            }
            "containsAny" => {
                if !self.check_argument_count(call, "containsAny", &arguments, 1) {
                    return None;
                }

                let argument = self.lower_expression(&arguments[0])?;
                Some(ast::Expression::binary(
                    ast::BinaryOperator::ContainsAny,
                    receiver,
                    argument,
                ))
            }
            "isEmpty" => {
                if !self.check_argument_count(call, "isEmpty", &arguments, 0) {
                    return None;
                }

                Some(ast::Expression::unary(
                    ast::UnaryOperator::IsEmpty,
                    receiver,
                ))
            }
            "getTag" => {
                if !self.check_argument_count(call, "getTag", &arguments, 1) {
                    return None;
                }

                let argument = self.lower_expression(&arguments[0])?;
                Some(ast::Expression::binary(
                    ast::BinaryOperator::GetTag,
                    receiver,
                    argument,
                ))
            }
            "hasTag" => {
                if !self.check_argument_count(call, "hasTag", &arguments, 1) {
                    return None;
                }

                let argument = self.lower_expression(&arguments[0])?;
                Some(ast::Expression::binary(
                    ast::BinaryOperator::HasTag,
                    receiver,
                    argument,
                ))
            }
            _ => {
                if EXTENSION_FUNCTIONS.contains(&method_name) {
                    let mut all_arguments = vec![receiver];
                    for argument in &arguments {
                        all_arguments.push(self.lower_expression(argument)?);
                    }

                    let identifier = self.ctx.make_identifier(method_name)?;
                    let function_name = ast::Name::unqualified(identifier);
                    Some(ast::Expression::extension_call(
                        function_name,
                        all_arguments,
                    ))
                } else {
                    let mut diagnostic =
                        Diagnostic::error(format!("unknown method `{method_name}`"))
                            .with_label(name_node.range(), "unknown method");

                    if let Some(suggestion) =
                        duramen_diagnostic::suggest(method_name, ALL_METHOD_CANDIDATES)
                    {
                        diagnostic = diagnostic
                            .with_help(format!("a similar method exists: `{suggestion}`"));
                    }

                    self.ctx.diagnostic(diagnostic);
                    None
                }
            }
        }
    }

    /// Lowers a direct function call (no dot, no receiver).
    fn lower_function_call(
        &mut self,
        name: &cst::Name<'src>,
        call: &cst::Call<'src>,
    ) -> Option<ast::Expression<'src>> {
        let text = name.basename(self.ctx.source)?;

        if name.is_qualified() || !EXTENSION_FUNCTIONS.contains(&text) {
            let full_text = &self.ctx.source[name.range()];

            let mut diagnostic =
                Diagnostic::error(format!("`{full_text}` is not a known function"))
                    .with_label(name.range(), "unknown function");

            if let Some(suggestion) = duramen_diagnostic::suggest(text, EXTENSION_FUNCTIONS) {
                diagnostic =
                    diagnostic.with_help(format!("a similar function exists: `{suggestion}`"));
            }

            self.ctx.diagnostic(diagnostic);
            return None;
        }

        let mut arguments = Vec::new();
        if let Some(argument_list) = call.arguments() {
            for argument in argument_list.expressions() {
                arguments.push(self.lower_expression(&argument)?);
            }
        }

        let identifier = self.ctx.make_identifier(text)?;
        let function_name = ast::Name::unqualified(identifier);
        Some(ast::Expression::extension_call(function_name, arguments))
    }

    /// Lowers a literal expression.
    fn lower_literal(&mut self, literal: &cst::Literal<'src>) -> Option<ast::Expression<'src>> {
        let kind = literal.kind()?;
        let token = literal.token()?;

        match kind {
            cst::LiteralKind::Bool => {
                let text = self.ctx.text(token);
                let value = text == "true";
                Some(ast::Expression::bool(value))
            }
            cst::LiteralKind::Integer => {
                let text = self.ctx.text(token);
                if let Ok(literal) = ast::IntegerLiteral::new(text) {
                    Some(ast::Expression::integer(literal))
                } else {
                    self.ctx.diagnostic(
                        Diagnostic::error(format!("integer literal `{text}` is out of range"))
                            .with_label(token.range(), "out of range for a 64-bit integer"),
                    );
                    None
                }
            }
            cst::LiteralKind::String => {
                let value = self.ctx.lower_string(token)?;
                Some(ast::Expression::string(value))
            }
        }
    }

    /// Lowers a slot expression.
    fn lower_slot(&mut self, slot: &cst::Slot<'_>) -> Option<ast::Expression<'src>> {
        let node = slot.name()?;
        let text = self.ctx.text(node);

        match ast::SlotKind::new(text) {
            Ok(kind) => Some(ast::Expression::slot(kind)),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers a list expression.
    fn lower_list(&mut self, list: &cst::List<'src>) -> ast::Expression<'src> {
        let mut elements = Vec::new();

        if let Some(arguments) = list.arguments() {
            for expression in arguments.expressions() {
                if let Some(lowered) = self.lower_expression(&expression) {
                    elements.push(lowered);
                }
            }
        }

        ast::Expression::set(elements)
    }

    /// Lowers a record expression.
    fn lower_record(&mut self, record: &cst::Record<'src>) -> Option<ast::Expression<'src>> {
        let mut entries = Vec::new();

        for entry in record.entries() {
            let Some(key_node) = entry.key() else {
                continue;
            };

            let key_text = self.ctx.text(key_node);
            let key = if key_node.kind() == Syntax::String {
                match self.ctx.lower_string(key_node) {
                    Some(unescaped) => unescaped,
                    None => continue,
                }
            } else {
                Cow::Borrowed(key_text)
            };

            let Some(value) = entry.value() else {
                continue;
            };

            let Some(value) = self.lower_expression(&value) else {
                continue;
            };

            entries.push((key, value));
        }

        let record_expression = match ast::RecordExpression::new(entries) {
            Ok(record_expression) => record_expression,
            Err(error) => {
                self.ctx.diagnostic(error);
                return None;
            }
        };

        Some(ast::Expression::record(record_expression))
    }

    /// Lowers a bare name in expression context.
    fn lower_name_expression(&mut self, name: &cst::Name<'_>) -> Option<ast::Expression<'src>> {
        if !name.is_qualified() {
            let text = name.basename(self.ctx.source)?;

            match text {
                "principal" => return Some(ast::Expression::variable(ast::Variable::Principal)),
                "action" => return Some(ast::Expression::variable(ast::Variable::Action)),
                "resource" => return Some(ast::Expression::variable(ast::Variable::Resource)),
                "context" => return Some(ast::Expression::variable(ast::Variable::Context)),
                _ => {}
            }
        }

        let first_start = name
            .segments()
            .next()
            .map_or_else(|| name.range().start, |segment| segment.range().start);
        let last_end = name
            .segments()
            .last()
            .map_or_else(|| name.range().end, |segment| segment.range().end);
        let span = first_start..last_end;
        let text = &self.ctx.source[span.clone()];

        let mut diagnostic = Diagnostic::error(format!("unknown variable `{text}`"))
            .with_label(span, "not a valid variable");

        if let Some(suggestion) = duramen_diagnostic::suggest(text, KNOWN_VARIABLES) {
            diagnostic = diagnostic.with_help(format!("a similar variable exists: `{suggestion}`"));
        } else {
            diagnostic = diagnostic.with_note(
                "`principal`, `action`, `resource`, and `context` are the only variables",
            );
        }

        self.ctx.diagnostic(diagnostic);
        None
    }
}

/// All method candidates for suggestions (built-in methods + extension functions).
const ALL_METHOD_CANDIDATES: &[&str] = &[
    "contains",
    "containsAll",
    "containsAny",
    "isEmpty",
    "getTag",
    "hasTag",
    "ip",
    "decimal",
    "datetime",
    "duration",
    "date",
    "time",
    "offset",
    "toDate",
    "toTime",
    "toDuration",
    "toMilliseconds",
    "toSeconds",
    "toMinutes",
    "toHours",
    "toDays",
    "isIpv4",
    "isIpv6",
    "isLoopback",
    "isMulticast",
    "isInRange",
    "lessThan",
    "lessThanOrEqual",
    "greaterThan",
    "greaterThanOrEqual",
];

/// Known variable names.
const KNOWN_VARIABLES: &[&str] = &["principal", "action", "resource", "context"];
