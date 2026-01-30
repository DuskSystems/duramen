//! EST → AST conversion for policies (infallible).

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use duramen_ast as ast;

use crate::convert::{entity_ref_to_uid, string_to_name};
use crate::json::expr::{Expr, ExprBuiltin, PatternElem, SlotId, Var};
use crate::json::policy::{
    ActionConstraint, ActionInTarget, Condition, ConditionKind, Effect, EntityTarget, InTarget,
    Policy, PolicySet, PrincipalConstraint, ResourceConstraint,
};
use crate::json::value::LiteralValue;

impl From<&Var> for ast::policy::Var {
    fn from(value: &Var) -> Self {
        match value {
            Var::Principal => Self::Principal,
            Var::Action => Self::Action,
            Var::Resource => Self::Resource,
            Var::Context => Self::Context,
        }
    }
}

impl From<&SlotId> for ast::policy::SlotId {
    fn from(value: &SlotId) -> Self {
        match value {
            SlotId::Principal => Self::Principal,
            SlotId::Resource => Self::Resource,
        }
    }
}

impl From<&LiteralValue> for ast::policy::Literal {
    fn from(value: &LiteralValue) -> Self {
        match value {
            LiteralValue::Bool(b) => Self::Bool(*b),
            LiteralValue::Long(n) => Self::Long((*n).into()),
            LiteralValue::String(s) => Self::String(s.clone()),
            LiteralValue::Entity(entity_value) => {
                Self::entity_uid(entity_ref_to_uid(&entity_value.entity))
            }
        }
    }
}

impl From<&PatternElem> for Vec<ast::policy::PatternElem> {
    fn from(value: &PatternElem) -> Self {
        match value {
            PatternElem::Wildcard => alloc::vec![ast::policy::PatternElem::Wildcard],
            PatternElem::Literal(lit) => lit
                .as_str()
                .chars()
                .map(ast::policy::PatternElem::Char)
                .collect(),
        }
    }
}

impl From<&Expr> for ast::policy::Expr {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Builtin(builtin) => Self::from(builtin),
            Expr::ExtensionCall(ext) => {
                let fn_name = string_to_name(&ext.fn_name);
                let args: Vec<Self> = ext.args.iter().map(Self::from).collect();
                Self::extension_call(fn_name, args)
            }
        }
    }
}

impl From<&ExprBuiltin> for ast::policy::Expr {
    fn from(value: &ExprBuiltin) -> Self {
        match value {
            ExprBuiltin::Value(value) => Self::literal(ast::policy::Literal::from(value)),
            ExprBuiltin::Var(var) => Self::var(ast::policy::Var::from(var)),
            ExprBuiltin::Slot(slot) => Self::slot(ast::policy::SlotId::from(slot)),

            ExprBuiltin::Set(elements) => {
                let ast_elements: Vec<Self> = elements.iter().map(Self::from).collect();
                Self::set(ast_elements)
            }

            ExprBuiltin::Record(fields) => {
                let ast_fields: BTreeMap<String, Self> = fields
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::from(v)))
                    .collect();
                Self::record(ast_fields)
            }

            ExprBuiltin::Not { arg } => {
                Self::unary(ast::policy::UnaryOp::Not, Self::from(arg.as_ref()))
            }

            ExprBuiltin::Neg { arg } => {
                Self::unary(ast::policy::UnaryOp::Neg, Self::from(arg.as_ref()))
            }

            ExprBuiltin::IsEmpty { arg } => {
                Self::unary(ast::policy::UnaryOp::IsEmpty, Self::from(arg.as_ref()))
            }

