//! AST → EST conversion for policies.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString as _};
use alloc::vec::Vec;

use duramen_ast as ast;

use crate::convert::{entity_type_to_string, entity_uid_to_ref, name_to_string};
use crate::json::expr::{Expr, PatternElem, PatternLiteral, SlotId, Var};
use crate::json::policy::{
    ActionConstraint, ActionInTarget, Condition, ConditionKind, Effect, EntityTarget, InTarget,
    Policy, PolicySet, PrincipalConstraint, ResourceConstraint,
};
use crate::json::value::{EntityRef, LiteralValue};

impl From<&ast::policy::Expr> for Expr {
    fn from(value: &ast::policy::Expr) -> Self {
        match value.kind() {
            ast::policy::ExprKind::Literal(lit) => match lit {
                ast::policy::Literal::Bool(b) => Self::value(LiteralValue::bool(*b)),
                ast::policy::Literal::Long(n) => Self::value(LiteralValue::long(n.get())),
                ast::policy::Literal::String(s) => Self::value(LiteralValue::string(s.clone())),
                ast::policy::Literal::EntityUid(uid) => Self::value(LiteralValue::entity(
                    EntityRef::new(name_to_string(uid.entity_type().name()), uid.eid().as_str()),
                )),
            },

            ast::policy::ExprKind::Var(var) => Self::var(var.into()),
            ast::policy::ExprKind::Slot(slot) => Self::slot(slot.into()),
            ast::policy::ExprKind::Unknown(_) => Self::extension_call("unknown", Vec::new()),

            ast::policy::ExprKind::Unary { op, arg } => {
                let arg_expr = Self::from(arg.as_ref());
                match op {
                    ast::policy::UnaryOp::Not => Self::not(arg_expr),
                    ast::policy::UnaryOp::Neg => Self::neg(arg_expr),
                    ast::policy::UnaryOp::IsEmpty => Self::is_empty(arg_expr),
                }
            }

            ast::policy::ExprKind::Binary { op, left, right } => {
                let left_expr = Self::from(left.as_ref());
                let right_expr = Self::from(right.as_ref());
                match op {
                    ast::policy::BinaryOp::Eq => Self::equal(left_expr, right_expr),
                    ast::policy::BinaryOp::Less => Self::less(left_expr, right_expr),
                    ast::policy::BinaryOp::LessEq => Self::less_eq(left_expr, right_expr),
                    ast::policy::BinaryOp::Greater => Self::greater(left_expr, right_expr),
                    ast::policy::BinaryOp::GreaterEq => Self::greater_eq(left_expr, right_expr),
                    ast::policy::BinaryOp::Add => Self::add(left_expr, right_expr),
                    ast::policy::BinaryOp::Sub => Self::sub(left_expr, right_expr),
                    ast::policy::BinaryOp::Mul => Self::mul(left_expr, right_expr),
                    ast::policy::BinaryOp::In => Self::is_in(left_expr, right_expr),
                    ast::policy::BinaryOp::Contains => Self::contains(left_expr, right_expr),
                    ast::policy::BinaryOp::ContainsAll => Self::contains_all(left_expr, right_expr),
                    ast::policy::BinaryOp::ContainsAny => Self::contains_any(left_expr, right_expr),
                    ast::policy::BinaryOp::GetTag => Self::get_tag(left_expr, right_expr),
                    ast::policy::BinaryOp::HasTag => Self::has_tag(left_expr, right_expr),
                }
            }

            ast::policy::ExprKind::And { left, right } => {
                Self::and(Self::from(left.as_ref()), Self::from(right.as_ref()))
            }

            ast::policy::ExprKind::Or { left, right } => {
                Self::or(Self::from(left.as_ref()), Self::from(right.as_ref()))
            }

            ast::policy::ExprKind::If {
                cond,
                then_expr,
                else_expr,
            } => Self::if_then_else(
                Self::from(cond.as_ref()),
                Self::from(then_expr.as_ref()),
                Self::from(else_expr.as_ref()),
            ),

            ast::policy::ExprKind::GetAttr { expr, attr } => {
                Self::get_attr(Self::from(expr.as_ref()), attr.clone())
            }

            ast::policy::ExprKind::HasAttr { expr, attr } => {
                Self::has_attr(Self::from(expr.as_ref()), attr.clone())
            }

            ast::policy::ExprKind::Is { expr, entity_type } => Self::is_type(
                Self::from(expr.as_ref()),
                entity_type_to_string(entity_type),
            ),

            ast::policy::ExprKind::Like { expr, pattern } => {
                let pattern_elems: Vec<PatternElem> =
                    pattern.iter().map(PatternElem::from).collect();
                Self::like(Self::from(expr.as_ref()), pattern_elems)
            }

            ast::policy::ExprKind::Set(elements) => {
                let est_elements: Vec<Self> = elements.iter().map(Self::from).collect();
                Self::set(est_elements)
            }

            ast::policy::ExprKind::Record(fields) => {
                let est_fields: BTreeMap<String, Self> = fields
                    .iter()
                    .map(|(k, v)| (k.clone(), Self::from(v)))
                    .collect();
                Self::record(est_fields)
            }

            ast::policy::ExprKind::ExtensionCall { fn_name, args } => {
                let est_args: Vec<Self> = args.iter().map(Self::from).collect();
                Self::extension_call(name_to_string(fn_name), est_args)
            }
        }
    }
}

