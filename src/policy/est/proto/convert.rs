use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::types as proto;
use crate::policy::est::types as est;

fn binary_kind(
    op: proto::expr::binary_app::Op,
    left: &est::Expression,
    right: &est::Expression,
) -> proto::expr::ExprKind {
    proto::expr::ExprKind::BApp(Box::new(proto::expr::BinaryApp {
        op: op as i32,
        left: Some(Box::new(expression_to_proto(left))),
        right: Some(Box::new(expression_to_proto(right))),
    }))
}

fn negated_binary_kind(
    op: proto::expr::binary_app::Op,
    left: &est::Expression,
    right: &est::Expression,
) -> proto::expr::ExprKind {
    proto::expr::ExprKind::UApp(Box::new(proto::expr::UnaryApp {
        op: proto::expr::unary_app::Op::Not as i32,
        expr: Some(Box::new(proto::Expr {
            expr_kind: Some(binary_kind(op, left, right)),
        })),
    }))
}

fn method_binary_kind(
    op: proto::expr::binary_app::Op,
    receiver: &est::Expression,
    arguments: &[est::Expression],
) -> proto::expr::ExprKind {
    proto::expr::ExprKind::BApp(Box::new(proto::expr::BinaryApp {
        op: op as i32,
        left: Some(Box::new(expression_to_proto(receiver))),
        right: arguments
            .first()
            .map(|arg| Box::new(expression_to_proto(arg))),
    }))
}

#[must_use]
pub fn policies_to_proto(policies: &[est::Policy]) -> proto::PolicySet {
    let mut templates = Vec::new();
    let mut links = Vec::new();

    for (index, policy) in policies.iter().enumerate() {
        let policy_id = format!("policy{index}");
        let template_body = policy_to_template_body(&policy_id, policy);
        templates.push(template_body);

        if !policy.is_template() {
            links.push(proto::Policy {
                template_id: policy_id,
                link_id: None,
                is_template_link: false,
                principal_euid: None,
                resource_euid: None,
            });
        }
    }

    proto::PolicySet { templates, links }
}

fn policy_to_template_body(id: &str, policy: &est::Policy) -> proto::TemplateBody {
    let effect = match policy.effect {
        est::Effect::Permit => proto::Effect::Permit as i32,
        est::Effect::Forbid => proto::Effect::Forbid as i32,
    };

    let annotations: BTreeMap<String, String> = policy
        .annotations
        .iter()
        .map(|(key, value)| (key.clone(), value.clone().unwrap_or_default()))
        .collect();

    let non_scope_constraints = build_non_scope_constraints(&policy.conditions);

    proto::TemplateBody {
        id: String::from(id),
        annotations,
        effect,
        principal_constraint: Some(principal_or_resource_to_proto(&policy.principal)),
        action_constraint: Some(action_to_proto(&policy.action)),
        resource_constraint: Some(principal_or_resource_to_proto(&policy.resource)),
        non_scope_constraints,
    }
}

fn build_non_scope_constraints(conditions: &[est::Condition]) -> Option<proto::Expr> {
    let mut result: Option<proto::Expr> = None;

    for condition in conditions.iter().rev() {
        let condition_expr = expression_to_proto(&condition.body);
        let expr = match condition.kind {
            est::ConditionKind::When => condition_expr,
            est::ConditionKind::Unless => proto::Expr {
                expr_kind: Some(proto::expr::ExprKind::UApp(Box::new(
                    proto::expr::UnaryApp {
                        op: proto::expr::unary_app::Op::Not as i32,
                        expr: Some(Box::new(condition_expr)),
                    },
                ))),
            },
        };

        result = Some(match result {
            None => expr,
            Some(right) => proto::Expr {
                expr_kind: Some(proto::expr::ExprKind::And(Box::new(proto::expr::And {
                    left: Some(Box::new(expr)),
                    right: Some(Box::new(right)),
                }))),
            },
        });
    }

    result
}