            ExprBuiltin::Eq { left, right } => Self::binary(
                ast::policy::BinaryOp::Eq,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            // Desugaring: != becomes !(==)
            ExprBuiltin::NotEq { left, right } => Self::unary(
                ast::policy::UnaryOp::Not,
                Self::binary(
                    ast::policy::BinaryOp::Eq,
                    Self::from(left.as_ref()),
                    Self::from(right.as_ref()),
                ),
            ),

            ExprBuiltin::Less { left, right } => Self::binary(
                ast::policy::BinaryOp::Less,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::LessEq { left, right } => Self::binary(
                ast::policy::BinaryOp::LessEq,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::Greater { left, right } => Self::binary(
                ast::policy::BinaryOp::Greater,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::GreaterEq { left, right } => Self::binary(
                ast::policy::BinaryOp::GreaterEq,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::And { left, right } => {
                Self::and(Self::from(left.as_ref()), Self::from(right.as_ref()))
            }

            ExprBuiltin::Or { left, right } => {
                Self::or(Self::from(left.as_ref()), Self::from(right.as_ref()))
            }

            ExprBuiltin::In { left, right } => Self::binary(
                ast::policy::BinaryOp::In,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::Add { left, right } => Self::binary(
                ast::policy::BinaryOp::Add,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::Sub { left, right } => Self::binary(
                ast::policy::BinaryOp::Sub,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::Mul { left, right } => Self::binary(
                ast::policy::BinaryOp::Mul,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::Contains { left, right } => Self::binary(
                ast::policy::BinaryOp::Contains,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::ContainsAll { left, right } => Self::binary(
                ast::policy::BinaryOp::ContainsAll,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::ContainsAny { left, right } => Self::binary(
                ast::policy::BinaryOp::ContainsAny,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::GetTag { left, right } => Self::binary(
                ast::policy::BinaryOp::GetTag,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::HasTag { left, right } => Self::binary(
                ast::policy::BinaryOp::HasTag,
                Self::from(left.as_ref()),
                Self::from(right.as_ref()),
            ),

            ExprBuiltin::GetAttr { left, attr } => {
                Self::get_attr(Self::from(left.as_ref()), attr.clone())
            }

            ExprBuiltin::HasAttr { left, attr } => {
                Self::has_attr(Self::from(left.as_ref()), attr.clone())
            }

            ExprBuiltin::Like { left, pattern } => {
                let ast_pattern: Vec<ast::policy::PatternElem> = pattern
                    .iter()
                    .flat_map(Vec::<ast::policy::PatternElem>::from)
                    .collect();
                Self::like(
                    Self::from(left.as_ref()),
                    ast::policy::Pattern::new(ast_pattern),
                )
            }

            // Desugaring: is...in becomes is && in
            ExprBuiltin::Is {
                left,
                entity_type,
                in_expr,
            } => {
                let expr = Self::from(left.as_ref());
                let et = ast::common::EntityType::new(string_to_name(entity_type));
                match in_expr {
                    Some(in_e) => Self::is_in(expr, et, Self::from(in_e.as_ref())),
                    None => Self::is(expr, et),
                }
            }

            ExprBuiltin::If {
                cond,
                then_expr,
                else_expr,
            } => Self::if_then_else(
                Self::from(cond.as_ref()),
                Self::from(then_expr.as_ref()),
                Self::from(else_expr.as_ref()),
            ),
        }
    }
}

impl From<&Effect> for ast::policy::Effect {
    fn from(value: &Effect) -> Self {
        match value {
            Effect::Permit => Self::Permit,
            Effect::Forbid => Self::Forbid,
        }
    }
}

fn entity_target_to_ref(target: &EntityTarget) -> ast::policy::EntityReference {
    match target {
        EntityTarget::Entity { entity } => {
            ast::policy::EntityReference::euid(entity_ref_to_uid(entity))
        }
        EntityTarget::Slot { .. } => ast::policy::EntityReference::Slot,
    }
}

fn in_target_to_ref(target: &InTarget) -> ast::policy::EntityReference {
    match target {
        InTarget::Entity { entity } => {
            ast::policy::EntityReference::euid(entity_ref_to_uid(entity))
        }
        InTarget::Slot { .. } => ast::policy::EntityReference::Slot,
    }
}

impl From<&PrincipalConstraint> for ast::policy::PrincipalConstraint {
    fn from(value: &PrincipalConstraint) -> Self {
        match value {
            PrincipalConstraint::All => Self::any(),

            PrincipalConstraint::Eq { target } => Self::equal(entity_target_to_ref(target)),

            PrincipalConstraint::In { target } => Self::is_in(entity_target_to_ref(target)),

            PrincipalConstraint::Is {
                entity_type,
                in_target: None,
            } => Self::is(ast::common::EntityType::new(string_to_name(entity_type))),

            PrincipalConstraint::Is {
                entity_type,
                in_target: Some(target),
            } => Self::is_in_type(
                ast::common::EntityType::new(string_to_name(entity_type)),
                in_target_to_ref(target),
            ),
        }
    }
}

impl From<&ResourceConstraint> for ast::policy::ResourceConstraint {
    fn from(value: &ResourceConstraint) -> Self {
        match value {
            ResourceConstraint::All => Self::any(),

            ResourceConstraint::Eq { target } => Self::equal(entity_target_to_ref(target)),

            ResourceConstraint::In { target } => Self::is_in(entity_target_to_ref(target)),

            ResourceConstraint::Is {
                entity_type,
                in_target: None,
            } => Self::is(ast::common::EntityType::new(string_to_name(entity_type))),

            ResourceConstraint::Is {
                entity_type,
                in_target: Some(target),
            } => Self::is_in_type(
                ast::common::EntityType::new(string_to_name(entity_type)),
                in_target_to_ref(target),
            ),
        }
    }
}

impl From<&ActionConstraint> for ast::policy::ActionConstraint {
    fn from(value: &ActionConstraint) -> Self {
        match value {
            ActionConstraint::All => Self::Any,
            ActionConstraint::Eq { entity } => Self::equal(entity_ref_to_uid(entity)),
            ActionConstraint::In { target } => match target {
                ActionInTarget::Single { entity } => {
                    Self::is_in(alloc::vec![entity_ref_to_uid(entity)])
                }
                ActionInTarget::Multiple { entities } => {
                    Self::is_in(entities.iter().map(entity_ref_to_uid).collect())
                }
            },
        }
    }
}

impl From<&Condition> for ast::policy::Clause {
    fn from(value: &Condition) -> Self {
        let kind = match value.kind {
            ConditionKind::When => ast::policy::ClauseKind::When,
            ConditionKind::Unless => ast::policy::ClauseKind::Unless,
        };
        Self::new(kind, ast::policy::Expr::from(&value.body))
    }
}

impl From<&Policy> for ast::policy::Template {
    fn from(value: &Policy) -> Self {
        let annotations: ast::common::Annotations = value
            .annotations
            .iter()
            .map(|(k, v)| {
                let key = ast::common::AnyId::new(k.clone());
                let annotation = if v.is_empty() {
                    ast::common::Annotation::without_value()
                } else {
                    ast::common::Annotation::with_value(v.clone())
                };
                (key, annotation)
            })
            .collect();

        let clauses: Vec<ast::policy::Clause> = value
            .conditions
            .iter()
            .map(ast::policy::Clause::from)
            .collect();

        Self::new(
            ast::policy::PolicyId::new("policy".into()),
            annotations,
            ast::policy::Effect::from(&value.effect),
            ast::policy::PrincipalConstraint::from(&value.principal),
            ast::policy::ActionConstraint::from(&value.action),
            ast::policy::ResourceConstraint::from(&value.resource),
            clauses,
        )
    }
}

impl From<&PolicySet> for Vec<ast::policy::Template> {
    fn from(value: &PolicySet) -> Self {
        let mut templates = Self::new();

        for (id, policy) in &value.static_policies {
            let template = ast::policy::Template::from(policy);
            let template = ast::policy::Template::new(
                ast::policy::PolicyId::new(id.clone()),
                template.annotations().clone(),
                template.effect(),
                template.principal().clone(),
                template.action().clone(),
                template.resource().clone(),
                template.clauses().to_vec(),
            );
            templates.push(template);
        }

        for (id, policy) in &value.templates {
            let template = ast::policy::Template::from(policy);
            let template = ast::policy::Template::new(
                ast::policy::PolicyId::new(id.clone()),
                template.annotations().clone(),
                template.effect(),
                template.principal().clone(),
                template.action().clone(),
                template.resource().clone(),
                template.clauses().to_vec(),
            );
            templates.push(template);
        }

        templates
    }
}