impl From<&ast::policy::PatternElem> for PatternElem {
    fn from(value: &ast::policy::PatternElem) -> Self {
        match value {
            ast::policy::PatternElem::Char(c) => {
                Self::Literal(PatternLiteral::new_unchecked(c.to_string()))
            }
            ast::policy::PatternElem::Wildcard => Self::Wildcard,
        }
    }
}

impl From<&ast::policy::Var> for Var {
    fn from(value: &ast::policy::Var) -> Self {
        match value {
            ast::policy::Var::Principal => Self::Principal,
            ast::policy::Var::Action => Self::Action,
            ast::policy::Var::Resource => Self::Resource,
            ast::policy::Var::Context => Self::Context,
        }
    }
}

impl From<&ast::policy::SlotId> for SlotId {
    fn from(value: &ast::policy::SlotId) -> Self {
        match value {
            ast::policy::SlotId::Principal => Self::Principal,
            ast::policy::SlotId::Resource => Self::Resource,
        }
    }
}

fn entity_ref_to_principal_target(entity_ref: &ast::policy::EntityReference) -> EntityTarget {
    match entity_ref {
        ast::policy::EntityReference::Euid(uid) => EntityTarget::Entity {
            entity: entity_uid_to_ref(uid),
        },
        ast::policy::EntityReference::Slot => EntityTarget::Slot {
            slot: crate::json::policy::SlotId::Principal,
        },
    }
}

fn entity_ref_to_resource_target(entity_ref: &ast::policy::EntityReference) -> EntityTarget {
    match entity_ref {
        ast::policy::EntityReference::Euid(uid) => EntityTarget::Entity {
            entity: entity_uid_to_ref(uid),
        },
        ast::policy::EntityReference::Slot => EntityTarget::Slot {
            slot: crate::json::policy::SlotId::Resource,
        },
    }
}

fn entity_ref_to_principal_in_target(entity_ref: &ast::policy::EntityReference) -> InTarget {
    match entity_ref {
        ast::policy::EntityReference::Euid(uid) => InTarget::Entity {
            entity: entity_uid_to_ref(uid),
        },
        ast::policy::EntityReference::Slot => InTarget::Slot {
            slot: crate::json::policy::SlotId::Principal,
        },
    }
}

fn entity_ref_to_resource_in_target(entity_ref: &ast::policy::EntityReference) -> InTarget {
    match entity_ref {
        ast::policy::EntityReference::Euid(uid) => InTarget::Entity {
            entity: entity_uid_to_ref(uid),
        },
        ast::policy::EntityReference::Slot => InTarget::Slot {
            slot: crate::json::policy::SlotId::Resource,
        },
    }
}