fn principal_or_resource_to_proto(
    constraint: &est::PrincipalOrResourceConstraint,
) -> proto::PrincipalOrResourceConstraint {
    use proto::principal_or_resource_constraint as porc;

    let data = match constraint {
        est::PrincipalOrResourceConstraint::Any => porc::Data::Any(porc::Any::Unit as i32),
        est::PrincipalOrResourceConstraint::Equal(expr) => porc::Data::Eq(porc::EqMessage {
            er: Some(expression_to_entity_reference(expr)),
        }),
        est::PrincipalOrResourceConstraint::In(expr) => porc::Data::In(porc::InMessage {
            er: Some(expression_to_entity_reference(expr)),
        }),
        est::PrincipalOrResourceConstraint::Is { entity_type } => porc::Data::Is(porc::IsMessage {
            entity_type: Some(parse_name(entity_type)),
        }),
        est::PrincipalOrResourceConstraint::IsIn {
            entity_type,
            in_entity,
        } => porc::Data::IsIn(porc::IsInMessage {
            er: Some(expression_to_entity_reference(in_entity)),
            entity_type: Some(parse_name(entity_type)),
        }),
    };

    proto::PrincipalOrResourceConstraint { data: Some(data) }
}

fn action_to_proto(constraint: &est::ActionConstraint) -> proto::ActionConstraint {
    use proto::action_constraint as ac;

    let data = match constraint {
        est::ActionConstraint::Any => ac::Data::Any(ac::Any::Unit as i32),
        est::ActionConstraint::Equal(expr) => ac::Data::Eq(ac::EqMessage {
            euid: expression_to_entity_uid(expr),
        }),
        est::ActionConstraint::In(exprs) => ac::Data::In(ac::InMessage {
            euids: exprs.iter().filter_map(expression_to_entity_uid).collect(),
        }),
    };

    proto::ActionConstraint { data: Some(data) }
}

fn expression_to_entity_reference(expr: &est::Expression) -> proto::EntityReference {
    use proto::entity_reference as er;

    let data = match expr {
        est::Expression::Slot(est::SlotId::Principal | est::SlotId::Resource) => {
            er::Data::Slot(er::Slot::Unit as i32)
        }
        est::Expression::Entity { entity_type, id } => er::Data::Euid(proto::EntityUid {
            ty: Some(parse_name(entity_type)),
            eid: id.clone(),
        }),
        _ => er::Data::Euid(proto::EntityUid {
            ty: Some(proto::Name {
                id: String::from("Unknown"),
                path: Vec::new(),
            }),
            eid: String::from("unknown"),
        }),
    };

    proto::EntityReference { data: Some(data) }
}

fn expression_to_entity_uid(expr: &est::Expression) -> Option<proto::EntityUid> {
    match expr {
        est::Expression::Entity { entity_type, id } => Some(proto::EntityUid {
            ty: Some(parse_name(entity_type)),
            eid: id.clone(),
        }),
        _ => None,
    }
}

fn parse_name(name: &str) -> proto::Name {
    let parts: Vec<&str> = name.split("::").collect();
    if parts.is_empty() {
        return proto::Name {
            id: String::new(),
            path: Vec::new(),
        };
    }

    let parts_len = parts.len();
    let id = parts
        .get(parts_len.saturating_sub(1))
        .map_or(String::new(), |segment| String::from(*segment));
    let path: Vec<String> = parts
        .get(..parts_len.saturating_sub(1))
        .unwrap_or_default()
        .iter()
        .map(|segment| String::from(*segment))
        .collect();

    proto::Name { id, path }
}

