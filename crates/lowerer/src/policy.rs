use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use duramen_ast as ast;
use duramen_cst::{self as cst, CstNode as _};
use duramen_diagnostic::Diagnostics;
use duramen_escape::Escaper;
use duramen_syntax::{Syntax, Token, Tree};

use crate::common::LowerContext;
use crate::error::LowerError;

/// Known extension function names.
pub const EXTENSION_FUNCTIONS: &[&str] = &[
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

/// Policy lowerer for CST-to-AST transformation.
pub struct PolicyLowerer {
    ctx: LowerContext,
}

impl PolicyLowerer {
    /// Lowers a parsed tree and its diagnostics to an AST.
    #[must_use]
    pub fn lower<'src>(
        tree: &'src Tree<'_>,
        diagnostics: Diagnostics,
    ) -> (ast::Policies<'src>, Diagnostics) {
        let mut this = Self {
            ctx: LowerContext::new(diagnostics),
        };

        let mut result = Vec::new();

        if let Some(root) = tree.root()
            && let Some(policies) = cst::Policies::cast(root)
        {
            for policy in policies.policies() {
                if let Some(lowered) = this.lower_policy(&policy) {
                    result.push(lowered);
                }
            }
        }

        (ast::Policies::new(result), this.ctx.diagnostics)
    }

    /// Lowers a single policy.
    fn lower_policy<'src>(&mut self, policy: &cst::Policy<'src>) -> Option<ast::Policy<'src>> {
        let annotations = self.ctx.lower_annotations(policy.annotations())?;

        let effect = policy.effect().map(|effect| match effect {
            cst::Effect::Permit => ast::Effect::Permit,
            cst::Effect::Forbid => ast::Effect::Forbid,
        });

        let Some(effect) = effect else {
            self.ctx.diagnostics.push(LowerError::MissingEffect {
                span: policy.range(),
            });

            return None;
        };

        let mut principal = None;
        let mut action = None;
        let mut resource = None;

        for variable_definition in policy.variable_definitions() {
            let Some(variable) = variable_definition.variable() else {
                continue;
            };

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
                    self.ctx.diagnostics.push(LowerError::ContextInScope {
                        span: variable_definition.range(),
                    });
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
    fn lower_scope_constraint<'src>(
        &mut self,
        variable_definition: &cst::VariableDefinition<'src>,
        variable_name: &str,
    ) -> Option<ast::ScopeConstraint<'src>> {
        let has_type_test = variable_definition.is_token().is_some();
        let has_membership = variable_definition.in_token().is_some();
        let has_operator = variable_definition.operator_token().is_some();

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
            Syntax::Token(Token::Equal) => {
                let entity_or_slot = self.lower_entity_or_slot_expression(variable_definition)?;
                Some(ast::ScopeConstraint::Equal(entity_or_slot))
            }
            Syntax::Token(Token::InKeyword) => {
                let entity_or_slot = self.lower_entity_or_slot_expression(variable_definition)?;
                Some(ast::ScopeConstraint::In(entity_or_slot))
            }
            _ => {
                self.ctx.diagnostics.push(LowerError::InvalidScopeOperator {
                    span: operator_token.range(),
                    variable: String::from(variable_name),
                });

                None
            }
        }
    }

    /// Extracts an entity reference or slot from a variable definition's expression.
    fn lower_entity_or_slot_expression<'src>(
        &mut self,
        variable_definition: &cst::VariableDefinition<'src>,
    ) -> Option<ast::EntityOrSlot<'src>> {
        if let Some(slot) = variable_definition.slot() {
            return self.lower_slot_to_entity_or_slot(&slot);
        }

        let expression = variable_definition.expression()?;
        self.lower_entity_or_slot(&expression)
    }

    /// Lowers a slot to `EntityOrSlot`.
    fn lower_slot_to_entity_or_slot<'src>(
        &mut self,
        slot: &cst::Slot<'src>,
    ) -> Option<ast::EntityOrSlot<'src>> {
        let node = slot.name()?;
        let text = node.text();

        match ast::SlotKind::new(text) {
            Ok(_) => Some(ast::EntityOrSlot::Slot),
            Err(error) => {
                self.ctx.diagnostics.push(error);
                None
            }
        }
    }

    /// Lowers an expression to `EntityOrSlot`.
    fn lower_entity_or_slot<'src>(
        &mut self,
        expression: &cst::Expression<'src>,
    ) -> Option<ast::EntityOrSlot<'src>> {
        match expression {
            cst::Expression::Slot(slot) => self.lower_slot_to_entity_or_slot(slot),
            cst::Expression::EntityReference(entity_reference) => {
                let reference = self.lower_entity_reference(entity_reference)?;
                Some(ast::EntityOrSlot::Entity(reference))
            }
            _ => {
                self.ctx.diagnostics.push(LowerError::UnexpectedExpression {
                    span: expression.range(),
                    expected: "expected an entity reference or slot",
                });

                None
            }
        }
    }

    /// Lowers an action scope constraint.
    fn lower_action_constraint<'src>(
        &mut self,
        variable_definition: &cst::VariableDefinition<'src>,
    ) -> Option<ast::ActionConstraint<'src>> {
        let Some(operator_token) = variable_definition.operator_token() else {
            return Some(ast::ActionConstraint::Any);
        };

        match operator_token.kind() {
            Syntax::Token(Token::Equal) => {
                let expression = variable_definition.expression()?;
                let entity_reference = self.lower_action_entity_reference(&expression)?;
                Some(ast::ActionConstraint::Equal(entity_reference))
            }
            Syntax::Token(Token::InKeyword) => self.lower_action_in_constraint(variable_definition),
            _ => {
                self.ctx.diagnostics.push(LowerError::InvalidScopeOperator {
                    span: operator_token.range(),
                    variable: String::from("action"),
                });

                None
            }
        }
    }

    /// Lowers `action in ...` constraint.
    fn lower_action_in_constraint<'src>(
        &mut self,
        variable_definition: &cst::VariableDefinition<'src>,
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
                    self.ctx.diagnostics.push(error);
                    None
                }
            }
        } else {
            let entity_reference = self.lower_action_entity_reference(&expression)?;
            match ast::ActionList::new(vec![entity_reference]) {
                Ok(list) => Some(ast::ActionConstraint::In(list)),
                Err(error) => {
                    self.ctx.diagnostics.push(error);
                    None
                }
            }
        }
    }

    /// Lowers an expression to an action entity reference.
    fn lower_action_entity_reference<'src>(
        &mut self,
        expression: &cst::Expression<'src>,
    ) -> Option<ast::EntityReference<'src>> {
        if let cst::Expression::EntityReference(entity_reference) = expression {
            self.lower_entity_reference(entity_reference)
        } else {
            self.ctx.diagnostics.push(LowerError::UnexpectedExpression {
                span: expression.range(),
                expected: "expected an entity reference",
            });

            None
        }
    }

    /// Lowers an entity reference.
    fn lower_entity_reference<'src>(
        &mut self,
        entity_reference: &cst::EntityReference<'src>,
    ) -> Option<ast::EntityReference<'src>> {
        let kind = entity_reference.kind()?;
        let kind = self.ctx.lower_name(&kind)?;

        let id = entity_reference.id()?;
        let id = self.ctx.lower_string(id)?;

        Some(ast::EntityReference::new(kind, id))
    }

    /// Lowers a condition.
    fn lower_condition<'src>(
        &mut self,
        condition: &cst::Condition<'src>,
    ) -> Option<ast::Condition<'src>> {
        let kind = condition.kind().map(|kind| match kind {
            cst::ConditionKind::When => ast::ConditionKind::When,
            cst::ConditionKind::Unless => ast::ConditionKind::Unless,
        })?;

        let body = condition.body()?;
        let body = self.lower_expression(&body)?;
        Some(ast::Condition::new(kind, body))
    }

    /// Lowers an expression.
    fn lower_expression<'src>(
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
    fn lower_if<'src>(
        &mut self,
        expression: &cst::IfExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let test = expression.test()?;
        let consequent = expression.consequent()?;
        let alternate = expression.alternate()?;

        let test = self.lower_expression(&test)?;
        let consequent = self.lower_expression(&consequent)?;
        let alternate = self.lower_expression(&alternate)?;

        Some(ast::Expression::if_then_else(test, consequent, alternate))
    }

    /// Lowers an or expression.
    fn lower_or<'src>(
        &mut self,
        expression: &cst::OrExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let left = expression.left()?;
        let left = self.lower_expression(&left)?;

        let right = expression.right()?;
        let right = self.lower_expression(&right)?;

        Some(ast::Expression::or(left, right))
    }

    /// Lowers an and expression.
    fn lower_and<'src>(
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
    fn lower_relation<'src>(
        &mut self,
        expression: &cst::RelationExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let Some(operator) = expression.operator() else {
            if let Some(token) = expression.operator_token()
                && token.kind() == Token::Assign
            {
                self.ctx.diagnostics.push(LowerError::InvalidEquals {
                    span: token.range(),
                });
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
    fn lower_sum<'src>(
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
    fn lower_product<'src>(
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
        }
    }

    /// Lowers a has expression.
    fn lower_has<'src>(
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
    fn extract_attribute_name<'src>(
        &mut self,
        expression: &cst::Expression<'src>,
    ) -> Option<Cow<'src, str>> {
        match expression {
            cst::Expression::Literal(literal) => {
                let token = literal.token()?;
                if token.kind() == Token::String {
                    self.ctx.lower_string(token)
                } else {
                    Some(Cow::Borrowed(token.text()))
                }
            }
            cst::Expression::Name(name) => {
                let text = name.basename()?;
                Some(Cow::Borrowed(text))
            }
            _ => {
                self.ctx.diagnostics.push(LowerError::UnexpectedExpression {
                    span: expression.range(),
                    expected: "expected an attribute name",
                });

                None
            }
        }
    }

    /// Lowers a like expression.
    fn lower_like<'src>(
        &mut self,
        expression: &cst::LikeExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let left = expression.expression()?;
        let left = self.lower_expression(&left)?;

        let pattern = expression.pattern()?;
        let pattern = pattern.syntax();

        let raw = pattern.text();
        let offset = pattern.range().start;

        match Escaper::new(raw).unescape_pattern() {
            Ok(elements) => {
                let pattern = ast::Pattern::new(elements);
                Some(ast::Expression::like(left, pattern))
            }
            Err(errors) => {
                for error in errors {
                    self.ctx.diagnostics.push(error.offset(offset));
                }

                None
            }
        }
    }

    /// Lowers an is expression.
    fn lower_is<'src>(
        &mut self,
        expression: &cst::IsExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
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
    fn lower_unary<'src>(
        &mut self,
        expression: &cst::UnaryExpression<'src>,
    ) -> Option<ast::Expression<'src>> {
        let operator = expression.operator()?;

        let operator_count = expression.operator_tokens().count();
        if operator_count > 4 {
            self.ctx.diagnostics.push(LowerError::UnaryOpLimit {
                span: expression.range(),
                count: operator_count,
            });

            return None;
        }

        let operand = expression.operand()?;
        if operator == cst::UnaryOperator::Negate
            && operator_count == 1
            && let cst::Expression::Literal(literal) = &operand
            && literal.kind() == Some(cst::LiteralKind::Integer)
            && let Some(token) = literal.token()
        {
            let text = token.text();

            let mut negated = String::from("-");
            negated.push_str(text);

            return match ast::IntegerLiteral::new(&negated) {
                Ok(literal) => Some(ast::Expression::integer(literal)),
                Err(error) => {
                    self.ctx.diagnostics.push(error);
                    None
                }
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
    fn lower_member<'src>(
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

    /// Lowers a single member access.
    fn lower_member_access<'src>(
        &mut self,
        base: ast::Expression<'src>,
        access: &cst::MemberAccess<'src>,
    ) -> Option<ast::Expression<'src>> {
        match access {
            cst::MemberAccess::Field(field) => {
                let name = field.name()?;
                let name = name.text();
                Some(ast::Expression::get_attribute(base, Cow::Borrowed(name)))
            }
            cst::MemberAccess::Call(call) => self.lower_method_call(base, call),
            cst::MemberAccess::Index(index) => {
                self.ctx.diagnostics.push(LowerError::UnsupportedIndex {
                    span: index.range(),
                });

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
        if arguments.len() == expected {
            return true;
        }

        self.ctx.diagnostics.push(LowerError::WrongArgumentCount {
            span: call.range(),
            function: String::from(function),
            expected,
            found: arguments.len(),
        });

        false
    }

    /// Lowers a method call.
    fn lower_method_call<'src>(
        &mut self,
        receiver: ast::Expression<'src>,
        call: &cst::Call<'src>,
    ) -> Option<ast::Expression<'src>> {
        let name_node = call.name()?;
        let method_name = name_node.text();

        let arguments: Vec<_> = if let Some(argument_list) = call.arguments() {
            argument_list.expressions().collect()
        } else {
            Vec::new()
        };

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
                // TODO, inline?
                if EXTENSION_FUNCTIONS.contains(&method_name) {
                    let mut all_arguments = vec![receiver];
                    for argument in &arguments {
                        all_arguments.push(self.lower_expression(argument)?);
                    }

                    let identifier = match ast::Identifier::new(method_name) {
                        Ok(identifier) => identifier,
                        Err(error) => {
                            self.ctx.diagnostics.push(error);
                            return None;
                        }
                    };
                    let function_name = ast::Name::unqualified(identifier);
                    Some(ast::Expression::extension_call(
                        function_name,
                        all_arguments,
                    ))
                } else {
                    self.ctx.diagnostics.push(LowerError::UnknownMethod {
                        span: name_node.range(),
                        name: String::from(method_name),
                    });

                    None
                }
            }
        }
    }

    /// Lowers a direct function call (no dot, no receiver).
    fn lower_function_call<'src>(
        &mut self,
        name: &cst::Name<'src>,
        call: &cst::Call<'src>,
    ) -> Option<ast::Expression<'src>> {
        let text = name.basename()?;

        if name.is_qualified() || !EXTENSION_FUNCTIONS.contains(&text) {
            self.ctx.diagnostics.push(LowerError::UnknownFunction {
                span: name.range(),
                name: String::from(name.text()),
            });

            return None;
        }

        let mut arguments = Vec::new();
        if let Some(argument_list) = call.arguments() {
            for argument in argument_list.expressions() {
                arguments.push(self.lower_expression(&argument)?);
            }
        }

        let identifier = match ast::Identifier::new(text) {
            Ok(identifier) => identifier,
            Err(error) => {
                self.ctx.diagnostics.push(error);
                return None;
            }
        };

        let function_name = ast::Name::unqualified(identifier);
        Some(ast::Expression::extension_call(function_name, arguments))
    }

    /// Lowers a literal expression.
    fn lower_literal<'src>(
        &mut self,
        literal: &cst::Literal<'src>,
    ) -> Option<ast::Expression<'src>> {
        let kind = literal.kind()?;
        let token = literal.token()?;

        match kind {
            cst::LiteralKind::Bool => {
                let text = token.text();
                let value = text == "true";
                Some(ast::Expression::bool(value))
            }
            cst::LiteralKind::Integer => {
                let text = token.text();
                match ast::IntegerLiteral::new(text) {
                    Ok(literal) => Some(ast::Expression::integer(literal)),
                    Err(error) => {
                        self.ctx.diagnostics.push(error);
                        None
                    }
                }
            }
            cst::LiteralKind::String => {
                let value = self.ctx.lower_string(token)?;
                Some(ast::Expression::string(value))
            }
        }
    }

    /// Lowers a slot expression.
    fn lower_slot<'src>(&mut self, slot: &cst::Slot<'src>) -> Option<ast::Expression<'src>> {
        let node = slot.name()?;
        let text = node.text();

        match ast::SlotKind::new(text) {
            Ok(kind) => Some(ast::Expression::slot(kind)),
            Err(error) => {
                self.ctx.diagnostics.push(error);
                None
            }
        }
    }

    /// Lowers a list expression.
    fn lower_list<'src>(&mut self, list: &cst::List<'src>) -> ast::Expression<'src> {
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
    fn lower_record<'src>(&mut self, record: &cst::Record<'src>) -> Option<ast::Expression<'src>> {
        let mut entries = Vec::new();

        for entry in record.entries() {
            let Some(key_node) = entry.key() else {
                continue;
            };

            let key = if key_node.kind() == Token::String {
                match self.ctx.lower_string(key_node) {
                    Some(unescaped) => unescaped,
                    None => continue,
                }
            } else {
                Cow::Borrowed(key_node.text())
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
                self.ctx.diagnostics.push(error);
                return None;
            }
        };

        Some(ast::Expression::record(record_expression))
    }

    /// Lowers a bare name in expression context.
    fn lower_name_expression<'src>(
        &mut self,
        name: &cst::Name<'src>,
    ) -> Option<ast::Expression<'src>> {
        if !name.is_qualified() {
            let text = name.basename()?;

            match text {
                "principal" => return Some(ast::Expression::variable(ast::Variable::Principal)),
                "action" => return Some(ast::Expression::variable(ast::Variable::Action)),
                "resource" => return Some(ast::Expression::variable(ast::Variable::Resource)),
                "context" => return Some(ast::Expression::variable(ast::Variable::Context)),
                _ => {}
            }
        }

        self.ctx.diagnostics.push(LowerError::UnknownVariable {
            span: name.range(),
            name: String::from(name.text()),
        });

        None
    }
}