impl From<&ast::policy::PrincipalConstraint> for PrincipalConstraint {
    fn from(value: &ast::policy::PrincipalConstraint) -> Self {
        match value.constraint() {
            ast::policy::PrincipalOrResourceConstraint::Any => Self::All,

            ast::policy::PrincipalOrResourceConstraint::Eq(entity_ref) => Self::Eq {
                target: entity_ref_to_principal_target(entity_ref),
            },

            ast::policy::PrincipalOrResourceConstraint::In(entity_ref) => Self::In {
                target: entity_ref_to_principal_target(entity_ref),
            },

            ast::policy::PrincipalOrResourceConstraint::Is(entity_type) => Self::Is {
                entity_type: entity_type_to_string(entity_type),
                in_target: None,
            },

            ast::policy::PrincipalOrResourceConstraint::IsIn(entity_type, entity_ref) => Self::Is {
                entity_type: entity_type_to_string(entity_type),
                in_target: Some(entity_ref_to_principal_in_target(entity_ref)),
            },
        }
    }
}

impl From<&ast::policy::ResourceConstraint> for ResourceConstraint {
    fn from(value: &ast::policy::ResourceConstraint) -> Self {
        match value.constraint() {
            ast::policy::PrincipalOrResourceConstraint::Any => Self::All,

            ast::policy::PrincipalOrResourceConstraint::Eq(entity_ref) => Self::Eq {
                target: entity_ref_to_resource_target(entity_ref),
            },

            ast::policy::PrincipalOrResourceConstraint::In(entity_ref) => Self::In {
                target: entity_ref_to_resource_target(entity_ref),
            },

            ast::policy::PrincipalOrResourceConstraint::Is(entity_type) => Self::Is {
                entity_type: entity_type_to_string(entity_type),
                in_target: None,
            },

            ast::policy::PrincipalOrResourceConstraint::IsIn(entity_type, entity_ref) => Self::Is {
                entity_type: entity_type_to_string(entity_type),
                in_target: Some(entity_ref_to_resource_in_target(entity_ref)),
            },
        }
    }
}

impl From<&ast::policy::ActionConstraint> for ActionConstraint {
    fn from(value: &ast::policy::ActionConstraint) -> Self {
        match value {
            ast::policy::ActionConstraint::Any => Self::All,
            ast::policy::ActionConstraint::Eq(uid) => Self::Eq {
                entity: entity_uid_to_ref(uid),
            },
            ast::policy::ActionConstraint::In(uids) => {
                let mut entities = uids.iter().map(|uid| entity_uid_to_ref(uid));
                let first = entities.next();
                let second = entities.next();
                match (first, second) {
                    (Some(entity), None) => Self::In {
                        target: ActionInTarget::Single { entity },
                    },
                    _ => Self::In {
                        target: ActionInTarget::Multiple {
                            entities: uids.iter().map(|uid| entity_uid_to_ref(uid)).collect(),
                        },
                    },
                }
            }
        }
    }
}

impl From<&ast::policy::Effect> for Effect {
    fn from(value: &ast::policy::Effect) -> Self {
        match value {
            ast::policy::Effect::Permit => Self::Permit,
            ast::policy::Effect::Forbid => Self::Forbid,
        }
    }
}

impl From<&ast::policy::Clause> for Condition {
    fn from(value: &ast::policy::Clause) -> Self {
        let kind = match value.kind() {
            ast::policy::ClauseKind::When => ConditionKind::When,
            ast::policy::ClauseKind::Unless => ConditionKind::Unless,
        };
        Self::new(kind, Expr::from(value.body()))
    }
}

impl From<&ast::policy::Template> for Policy {
    fn from(value: &ast::policy::Template) -> Self {
        let effect = Effect::from(&value.effect());
        let principal = PrincipalConstraint::from(value.principal());
        let action = ActionConstraint::from(value.action());
        let resource = ResourceConstraint::from(value.resource());
        let conditions: Vec<Condition> = value.clauses().iter().map(Condition::from).collect();

        let annotations: BTreeMap<String, String> = value
            .annotations()
            .iter()
            .map(|(k, v)| (k.as_str().into(), v.value().unwrap_or("").into()))
            .collect();

        Self::new(effect, principal, action, resource, conditions, annotations)
    }
}

impl From<&[ast::policy::Template]> for PolicySet {
    fn from(value: &[ast::policy::Template]) -> Self {
        let mut policy_set = Self::new();

        for template in value {
            let id = template.id().as_str().into();
            let policy = Policy::from(template);

            if template.is_static() {
                policy_set.add_static_policy(id, policy);
            } else {
                policy_set.add_template(id, policy);
            }
        }

        policy_set
    }
}