fn expression_to_proto(expr: &est::Expression) -> proto::Expr {
    use proto::expr::{
        And, BinaryApp, ExprKind, ExtensionFunctionApp, GetAttr, HasAttr, If, Is, Like, Literal,
        Or, Record, Set, UnaryApp, Var, binary_app, like, literal, unary_app,
    };

    let kind = match expr {
        est::Expression::Boolean(value) => ExprKind::Lit(Literal {
            lit: Some(literal::Lit::B(*value)),
        }),
        est::Expression::Integer(value) => ExprKind::Lit(Literal {
            lit: Some(literal::Lit::I(*value)),
        }),
        est::Expression::String(value) => ExprKind::Lit(Literal {
            lit: Some(literal::Lit::S(value.clone())),
        }),
        est::Expression::Entity { entity_type, id } => ExprKind::Lit(Literal {
            lit: Some(literal::Lit::Euid(proto::EntityUid {
                ty: Some(parse_name(entity_type)),
                eid: id.clone(),
            })),
        }),
        est::Expression::Variable(var) => {
            let var_enum = match var {
                est::Variable::Principal => Var::Principal,
                est::Variable::Action => Var::Action,
                est::Variable::Resource => Var::Resource,
                est::Variable::Context => Var::Context,
            };
            ExprKind::Var(var_enum as i32)
        }
        est::Expression::Slot(slot) => {
            let slot_enum = match slot {
                est::SlotId::Principal => proto::SlotId::Principal,
                est::SlotId::Resource => proto::SlotId::Resource,
            };
            ExprKind::Slot(slot_enum as i32)
        }
        est::Expression::Set(elements) => ExprKind::Set(Set {
            elements: elements.iter().map(expression_to_proto).collect(),
        }),
        est::Expression::Record(entries) => ExprKind::Record(Record {
            items: entries
                .iter()
                .map(|(key, value)| (key.clone(), expression_to_proto(value)))
                .collect(),
        }),
        est::Expression::Not(inner) => ExprKind::UApp(Box::new(UnaryApp {
            op: unary_app::Op::Not as i32,
            expr: Some(Box::new(expression_to_proto(inner))),
        })),
        est::Expression::Negate(inner) => ExprKind::UApp(Box::new(UnaryApp {
            op: unary_app::Op::Neg as i32,
            expr: Some(Box::new(expression_to_proto(inner))),
        })),
        est::Expression::Or(left, right) => ExprKind::Or(Box::new(Or {
            left: Some(Box::new(expression_to_proto(left))),
            right: Some(Box::new(expression_to_proto(right))),
        })),
        est::Expression::And(left, right) => ExprKind::And(Box::new(And {
            left: Some(Box::new(expression_to_proto(left))),
            right: Some(Box::new(expression_to_proto(right))),
        })),
        est::Expression::Equal(left, right) => binary_kind(binary_app::Op::Eq, left, right),
        est::Expression::NotEqual(left, right) => {
            negated_binary_kind(binary_app::Op::Eq, left, right)
        }
        est::Expression::LessThan(left, right) => binary_kind(binary_app::Op::Less, left, right),
        est::Expression::LessThanOrEqual(left, right) => {
            binary_kind(binary_app::Op::LessEq, left, right)
        }
        est::Expression::GreaterThan(left, right) => {
            negated_binary_kind(binary_app::Op::LessEq, left, right)
        }
        est::Expression::GreaterThanOrEqual(left, right) => {
            negated_binary_kind(binary_app::Op::Less, left, right)
        }
        est::Expression::In(left, right) => binary_kind(binary_app::Op::In, left, right),
        est::Expression::Add(left, right) => binary_kind(binary_app::Op::Add, left, right),
        est::Expression::Subtract(left, right) => binary_kind(binary_app::Op::Sub, left, right),
        est::Expression::Multiply(left, right) => binary_kind(binary_app::Op::Mul, left, right),
        est::Expression::GetAttribute {
            expression,
            attribute,
        } => ExprKind::GetAttr(Box::new(GetAttr {
            expr: Some(Box::new(expression_to_proto(expression))),
            attr: attribute.clone(),
        })),
        est::Expression::HasAttribute {
            expression,
            attribute,
        } => ExprKind::HasAttr(Box::new(HasAttr {
            expr: Some(Box::new(expression_to_proto(expression))),
            attr: attribute.clone(),
        })),
        est::Expression::Index { expression, index } => ExprKind::ExtApp(ExtensionFunctionApp {
            fn_name: Some(proto::Name {
                id: String::from("index"),
                path: Vec::new(),
            }),
            args: alloc::vec![expression_to_proto(expression), expression_to_proto(index)],
        }),
        est::Expression::Like {
            expression,
            pattern,
        } => {
            let pattern_elems: Vec<like::PatternElem> = pattern
                .iter()
                .map(|elem| {
                    use like::pattern_elem;
                    let data = match elem {
                        est::PatternElement::Wildcard => {
                            pattern_elem::Data::Wildcard(pattern_elem::Wildcard::Unit as i32)
                        }
                        est::PatternElement::Literal(s) => pattern_elem::Data::C(s.clone()),
                    };
                    like::PatternElem { data: Some(data) }
                })
                .collect();

            ExprKind::Like(Box::new(Like {
                expr: Some(Box::new(expression_to_proto(expression))),
                pattern: pattern_elems,
            }))
        }
        est::Expression::Is {
            expression,
            entity_type,
            in_expression,
        } => {
            let is_expr = proto::Expr {
                expr_kind: Some(ExprKind::Is(Box::new(Is {
                    expr: Some(Box::new(expression_to_proto(expression))),
                    entity_type: Some(parse_name(entity_type)),
                }))),
            };

            if let Some(in_expr) = in_expression {
                ExprKind::And(Box::new(And {
                    left: Some(Box::new(is_expr)),
                    right: Some(Box::new(proto::Expr {
                        expr_kind: Some(ExprKind::BApp(Box::new(BinaryApp {
                            op: binary_app::Op::In as i32,
                            left: Some(Box::new(expression_to_proto(expression))),
                            right: Some(Box::new(expression_to_proto(in_expr))),
                        }))),
                    })),
                }))
            } else {
                is_expr.expr_kind.unwrap_or(ExprKind::Lit(Literal {
                    lit: Some(literal::Lit::B(true)),
                }))
            }
        }
        est::Expression::If {
            condition,
            then_expression,
            else_expression,
        } => ExprKind::If(Box::new(If {
            test_expr: Some(Box::new(expression_to_proto(condition))),
            then_expr: Some(Box::new(expression_to_proto(then_expression))),
            else_expr: Some(Box::new(expression_to_proto(else_expression))),
        })),
        est::Expression::MethodCall {
            receiver,
            method,
            arguments,
        } => method_to_proto(receiver, method, arguments),
        est::Expression::ExtensionCall { name, arguments } => {
            ExprKind::ExtApp(ExtensionFunctionApp {
                fn_name: Some(parse_name(name)),
                args: arguments.iter().map(expression_to_proto).collect(),
            })
        }
    };

    proto::Expr {
        expr_kind: Some(kind),
    }
}

fn method_to_proto(
    receiver: &est::Expression,
    method: &str,
    arguments: &[est::Expression],
) -> proto::expr::ExprKind {
    use proto::expr::{ExprKind, ExtensionFunctionApp, UnaryApp, binary_app, unary_app};

    match method {
        "contains" => method_binary_kind(binary_app::Op::Contains, receiver, arguments),
        "containsAll" => method_binary_kind(binary_app::Op::ContainsAll, receiver, arguments),
        "containsAny" => method_binary_kind(binary_app::Op::ContainsAny, receiver, arguments),
        "isEmpty" => ExprKind::UApp(Box::new(UnaryApp {
            op: unary_app::Op::IsEmpty as i32,
            expr: Some(Box::new(expression_to_proto(receiver))),
        })),
        "hasTag" => method_binary_kind(binary_app::Op::HasTag, receiver, arguments),
        "getTag" => method_binary_kind(binary_app::Op::GetTag, receiver, arguments),
        _ => {
            let mut args = alloc::vec![expression_to_proto(receiver)];
            args.extend(arguments.iter().map(expression_to_proto));
            ExprKind::ExtApp(ExtensionFunctionApp {
                fn_name: Some(parse_name(method)),
                args,
            })
        }
    }
}
