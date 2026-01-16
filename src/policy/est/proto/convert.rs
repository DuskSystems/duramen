use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::types as proto;
use crate::policy::est::types as est;

fn binary_kind(
    op: proto::expr::binary_app::Op,
    left: &est::Expression<'_>,
    right: &est::Expression<'_>,
) -> proto::expr::ExprKind {
    proto::expr::ExprKind::BApp(Box::new(proto::expr::BinaryApp {
        op: op as i32,
        left: Some(Box::new(expression_to_proto(left))),
        right: Some(Box::new(expression_to_proto(right))),
    }))
}

fn negated_binary_kind(
    op: proto::expr::binary_app::Op,
    left: &est::Expression<'_>,
    right: &est::Expression<'_>,
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
    receiver: &est::Expression<'_>,
    arguments: &[est::Expression<'_>],
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
pub fn policies_to_proto(policies: &[est::Policy<'_>]) -> proto::PolicySet {
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

fn policy_to_template_body(id: &str, policy: &est::Policy<'_>) -> proto::TemplateBody {
    let effect = match policy.effect {
        est::Effect::Permit => proto::Effect::Permit as i32,
        est::Effect::Forbid => proto::Effect::Forbid as i32,
    };

    let annotations: BTreeMap<String, String> = policy
        .annotations
        .iter()
        .map(|(key, value)| {
            (
                (*key).to_owned(),
                value.map_or(String::new(), str::to_owned),
            )
        })
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

fn build_non_scope_constraints(conditions: &[est::Condition<'_>]) -> Option<proto::Expr> {
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
    constraint: &est::PrincipalOrResourceConstraint<'_>,
) -> proto::PrincipalOrResourceConstraint {
    let data = match constraint {
        est::PrincipalOrResourceConstraint::Any => {
            proto::principal_or_resource_constraint::Data::Any(
                proto::principal_or_resource_constraint::Any::Unit as i32,
            )
        }
        est::PrincipalOrResourceConstraint::Equal(expr) => {
            proto::principal_or_resource_constraint::Data::Eq(
                proto::principal_or_resource_constraint::EqMessage {
                    er: Some(expression_to_entity_reference(expr)),
                },
            )
        }
        est::PrincipalOrResourceConstraint::In(expr) => {
            proto::principal_or_resource_constraint::Data::In(
                proto::principal_or_resource_constraint::InMessage {
                    er: Some(expression_to_entity_reference(expr)),
                },
            )
        }
        est::PrincipalOrResourceConstraint::Is { entity_type } => {
            proto::principal_or_resource_constraint::Data::Is(
                proto::principal_or_resource_constraint::IsMessage {
                    entity_type: Some(parse_name(entity_type)),
                },
            )
        }
        est::PrincipalOrResourceConstraint::IsIn {
            entity_type,
            in_entity,
        } => proto::principal_or_resource_constraint::Data::IsIn(
            proto::principal_or_resource_constraint::IsInMessage {
                er: Some(expression_to_entity_reference(in_entity)),
                entity_type: Some(parse_name(entity_type)),
            },
        ),
    };

    proto::PrincipalOrResourceConstraint { data: Some(data) }
}

fn action_to_proto(constraint: &est::ActionConstraint<'_>) -> proto::ActionConstraint {
    let data = match constraint {
        est::ActionConstraint::Any => {
            proto::action_constraint::Data::Any(proto::action_constraint::Any::Unit as i32)
        }
        est::ActionConstraint::Equal(expr) => {
            proto::action_constraint::Data::Eq(proto::action_constraint::EqMessage {
                euid: expression_to_entity_uid(expr),
            })
        }
        est::ActionConstraint::In(exprs) => {
            proto::action_constraint::Data::In(proto::action_constraint::InMessage {
                euids: exprs.iter().filter_map(expression_to_entity_uid).collect(),
            })
        }
    };

    proto::ActionConstraint { data: Some(data) }
}

fn expression_to_entity_reference(expr: &est::Expression<'_>) -> proto::EntityReference {
    let data = match expr {
        est::Expression::Slot(est::SlotId::Principal | est::SlotId::Resource) => {
            proto::entity_reference::Data::Slot(proto::entity_reference::Slot::Unit as i32)
        }
        est::Expression::Entity { entity_type, id } => {
            proto::entity_reference::Data::Euid(proto::EntityUid {
                ty: Some(parse_name(entity_type)),
                eid: (*id).to_owned(),
            })
        }
        _ => proto::entity_reference::Data::Euid(proto::EntityUid {
            ty: Some(proto::Name {
                id: String::from("Unknown"),
                path: Vec::new(),
            }),
            eid: String::from("unknown"),
        }),
    };

    proto::EntityReference { data: Some(data) }
}

fn expression_to_entity_uid(expr: &est::Expression<'_>) -> Option<proto::EntityUid> {
    match expr {
        est::Expression::Entity { entity_type, id } => Some(proto::EntityUid {
            ty: Some(parse_name(entity_type)),
            eid: (*id).to_owned(),
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

fn expression_to_proto(expr: &est::Expression<'_>) -> proto::Expr {
    let kind = match expr {
        est::Expression::Boolean(value) => proto::expr::ExprKind::Lit(proto::expr::Literal {
            lit: Some(proto::expr::literal::Lit::B(*value)),
        }),
        est::Expression::Integer(value) => proto::expr::ExprKind::Lit(proto::expr::Literal {
            lit: Some(proto::expr::literal::Lit::I(*value)),
        }),
        est::Expression::String(value) => proto::expr::ExprKind::Lit(proto::expr::Literal {
            lit: Some(proto::expr::literal::Lit::S((*value).to_owned())),
        }),
        est::Expression::Entity { entity_type, id } => {
            proto::expr::ExprKind::Lit(proto::expr::Literal {
                lit: Some(proto::expr::literal::Lit::Euid(proto::EntityUid {
                    ty: Some(parse_name(entity_type)),
                    eid: (*id).to_owned(),
                })),
            })
        }
        est::Expression::Variable(var) => {
            let var_enum = match var {
                est::Variable::Principal => proto::expr::Var::Principal,
                est::Variable::Action => proto::expr::Var::Action,
                est::Variable::Resource => proto::expr::Var::Resource,
                est::Variable::Context => proto::expr::Var::Context,
            };
            proto::expr::ExprKind::Var(var_enum as i32)
        }
        est::Expression::Slot(slot) => {
            let slot_enum = match slot {
                est::SlotId::Principal => proto::SlotId::Principal,
                est::SlotId::Resource => proto::SlotId::Resource,
            };
            proto::expr::ExprKind::Slot(slot_enum as i32)
        }
        est::Expression::Set(elements) => proto::expr::ExprKind::Set(proto::expr::Set {
            elements: elements.iter().map(expression_to_proto).collect(),
        }),
        est::Expression::Record(entries) => proto::expr::ExprKind::Record(proto::expr::Record {
            items: entries
                .iter()
                .map(|(key, value)| ((*key).to_owned(), expression_to_proto(value)))
                .collect(),
        }),
        est::Expression::Not(inner) => {
            proto::expr::ExprKind::UApp(Box::new(proto::expr::UnaryApp {
                op: proto::expr::unary_app::Op::Not as i32,
                expr: Some(Box::new(expression_to_proto(inner))),
            }))
        }
        est::Expression::Negate(inner) => {
            proto::expr::ExprKind::UApp(Box::new(proto::expr::UnaryApp {
                op: proto::expr::unary_app::Op::Neg as i32,
                expr: Some(Box::new(expression_to_proto(inner))),
            }))
        }
        est::Expression::Or(left, right) => proto::expr::ExprKind::Or(Box::new(proto::expr::Or {
            left: Some(Box::new(expression_to_proto(left))),
            right: Some(Box::new(expression_to_proto(right))),
        })),
        est::Expression::And(left, right) => {
            proto::expr::ExprKind::And(Box::new(proto::expr::And {
                left: Some(Box::new(expression_to_proto(left))),
                right: Some(Box::new(expression_to_proto(right))),
            }))
        }
        est::Expression::Equal(left, right) => {
            binary_kind(proto::expr::binary_app::Op::Eq, left, right)
        }
        est::Expression::NotEqual(left, right) => {
            negated_binary_kind(proto::expr::binary_app::Op::Eq, left, right)
        }
        est::Expression::LessThan(left, right) => {
            binary_kind(proto::expr::binary_app::Op::Less, left, right)
        }
        est::Expression::LessThanOrEqual(left, right) => {
            binary_kind(proto::expr::binary_app::Op::LessEq, left, right)
        }
        est::Expression::GreaterThan(left, right) => {
            negated_binary_kind(proto::expr::binary_app::Op::LessEq, left, right)
        }
        est::Expression::GreaterThanOrEqual(left, right) => {
            negated_binary_kind(proto::expr::binary_app::Op::Less, left, right)
        }
        est::Expression::In(left, right) => {
            binary_kind(proto::expr::binary_app::Op::In, left, right)
        }
        est::Expression::Add(left, right) => {
            binary_kind(proto::expr::binary_app::Op::Add, left, right)
        }
        est::Expression::Subtract(left, right) => {
            binary_kind(proto::expr::binary_app::Op::Sub, left, right)
        }
        est::Expression::Multiply(left, right) => {
            binary_kind(proto::expr::binary_app::Op::Mul, left, right)
        }
        est::Expression::GetAttribute {
            expression,
            attribute,
        } => proto::expr::ExprKind::GetAttr(Box::new(proto::expr::GetAttr {
            expr: Some(Box::new(expression_to_proto(expression))),
            attr: (*attribute).to_owned(),
        })),
        est::Expression::HasAttribute {
            expression,
            attribute,
        } => proto::expr::ExprKind::HasAttr(Box::new(proto::expr::HasAttr {
            expr: Some(Box::new(expression_to_proto(expression))),
            attr: (*attribute).to_owned(),
        })),
        est::Expression::Index { expression, index } => {
            proto::expr::ExprKind::ExtApp(proto::expr::ExtensionFunctionApp {
                fn_name: Some(proto::Name {
                    id: String::from("index"),
                    path: Vec::new(),
                }),
                args: alloc::vec![expression_to_proto(expression), expression_to_proto(index)],
            })
        }
        est::Expression::Like {
            expression,
            pattern,
        } => {
            let pattern_elems: Vec<proto::expr::like::PatternElem> = pattern
                .iter()
                .map(|elem| {
                    let data = match elem {
                        est::PatternElement::Wildcard => {
                            proto::expr::like::pattern_elem::Data::Wildcard(
                                proto::expr::like::pattern_elem::Wildcard::Unit as i32,
                            )
                        }
                        est::PatternElement::Literal(s) => {
                            proto::expr::like::pattern_elem::Data::C((*s).to_owned())
                        }
                    };
                    proto::expr::like::PatternElem { data: Some(data) }
                })
                .collect();

            proto::expr::ExprKind::Like(Box::new(proto::expr::Like {
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
                expr_kind: Some(proto::expr::ExprKind::Is(Box::new(proto::expr::Is {
                    expr: Some(Box::new(expression_to_proto(expression))),
                    entity_type: Some(parse_name(entity_type)),
                }))),
            };

            if let Some(in_expr) = in_expression {
                proto::expr::ExprKind::And(Box::new(proto::expr::And {
                    left: Some(Box::new(is_expr)),
                    right: Some(Box::new(proto::Expr {
                        expr_kind: Some(proto::expr::ExprKind::BApp(Box::new(
                            proto::expr::BinaryApp {
                                op: proto::expr::binary_app::Op::In as i32,
                                left: Some(Box::new(expression_to_proto(expression))),
                                right: Some(Box::new(expression_to_proto(in_expr))),
                            },
                        ))),
                    })),
                }))
            } else {
                is_expr
                    .expr_kind
                    .unwrap_or(proto::expr::ExprKind::Lit(proto::expr::Literal {
                        lit: Some(proto::expr::literal::Lit::B(true)),
                    }))
            }
        }
        est::Expression::If {
            condition,
            then_expression,
            else_expression,
        } => proto::expr::ExprKind::If(Box::new(proto::expr::If {
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
            proto::expr::ExprKind::ExtApp(proto::expr::ExtensionFunctionApp {
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
    receiver: &est::Expression<'_>,
    method: &str,
    arguments: &[est::Expression<'_>],
) -> proto::expr::ExprKind {
    match method {
        "contains" => {
            method_binary_kind(proto::expr::binary_app::Op::Contains, receiver, arguments)
        }
        "containsAll" => method_binary_kind(
            proto::expr::binary_app::Op::ContainsAll,
            receiver,
            arguments,
        ),
        "containsAny" => method_binary_kind(
            proto::expr::binary_app::Op::ContainsAny,
            receiver,
            arguments,
        ),
        "isEmpty" => proto::expr::ExprKind::UApp(Box::new(proto::expr::UnaryApp {
            op: proto::expr::unary_app::Op::IsEmpty as i32,
            expr: Some(Box::new(expression_to_proto(receiver))),
        })),
        "hasTag" => method_binary_kind(proto::expr::binary_app::Op::HasTag, receiver, arguments),
        "getTag" => method_binary_kind(proto::expr::binary_app::Op::GetTag, receiver, arguments),
        _ => {
            let mut args = alloc::vec![expression_to_proto(receiver)];
            args.extend(arguments.iter().map(expression_to_proto));
            proto::expr::ExprKind::ExtApp(proto::expr::ExtensionFunctionApp {
                fn_name: Some(parse_name(method)),
                args,
            })
        }
    }
}
