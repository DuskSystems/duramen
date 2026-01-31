use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use duramen_cst::CstNode as _;
use duramen_diagnostics::{Diagnostic, Diagnostics};
use {duramen_ast as ast, duramen_cst as cst};

use crate::unescape::{PatternUnescaper, StringUnescaper};

/// Lowers a policy CST into an AST.
pub struct PolicyLowerer<'a> {
    source: &'a str,
    diagnostics: Diagnostics,
}

impl<'a> PolicyLowerer<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            diagnostics: Diagnostics::new(),
        }
    }

    /// # Errors
    pub fn lower(
        mut self,
        tree: &cst::policy::PolicyTree,
    ) -> Result<(Vec<ast::policy::Template>, Diagnostics), Diagnostics> {
        let value = self.lower_policies(tree);

        if self.diagnostics.has_error() {
            Err(self.diagnostics)
        } else {
            Ok((value, self.diagnostics))
        }
    }

    fn lower_policies(&mut self, tree: &cst::policy::PolicyTree) -> Vec<ast::policy::Template> {
        let mut templates = Vec::new();

        let Some(policies) = tree.children().find_map(cst::policy::Policies::cast) else {
            return templates;
        };

        for (index, policy) in policies.iter().enumerate() {
            if let Some(template) = self.lower_policy(&policy, index) {
                templates.push(template);
            }
        }

        templates
    }

    fn lower_policy(
        &mut self,
        policy: &cst::policy::Policy<'_>,
        index: usize,
    ) -> Option<ast::policy::Template> {
        let span = policy.range();

        let effect = policy
            .effect()
            .map(|eff| match eff {
                cst::policy::Effect::Permit => ast::policy::Effect::Permit,
                cst::policy::Effect::Forbid => ast::policy::Effect::Forbid,
            })
            .or_else(|| {
                self.diagnostics
                    .push(Diagnostic::missing_effect(span.clone()));
                None
            })?;

        let (principal, action, resource) = self.lower_scope(policy, span);
        let condition = self.lower_conditions(policy);
        let (annotations, id) = self.lower_annotations(policy, index);

        Some(ast::policy::Template::new(
            id,
            annotations,
            effect,
            principal,
            action,
            resource,
            condition,
        ))
    }

    fn lower_scope(
        &mut self,
        policy: &cst::policy::Policy<'_>,
        span: Range<u32>,
    ) -> (
        ast::policy::PrincipalConstraint,
        ast::policy::ActionConstraint,
        ast::policy::ResourceConstraint,
    ) {
        let mut principal = None;
        let mut action = None;
        let mut resource = None;

        for var_def in policy.variables() {
            let Some(variable) = var_def.variable() else {
                continue;
            };

            match variable {
                cst::policy::Variable::Principal => {
                    principal = Some(self.lower_principal(&var_def));
                }
                cst::policy::Variable::Action => {
                    action = Some(self.lower_action_constraint(&var_def));
                }
                cst::policy::Variable::Resource => {
                    resource = Some(self.lower_resource(&var_def));
                }
                cst::policy::Variable::Context => {}
            }
        }

        let principal = principal.unwrap_or_else(|| {
            self.diagnostics.push(Diagnostic::missing_scope_variable(
                "principal",
                span.clone(),
            ));
            ast::policy::PrincipalConstraint::any()
        });

        let action = action.unwrap_or_else(|| {
            self.diagnostics
                .push(Diagnostic::missing_scope_variable("action", span.clone()));
            ast::policy::ActionConstraint::any()
        });

        let resource = resource.unwrap_or_else(|| {
            self.diagnostics
                .push(Diagnostic::missing_scope_variable("resource", span));
            ast::policy::ResourceConstraint::any()
        });

        (principal, action, resource)
    }

    fn lower_conditions(&mut self, policy: &cst::policy::Policy<'_>) -> Vec<ast::policy::Clause> {
        let mut clauses: Vec<ast::policy::Clause> = Vec::new();

        for condition in policy.conditions() {
            let Some(kind) = condition.kind() else {
                continue;
            };

            let Some(cst_expr) = condition.expr() else {
                self.diagnostics
                    .push(Diagnostic::empty_node("expression", condition.range()));
                continue;
            };

            let Some(expr) = self.lower_expr(&cst_expr) else {
                continue;
            };

            let clause_kind = match kind {
                cst::policy::ConditionKind::When => ast::policy::ClauseKind::When,
                cst::policy::ConditionKind::Unless => ast::policy::ClauseKind::Unless,
            };

            clauses.push(ast::policy::Clause::new(clause_kind, expr));
        }

        clauses
    }

    fn lower_annotations(
        &mut self,
        policy: &cst::policy::Policy<'_>,
        index: usize,
    ) -> (ast::common::Annotations, ast::policy::PolicyId) {
        let mut map: BTreeMap<ast::common::AnyId, ast::common::Annotation> = BTreeMap::new();
        let mut seen: BTreeMap<String, Range<u32>> = BTreeMap::new();
        let mut policy_id = None;

        for annotation in policy.annotations() {
            let Some(name) = annotation.name(self.source) else {
                continue;
            };

            let span = annotation.range();

            if let Some(first) = seen.get(name) {
                self.diagnostics.push(Diagnostic::duplicate_annotation(
                    name,
                    span.clone(),
                    first.clone(),
                ));
                continue;
            }

            seen.insert(name.into(), span.clone());

            let value: Option<String> = annotation.value(self.source).and_then(|val| {
                StringUnescaper::new(val).unescape().or_else(|| {
                    self.diagnostics.push(Diagnostic::invalid_string_escape(
                        "invalid escape",
                        span.clone(),
                    ));
                    None
                })
            });

            if name == "id"
                && let Some(id_val) = &value
            {
                policy_id = Some(ast::policy::PolicyId::new(id_val.clone()));
            }

            let annotation_value = value.map_or_else(
                ast::common::Annotation::without_value,
                ast::common::Annotation::with_value,
            );
            map.insert(ast::common::AnyId::new(name.into()), annotation_value);
        }

        let id = policy_id.unwrap_or_else(|| ast::policy::PolicyId::new(format!("policy{index}")));
        (ast::common::Annotations::from_map(map), id)
    }

    fn lower_principal(
        &mut self,
        var_def: &cst::policy::VariableDef<'_>,
    ) -> ast::policy::PrincipalConstraint {
        let constraint = self.lower_principal_or_resource(var_def, ast::policy::SlotId::Principal);
        ast::policy::PrincipalConstraint::new(constraint)
    }

    fn lower_resource(
        &mut self,
        var_def: &cst::policy::VariableDef<'_>,
    ) -> ast::policy::ResourceConstraint {
        let constraint = self.lower_principal_or_resource(var_def, ast::policy::SlotId::Resource);
        ast::policy::ResourceConstraint::new(constraint)
    }

    fn lower_principal_or_resource(
        &mut self,
        var_def: &cst::policy::VariableDef<'_>,
        slot_id: ast::policy::SlotId,
    ) -> ast::policy::PrincipalOrResourceConstraint {
        let span = var_def.range();

        let entity_type = var_def
            .entity_type()
            .and_then(|name| lower_entity_type(&name, self.source));

        let operator = var_def.operator();
        let constraint_expr = var_def.constraint();

        match (operator, constraint_expr, entity_type) {
            (None, None, None) => ast::policy::PrincipalOrResourceConstraint::Any,

            // `principal is User` - entity_type is set, no operator
            // Note: constraint_expr may be Some(Name) due to Name being an Expression,
            // so we handle both None and Some cases when entity_type is present
            (None, _, Some(entity_type)) => {
                ast::policy::PrincipalOrResourceConstraint::Is(entity_type)
            }

            (Some(cst::policy::RelOp::Eq), Some(expr), None) => {
                self.lower_entity_or_slot(&expr, slot_id).map_or(
                    ast::policy::PrincipalOrResourceConstraint::Any,
                    ast::policy::PrincipalOrResourceConstraint::Eq,
                )
            }

            (Some(cst::policy::RelOp::In), Some(expr), None) => {
                self.lower_entity_or_slot(&expr, slot_id).map_or(
                    ast::policy::PrincipalOrResourceConstraint::Any,
                    ast::policy::PrincipalOrResourceConstraint::In,
                )
            }

            (
                Some(cst::policy::RelOp::Eq | cst::policy::RelOp::In),
                Some(expr),
                Some(entity_type),
            ) => match self.lower_entity_or_slot(&expr, slot_id) {
                Some(entity_ref) => {
                    ast::policy::PrincipalOrResourceConstraint::IsIn(entity_type, entity_ref)
                }
                None => ast::policy::PrincipalOrResourceConstraint::Is(entity_type),
            },

            (Some(op), _, _) => {
                self.diagnostics.push(Diagnostic::invalid_scope_operator(
                    &format!("{op:?}"),
                    "`==` or `in`",
                    span,
                ));
                ast::policy::PrincipalOrResourceConstraint::Any
            }

            (None, Some(_), None) => {
                self.diagnostics.push(Diagnostic::missing_child(
                    "scope constraint",
                    "operator",
                    span,
                ));
                ast::policy::PrincipalOrResourceConstraint::Any
            }
        }
    }

    fn lower_action_constraint(
        &mut self,
        var_def: &cst::policy::VariableDef<'_>,
    ) -> ast::policy::ActionConstraint {
        let span = var_def.range();
        let operator = var_def.operator();
        let constraint_expr = var_def.constraint();

        match (operator, constraint_expr) {
            (None, None) => ast::policy::ActionConstraint::any(),

            (Some(cst::policy::RelOp::Eq), Some(expr)) => {
                self.lower_constraint_entity_ref(&expr).map_or_else(
                    ast::policy::ActionConstraint::any,
                    ast::policy::ActionConstraint::equal,
                )
            }

            (Some(cst::policy::RelOp::In), Some(expr)) => {
                let actions = self.lower_action_list(&expr);
                ast::policy::ActionConstraint::is_in(actions)
            }

            (Some(op), _) => {
                self.diagnostics.push(Diagnostic::invalid_scope_operator(
                    &format!("{op:?}"),
                    "`==` or `in`",
                    span,
                ));
                ast::policy::ActionConstraint::any()
            }

            (None, Some(_)) => {
                self.diagnostics.push(Diagnostic::missing_child(
                    "action constraint",
                    "operator",
                    span,
                ));
                ast::policy::ActionConstraint::any()
            }
        }
    }

    fn lower_entity_or_slot(
        &mut self,
        expr: &cst::policy::Expression<'_>,
        slot_id: ast::policy::SlotId,
    ) -> Option<ast::policy::EntityReference> {
        match expr {
            cst::policy::Expression::EntityRef(entity_ref) => {
                let uid = self.lower_entity_ref_node(entity_ref)?;
                Some(ast::policy::EntityReference::euid(uid))
            }
            cst::policy::Expression::Slot(slot) => {
                let kind = slot.kind()?;
                match kind {
                    cst::policy::SlotKind::Principal
                        if slot_id == ast::policy::SlotId::Principal =>
                    {
                        Some(ast::policy::EntityReference::Slot)
                    }
                    cst::policy::SlotKind::Resource if slot_id == ast::policy::SlotId::Resource => {
                        Some(ast::policy::EntityReference::Slot)
                    }
                    cst::policy::SlotKind::Principal
                    | cst::policy::SlotKind::Resource
                    | cst::policy::SlotKind::Other => {
                        if let Some(name) = slot.name(self.source) {
                            self.diagnostics
                                .push(Diagnostic::invalid_slot_id(name, slot.range()));
                        }
                        None
                    }
                }
            }
            _ => {
                self.diagnostics
                    .push(Diagnostic::expected_entity_or_slot(expr.syntax().range()));
                None
            }
        }
    }

    fn lower_constraint_entity_ref(
        &mut self,
        expr: &cst::policy::Expression<'_>,
    ) -> Option<ast::common::EntityUid> {
        if let cst::policy::Expression::EntityRef(entity_ref) = expr {
            self.lower_entity_ref_node(entity_ref)
        } else {
            self.diagnostics
                .push(Diagnostic::expected_entity(expr.syntax().range()));
            None
        }
    }

    fn lower_action_list(
        &mut self,
        expr: &cst::policy::Expression<'_>,
    ) -> Vec<ast::common::EntityUid> {
        let mut actions = Vec::new();

        match expr {
            cst::policy::Expression::List(list) => {
                for elem in list.elements() {
                    if let Some(uid) = self.lower_constraint_entity_ref(&elem) {
                        actions.push(uid);
                    }
                }
            }
            cst::policy::Expression::EntityRef(entity_ref) => {
                if let Some(uid) = self.lower_entity_ref_node(entity_ref) {
                    actions.push(uid);
                }
            }
            _ => {
                self.diagnostics
                    .push(Diagnostic::expected_entity(expr.syntax().range()));
            }
        }

        actions
    }

    fn lower_entity_ref_node(
        &mut self,
        entity_ref: &cst::policy::EntityRefExpression<'_>,
    ) -> Option<ast::common::EntityUid> {
        let type_name = entity_ref.type_name()?;
        let entity_type = lower_entity_type(&type_name, self.source)?;

        let id_str = entity_ref.id(self.source)?;
        let id = StringUnescaper::new(id_str).unescape().or_else(|| {
            self.diagnostics.push(Diagnostic::invalid_string_escape(
                "invalid escape",
                entity_ref.range(),
            ));
            None
        })?;

        Some(ast::common::EntityUid::new(
            entity_type,
            ast::common::Eid::new(id),
        ))
    }

    fn lower_expr(&mut self, expr: &cst::policy::Expression<'_>) -> Option<ast::policy::Expr> {
        match expr {
            cst::policy::Expression::If(if_expr) => self.lower_if(if_expr),
            cst::policy::Expression::Or(or_expr) => self.lower_or(or_expr),
            cst::policy::Expression::And(and_expr) => self.lower_and(and_expr),
            cst::policy::Expression::Relation(rel_expr) => self.lower_relation(rel_expr),
            cst::policy::Expression::Sum(sum_expr) => self.lower_sum(sum_expr),
            cst::policy::Expression::Product(prod_expr) => self.lower_product(prod_expr),
            cst::policy::Expression::Has(has_expr) => self.lower_has(has_expr),
            cst::policy::Expression::Like(like_expr) => self.lower_like(like_expr),
            cst::policy::Expression::Is(is_expr) => self.lower_is(is_expr),
            cst::policy::Expression::Unary(unary_expr) => self.lower_unary(unary_expr),
            cst::policy::Expression::Member(member_expr) => self.lower_member(member_expr),
            cst::policy::Expression::Literal(lit_expr) => self.lower_literal(lit_expr),
            cst::policy::Expression::EntityRef(entity_ref) => {
                self.lower_entity_ref_expr(entity_ref)
            }
            cst::policy::Expression::Slot(slot_expr) => self.lower_slot(slot_expr),
            cst::policy::Expression::Paren(paren_expr) => {
                let inner = paren_expr.inner()?;
                self.lower_expr(&inner)
            }
            cst::policy::Expression::List(list_expr) => Some(self.lower_list(list_expr)),
            cst::policy::Expression::Record(record_expr) => self.lower_record(record_expr),
            cst::policy::Expression::Name(name) => self.lower_name_expr(name),
        }
    }

    fn lower_name_expr(&mut self, name: &cst::policy::Name<'_>) -> Option<ast::policy::Expr> {
        let basename = name.basename(self.source)?;
        let var = match basename {
            "principal" => ast::policy::Var::Principal,
            "action" => ast::policy::Var::Action,
            "resource" => ast::policy::Var::Resource,
            "context" => ast::policy::Var::Context,
            _ => {
                self.diagnostics
                    .push(Diagnostic::invalid_identifier(basename, name.range()));

                return None;
            }
        };

        if name.is_qualified() {
            self.diagnostics
                .push(Diagnostic::invalid_identifier(basename, name.range()));

            return None;
        }

        Some(ast::policy::Expr::var(var))
    }

    fn lower_if(&mut self, if_expr: &cst::policy::IfExpression<'_>) -> Option<ast::policy::Expr> {
        let cond = if_expr.condition()?;
        let then_expr = if_expr.then_expr()?;
        let else_expr = if_expr.else_expr()?;

        let cond = self.lower_expr(&cond)?;
        let then_expr = self.lower_expr(&then_expr)?;
        let else_expr = self.lower_expr(&else_expr)?;

        Some(ast::policy::Expr::if_then_else(cond, then_expr, else_expr))
    }

    fn lower_or(&mut self, or_expr: &cst::policy::OrExpression<'_>) -> Option<ast::policy::Expr> {
        let mut operands = or_expr.operands();

        let first = operands.next()?;
        let mut result = self.lower_expr(&first)?;

        for operand in operands {
            let right = self.lower_expr(&operand)?;
            result = ast::policy::Expr::or(result, right);
        }

        Some(result)
    }

    fn lower_and(
        &mut self,
        and_expr: &cst::policy::AndExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let mut operands = and_expr.operands();

        let first = operands.next()?;
        let mut result = self.lower_expr(&first)?;

        for operand in operands {
            let right = self.lower_expr(&operand)?;
            result = ast::policy::Expr::and(result, right);
        }

        Some(result)
    }

    fn lower_relation(
        &mut self,
        rel_expr: &cst::policy::RelationExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let left = rel_expr.left()?;
        let right = rel_expr.right()?;
        let operator = rel_expr.operator()?;

        let left = self.lower_expr(&left)?;
        let right = self.lower_expr(&right)?;

        let expr = match operator {
            cst::policy::RelOp::Eq => {
                ast::policy::Expr::binary(ast::policy::BinaryOp::Eq, left, right)
            }
            cst::policy::RelOp::NotEq => ast::policy::Expr::unary(
                ast::policy::UnaryOp::Not,
                ast::policy::Expr::binary(ast::policy::BinaryOp::Eq, left, right),
            ),
            cst::policy::RelOp::Less => {
                ast::policy::Expr::binary(ast::policy::BinaryOp::Less, left, right)
            }
            cst::policy::RelOp::LessEq => {
                ast::policy::Expr::binary(ast::policy::BinaryOp::LessEq, left, right)
            }
            cst::policy::RelOp::Greater => {
                ast::policy::Expr::binary(ast::policy::BinaryOp::Greater, left, right)
            }
            cst::policy::RelOp::GreaterEq => {
                ast::policy::Expr::binary(ast::policy::BinaryOp::GreaterEq, left, right)
            }
            cst::policy::RelOp::In => {
                ast::policy::Expr::binary(ast::policy::BinaryOp::In, left, right)
            }
        };

        Some(expr)
    }

    fn lower_sum(
        &mut self,
        sum_expr: &cst::policy::SumExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let mut operands = sum_expr.operands();
        let mut operators = sum_expr.operators();

        let first = operands.next()?;
        let mut result = self.lower_expr(&first)?;

        for operand in operands {
            let operator = operators.next()?;
            let right = self.lower_expr(&operand)?;

            result = match operator {
                cst::policy::AddOp::Plus => {
                    ast::policy::Expr::binary(ast::policy::BinaryOp::Add, result, right)
                }
                cst::policy::AddOp::Minus => {
                    ast::policy::Expr::binary(ast::policy::BinaryOp::Sub, result, right)
                }
            };
        }

        Some(result)
    }

    fn lower_product(
        &mut self,
        prod_expr: &cst::policy::ProductExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let mut operands = prod_expr.operands();
        let mut operators = prod_expr.operators();

        let first = operands.next()?;
        let mut result = self.lower_expr(&first)?;

        for operand in operands {
            let operator = operators.next()?;
            let right = self.lower_expr(&operand)?;

            result = match operator {
                cst::policy::MulOp::Times => {
                    ast::policy::Expr::binary(ast::policy::BinaryOp::Mul, result, right)
                }
                cst::policy::MulOp::Divide => {
                    self.diagnostics
                        .push(Diagnostic::unknown_method("/", prod_expr.range()));
                    return None;
                }
                cst::policy::MulOp::Mod => {
                    self.diagnostics
                        .push(Diagnostic::unknown_method("%", prod_expr.range()));
                    return None;
                }
            };
        }

        Some(result)
    }

    fn lower_has(
        &mut self,
        has_expr: &cst::policy::HasExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let target = has_expr.target()?;
        let span = has_expr.range();

        if has_expr.is_field_reserved() {
            let (field_text, _) = has_expr.field(self.source)?;
            self.diagnostics
                .push(Diagnostic::reserved_identifier(field_text, span));

            return None;
        }

        let (field_text, is_quoted) = has_expr.field(self.source)?;

        let target = self.lower_expr(&target)?;
        let field = self.unescape_field(field_text, is_quoted, span)?;

        Some(ast::policy::Expr::has_attr(target, field))
    }

    fn lower_like(
        &mut self,
        like_expr: &cst::policy::LikeExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let target = like_expr.target()?;
        let pattern_str = like_expr.pattern(self.source)?;

        let target = self.lower_expr(&target)?;

        let pattern = PatternUnescaper::new(pattern_str).unescape().or_else(|| {
            self.diagnostics.push(Diagnostic::invalid_pattern(
                "invalid escape",
                like_expr.range(),
            ));
            None
        })?;

        Some(ast::policy::Expr::like(target, pattern))
    }

    fn lower_is(&mut self, is_expr: &cst::policy::IsExpression<'_>) -> Option<ast::policy::Expr> {
        let target_cst = is_expr.target()?;
        let entity_type_name = is_expr.entity_type()?;

        let target = self.lower_expr(&target_cst)?;
        let entity_type = lower_entity_type(&entity_type_name, self.source)?;

        if let Some(in_expr) = is_expr.in_expr() {
            let container = self.lower_expr(&in_expr)?;
            Some(ast::policy::Expr::is_in(target, entity_type, container))
        } else {
            Some(ast::policy::Expr::is(target, entity_type))
        }
    }

    fn lower_unary(
        &mut self,
        unary_expr: &cst::policy::UnaryExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        const MAX_OPERATORS: usize = 4;

        let operators: Vec<_> = unary_expr.operators().collect();

        let mut not_count: usize = 0;
        let mut neg_count: usize = 0;
        for op in &operators {
            match op {
                cst::policy::UnaryOp::Not => not_count = not_count.saturating_add(1),
                cst::policy::UnaryOp::Neg => neg_count = neg_count.saturating_add(1),
            }
        }

        let span = unary_expr.range();
        if not_count > MAX_OPERATORS {
            self.diagnostics
                .push(Diagnostic::too_many_operators("!", span));

            return None;
        }

        if neg_count > MAX_OPERATORS {
            self.diagnostics
                .push(Diagnostic::too_many_operators("-", span));

            return None;
        }

        let operand = unary_expr.operand()?;

        if neg_count > 0
            && let cst::policy::Expression::Literal(lit_expr) = &operand
            && lit_expr.kind() == Some(cst::policy::LiteralKind::Int)
            && let Some(text) = lit_expr.as_int(self.source)
        {
            return self.lower_negated_integer(text, neg_count, not_count, lit_expr.range());
        }

        let mut result = self.lower_expr(&operand)?;
        for operator in operators.into_iter().rev() {
            result = match operator {
                cst::policy::UnaryOp::Not => {
                    ast::policy::Expr::unary(ast::policy::UnaryOp::Not, result)
                }
                cst::policy::UnaryOp::Neg => {
                    ast::policy::Expr::unary(ast::policy::UnaryOp::Neg, result)
                }
            };
        }

        Some(result)
    }

    fn lower_negated_integer(
        &mut self,
        text: &str,
        neg_count: usize,
        not_count: usize,
        span: Range<u32>,
    ) -> Option<ast::policy::Expr> {
        const I64_MIN_ABS: u64 = (i64::MAX as u64) + 1;

        let unsigned: u64 = text.parse().ok().or_else(|| {
            self.diagnostics
                .push(Diagnostic::invalid_integer(text, span.clone()));
            None
        })?;

        let (value, remaining_negs) = if unsigned == I64_MIN_ABS {
            (i64::MIN, neg_count.saturating_sub(1))
        } else if let Ok(signed) = i64::try_from(unsigned)
            && let Some(negated) = signed.checked_neg()
        {
            (negated, neg_count.saturating_sub(1))
        } else {
            self.diagnostics
                .push(Diagnostic::integer_overflow(text, span));

            return None;
        };

        let mut result = ast::policy::Expr::long(ast::policy::Integer::new(value));

        if remaining_negs % 2 == 1 {
            result = ast::policy::Expr::unary(ast::policy::UnaryOp::Neg, result);
        }

        for _ in 0..not_count {
            result = ast::policy::Expr::unary(ast::policy::UnaryOp::Not, result);
        }

        Some(result)
    }

    fn lower_member(
        &mut self,
        member_expr: &cst::policy::MemberExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let base = member_expr.base()?;
        let mut accesses = member_expr.accesses().peekable();

        if let cst::policy::Expression::Name(name) = &base
            && !name.is_qualified()
            && let Some(fn_name) = name.basename(self.source)
            && is_extension_function(fn_name)
            && let Some(cst::policy::MemberAccess::Call(method_call)) = accesses.peek()
        {
            let fn_name_ast =
                ast::common::Name::unqualified(ast::common::Id::new(String::from(fn_name)));
            let args: Vec<_> = method_call.arguments().collect();
            let mut fn_args = Vec::with_capacity(args.len());

            for arg in args {
                fn_args.push(self.lower_expr(&arg)?);
            }

            let _ = accesses.next();

            let mut result = ast::policy::Expr::extension_call(fn_name_ast, fn_args);

            for access in accesses {
                result = match access {
                    cst::policy::MemberAccess::Field(field_access) => {
                        let span = field_access.range();

                        if field_access.is_field_reserved() {
                            let field = field_access.field(self.source)?;
                            self.diagnostics
                                .push(Diagnostic::reserved_identifier(field, span));

                            return None;
                        }

                        let field = field_access.field(self.source)?;
                        let field = self.unescape_field(field, false, span)?;
                        ast::policy::Expr::get_attr(result, field)
                    }
                    cst::policy::MemberAccess::Call(method_call) => {
                        self.lower_method_call(result, &method_call)?
                    }
                    cst::policy::MemberAccess::Index(index_access) => {
                        let index = index_access.index()?;
                        let index = self.lower_expr(&index)?;
                        if let Some(key) = extract_string_literal(&index) {
                            ast::policy::Expr::get_attr(result, key)
                        } else {
                            self.diagnostics.push(Diagnostic::unknown_method(
                                "dynamic indexing",
                                index_access.range(),
                            ));

                            return None;
                        }
                    }
                };
            }

            return Some(result);
        }

        let mut result = self.lower_expr(&base)?;

        for access in accesses {
            result = match access {
                cst::policy::MemberAccess::Field(field_access) => {
                    let span = field_access.range();

                    if field_access.is_field_reserved() {
                        let field = field_access.field(self.source)?;
                        self.diagnostics
                            .push(Diagnostic::reserved_identifier(field, span));

                        return None;
                    }

                    let field = field_access.field(self.source)?;
                    let field = self.unescape_field(field, false, span)?;
                    ast::policy::Expr::get_attr(result, field)
                }
                cst::policy::MemberAccess::Call(method_call) => {
                    self.lower_method_call(result, &method_call)?
                }
                cst::policy::MemberAccess::Index(index_access) => {
                    let index = index_access.index()?;
                    let index = self.lower_expr(&index)?;
                    if let Some(key) = extract_string_literal(&index) {
                        ast::policy::Expr::get_attr(result, key)
                    } else {
                        self.diagnostics.push(Diagnostic::unknown_method(
                            "dynamic indexing",
                            index_access.range(),
                        ));

                        return None;
                    }
                }
            };
        }

        Some(result)
    }

    fn lower_method_call(
        &mut self,
        receiver: ast::policy::Expr,
        method_call: &cst::policy::MethodCall<'_>,
    ) -> Option<ast::policy::Expr> {
        let span = method_call.range();
        let method_name = method_call.name(self.source)?;

        let args: Vec<_> = method_call.arguments().collect();

        match method_name {
            "contains" => {
                self.check_arity(method_name, 1, args.len(), span);
                let arg = self.lower_expr(args.first()?)?;
                Some(ast::policy::Expr::binary(
                    ast::policy::BinaryOp::Contains,
                    receiver,
                    arg,
                ))
            }
            "containsAll" => {
                self.check_arity(method_name, 1, args.len(), span);
                let arg = self.lower_expr(args.first()?)?;
                Some(ast::policy::Expr::binary(
                    ast::policy::BinaryOp::ContainsAll,
                    receiver,
                    arg,
                ))
            }
            "containsAny" => {
                self.check_arity(method_name, 1, args.len(), span);
                let arg = self.lower_expr(args.first()?)?;
                Some(ast::policy::Expr::binary(
                    ast::policy::BinaryOp::ContainsAny,
                    receiver,
                    arg,
                ))
            }
            "isEmpty" => {
                self.check_arity(method_name, 0, args.len(), span);
                Some(ast::policy::Expr::unary(
                    ast::policy::UnaryOp::IsEmpty,
                    receiver,
                ))
            }
            "getTag" => {
                self.check_arity(method_name, 1, args.len(), span);
                let arg = self.lower_expr(args.first()?)?;
                Some(ast::policy::Expr::binary(
                    ast::policy::BinaryOp::GetTag,
                    receiver,
                    arg,
                ))
            }
            "hasTag" => {
                self.check_arity(method_name, 1, args.len(), span);
                let arg = self.lower_expr(args.first()?)?;
                Some(ast::policy::Expr::binary(
                    ast::policy::BinaryOp::HasTag,
                    receiver,
                    arg,
                ))
            }
            "isLoopback" | "isMulticast" | "isIpv4" | "isIpv6" | "isInRange" | "lessThan"
            | "lessThanOrEqual" | "greaterThan" | "greaterThanOrEqual" | "toDate" | "toTime"
            | "toDecimal" | "toDuration" | "offset" | "durationSince" | "toMilliseconds"
            | "toSeconds" | "toMinutes" | "toHours" | "toDays" => {
                let fn_name =
                    ast::common::Name::unqualified(ast::common::Id::new(String::from(method_name)));

                let mut fn_args = Vec::with_capacity(args.len().saturating_add(1));
                fn_args.push(receiver);

                for arg in args {
                    fn_args.push(self.lower_expr(&arg)?);
                }

                Some(ast::policy::Expr::extension_call(fn_name, fn_args))
            }
            _ => {
                self.diagnostics
                    .push(Diagnostic::unknown_method(method_name, span));
                None
            }
        }
    }

    fn lower_literal(
        &mut self,
        lit_expr: &cst::policy::LiteralExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let kind = lit_expr.kind()?;

        match kind {
            cst::policy::LiteralKind::Bool(value) => Some(ast::policy::Expr::bool(value)),
            cst::policy::LiteralKind::Int => {
                let text = lit_expr.as_int(self.source)?;
                let value = self.parse_integer(text, lit_expr.range())?;
                Some(ast::policy::Expr::long(value))
            }
            cst::policy::LiteralKind::String => {
                let text = lit_expr.as_string(self.source)?;
                let value = StringUnescaper::new(text).unescape().or_else(|| {
                    self.diagnostics.push(Diagnostic::invalid_string_escape(
                        "invalid escape",
                        lit_expr.range(),
                    ));
                    None
                })?;

                Some(ast::policy::Expr::string(value))
            }
        }
    }

    fn lower_entity_ref_expr(
        &mut self,
        entity_ref: &cst::policy::EntityRefExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let type_name = entity_ref.type_name()?;

        if let Some((reserved, span)) = type_name.first_reserved_segment(self.source) {
            self.diagnostics
                .push(Diagnostic::reserved_identifier(reserved, span));

            return None;
        }

        let name_text = type_name.basename(self.source)?;

        if !type_name.is_qualified() && is_extension_function(name_text) {
            let fn_name =
                ast::common::Name::unqualified(ast::common::Id::new(String::from(name_text)));
            let id_str = entity_ref.id(self.source)?;

            let arg = StringUnescaper::new(id_str).unescape().or_else(|| {
                self.diagnostics.push(Diagnostic::invalid_string_escape(
                    "invalid escape",
                    entity_ref.range(),
                ));
                None
            })?;

            return Some(ast::policy::Expr::extension_call(
                fn_name,
                alloc::vec![ast::policy::Expr::string(arg)],
            ));
        }

        let uid = self.lower_entity_ref_node(entity_ref)?;
        Some(ast::policy::Expr::literal(
            ast::policy::Literal::entity_uid(uid),
        ))
    }

    fn lower_slot(
        &mut self,
        slot_expr: &cst::policy::SlotExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let kind = slot_expr.kind()?;

        match kind {
            cst::policy::SlotKind::Principal => {
                Some(ast::policy::Expr::slot(ast::policy::SlotId::Principal))
            }
            cst::policy::SlotKind::Resource => {
                Some(ast::policy::Expr::slot(ast::policy::SlotId::Resource))
            }
            cst::policy::SlotKind::Other => {
                if let Some(name) = slot_expr.name(self.source) {
                    self.diagnostics
                        .push(Diagnostic::invalid_slot_id(name, slot_expr.range()));
                }
                None
            }
        }
    }

    fn lower_list(&mut self, list_expr: &cst::policy::ListExpression<'_>) -> ast::policy::Expr {
        let mut elements = Vec::new();

        for elem in list_expr.elements() {
            if let Some(expr) = self.lower_expr(&elem) {
                elements.push(expr);
            }
        }

        ast::policy::Expr::set(elements)
    }

    fn lower_record(
        &mut self,
        record_expr: &cst::policy::RecordExpression<'_>,
    ) -> Option<ast::policy::Expr> {
        let mut fields = BTreeMap::new();
        let mut seen: BTreeMap<String, Range<u32>> = BTreeMap::new();

        for entry in record_expr.entries() {
            let entry_span = entry.range();

            if entry.is_key_reserved() {
                let (key_text, _) = entry.key(self.source)?;
                self.diagnostics.push(Diagnostic::reserved_identifier(
                    key_text,
                    entry_span.clone(),
                ));

                continue;
            }

            let (key_text, is_quoted) = entry.key(self.source)?;
            let key = self.unescape_field(key_text, is_quoted, entry_span.clone())?;

            if let Some(first_span) = seen.get(&key) {
                self.diagnostics.push(Diagnostic::duplicate_key(
                    &key,
                    entry_span,
                    first_span.clone(),
                ));
                continue;
            }

            seen.insert(key.clone(), entry_span);

            let value = entry.value()?;
            let value = self.lower_expr(&value)?;

            fields.insert(key, value);
        }

        Some(ast::policy::Expr::record(fields))
    }

    fn check_arity(&mut self, name: &str, expected: usize, got: usize, span: Range<u32>) {
        if expected != got {
            self.diagnostics
                .push(Diagnostic::wrong_arity(name, expected, got, span));
        }
    }

    fn parse_integer(&mut self, text: &str, span: Range<u32>) -> Option<ast::policy::Integer> {
        let value: i64 = text.parse().ok().or_else(|| {
            self.diagnostics
                .push(Diagnostic::invalid_integer(text, span.clone()));

            None
        })?;

        Some(ast::policy::Integer::new(value))
    }

    fn unescape_field(&mut self, field: &str, is_quoted: bool, span: Range<u32>) -> Option<String> {
        if is_quoted {
            StringUnescaper::new(field).unescape().or_else(|| {
                self.diagnostics
                    .push(Diagnostic::invalid_string_escape("invalid escape", span));
                None
            })
        } else {
            Some(String::from(field))
        }
    }
}

fn lower_name(name: &cst::policy::Name<'_>, source: &str) -> Option<ast::common::Name> {
    let segments: Vec<&str> = name.segments(source).collect();
    if segments.is_empty() {
        return None;
    }

    let path: Vec<ast::common::Id> = segments
        .iter()
        .take(segments.len().saturating_sub(1))
        .map(|seg| ast::common::Id::new(String::from(*seg)))
        .collect();

    let basename = ast::common::Id::new(String::from(*segments.last()?));

    Some(ast::common::Name::new(path, basename))
}

fn lower_entity_type(
    name: &cst::policy::Name<'_>,
    source: &str,
) -> Option<ast::common::EntityType> {
    lower_name(name, source).map(ast::common::EntityType::new)
}

fn extract_string_literal(expr: &ast::policy::Expr) -> Option<String> {
    if let ast::policy::ExprKind::Literal(ast::policy::Literal::String(string)) = expr.kind() {
        Some(string.clone())
    } else {
        None
    }
}

fn is_extension_function(name: &str) -> bool {
    matches!(
        name,
        "ip" | "decimal" | "datetime" | "duration" | "date" | "time"
    )
}
