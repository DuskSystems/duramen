use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

use super::types as json;
use crate::policy::est::types as est;

#[must_use]
pub fn policies_to_json(policies: &[est::Policy<'_>]) -> json::PolicySetJson {
    let mut templates = BTreeMap::new();
    let mut static_policies = BTreeMap::new();

    for (index, policy) in policies.iter().enumerate() {
        let policy_id = format!("policy{index}");
        let policy_json = policy_to_json(policy);

        if policy.is_template() {
            templates.insert(policy_id, policy_json);
        } else {
            static_policies.insert(policy_id, policy_json);
        }
    }

    json::PolicySetJson {
        templates,
        static_policies,
        template_links: Vec::new(),
    }
}

fn policy_to_json(policy: &est::Policy<'_>) -> json::PolicyJson {
    let effect = match policy.effect {
        est::Effect::Permit => String::from("permit"),
        est::Effect::Forbid => String::from("forbid"),
    };

    let annotations = if policy.annotations.is_empty() {
        None
    } else {
        Some(
            policy
                .annotations
                .iter()
                .map(|(name, value)| ((*name).to_owned(), value.map(str::to_owned)))
                .collect(),
        )
    };

    json::PolicyJson {
        effect,
        principal: scope_to_json(&policy.principal),
        action: action_to_json(&policy.action),
        resource: scope_to_json(&policy.resource),
        conditions: policy.conditions.iter().map(condition_to_json).collect(),
        annotations,
    }
}

fn scope_json(op: &str) -> json::ScopeJson {
    json::ScopeJson {
        op: String::from(op),
        entity: None,
        entities: None,
        slot: None,
        entity_type: None,
        in_entity: None,
    }
}

fn scope_to_json(constraint: &est::PrincipalOrResourceConstraint<'_>) -> json::ScopeJson {
    match constraint {
        est::PrincipalOrResourceConstraint::Any => scope_json("All"),
        est::PrincipalOrResourceConstraint::Equal(expression) => {
            let (entity, slot) = entity_or_slot(expression);
            json::ScopeJson {
                op: String::from("=="),
                entity,
                slot,
                ..scope_json("==")
            }
        }
        est::PrincipalOrResourceConstraint::In(expression) => {
            let (entity, slot) = entity_or_slot(expression);
            json::ScopeJson {
                op: String::from("in"),
                entity,
                slot,
                ..scope_json("in")
            }
        }
        est::PrincipalOrResourceConstraint::Is { entity_type } => json::ScopeJson {
            entity_type: Some((*entity_type).to_owned()),
            ..scope_json("is")
        },
        est::PrincipalOrResourceConstraint::IsIn {
            entity_type,
            in_entity,
        } => json::ScopeJson {
            entity_type: Some((*entity_type).to_owned()),
            in_entity: Some(json::ScopeInJson {
                entity: expression_to_entity(in_entity),
            }),
            ..scope_json("is")
        },
    }
}

fn action_to_json(constraint: &est::ActionConstraint<'_>) -> json::ScopeJson {
    match constraint {
        est::ActionConstraint::Any => scope_json("All"),
        est::ActionConstraint::Equal(expression) => json::ScopeJson {
            entity: Some(expression_to_entity(expression)),
            ..scope_json("==")
        },
        est::ActionConstraint::In(expressions) if expressions.len() == 1 => json::ScopeJson {
            entity: expressions.first().map(expression_to_entity),
            ..scope_json("in")
        },
        est::ActionConstraint::In(expressions) => json::ScopeJson {
            entities: Some(expressions.iter().map(expression_to_entity).collect()),
            ..scope_json("in")
        },
    }
}

fn entity_or_slot(
    expression: &est::Expression<'_>,
) -> (Option<json::EntityUidJson>, Option<String>) {
    match expression {
        est::Expression::Slot(est::SlotId::Principal) => (None, Some(String::from("?principal"))),
        est::Expression::Slot(est::SlotId::Resource) => (None, Some(String::from("?resource"))),
        _ => (Some(expression_to_entity(expression)), None),
    }
}

fn expression_to_entity(expression: &est::Expression<'_>) -> json::EntityUidJson {
    match expression {
        est::Expression::Entity { entity_type, id } => json::EntityUidJson {
            entity_type: (*entity_type).to_owned(),
            id: (*id).to_owned(),
        },
        _ => json::EntityUidJson {
            entity_type: String::from("Unknown"),
            id: String::from("unknown"),
        },
    }
}

fn condition_to_json(condition: &est::Condition<'_>) -> json::ConditionJson {
    json::ConditionJson {
        kind: match condition.kind {
            est::ConditionKind::When => String::from("when"),
            est::ConditionKind::Unless => String::from("unless"),
        },
        body: expression_to_json(&condition.body),
    }
}

fn expression_to_json(expression: &est::Expression<'_>) -> json::ExpressionJson {
    match expression {
        est::Expression::Boolean(value) => json::ExpressionJson::Value(json::ExpressionValueJson {
            value: json::ValueJson::Bool(*value),
        }),
        est::Expression::Integer(value) => json::ExpressionJson::Value(json::ExpressionValueJson {
            value: json::ValueJson::Int(*value),
        }),
        est::Expression::String(value) => json::ExpressionJson::Value(json::ExpressionValueJson {
            value: json::ValueJson::String((*value).to_owned()),
        }),
        est::Expression::Variable(variable) => {
            json::ExpressionJson::Variable(json::ExpressionVariableJson {
                var: match variable {
                    est::Variable::Principal => String::from("principal"),
                    est::Variable::Action => String::from("action"),
                    est::Variable::Resource => String::from("resource"),
                    est::Variable::Context => String::from("context"),
                },
            })
        }
        est::Expression::Slot(slot) => json::ExpressionJson::Slot(json::ExpressionSlotJson {
            slot: match slot {
                est::SlotId::Principal => String::from("?principal"),
                est::SlotId::Resource => String::from("?resource"),
            },
        }),
        est::Expression::Entity { entity_type, id } => {
            json::ExpressionJson::Value(json::ExpressionValueJson {
                value: json::ValueJson::Entity(json::EntityValueJson {
                    entity: json::EntityUidJson {
                        entity_type: (*entity_type).to_owned(),
                        id: (*id).to_owned(),
                    },
                }),
            })
        }
        est::Expression::Set(elements) => json::ExpressionJson::Set(json::ExpressionSetJson {
            set: elements.iter().map(expression_to_json).collect(),
        }),
        est::Expression::Record(entries) => {
            json::ExpressionJson::Record(json::ExpressionRecordJson {
                record: entries
                    .iter()
                    .map(|(key, value)| ((*key).to_owned(), expression_to_json(value)))
                    .collect(),
            })
        }
        est::Expression::Not(inner) => json::ExpressionJson::Not(json::ExpressionNotJson {
            not: json::UnaryArgumentJson {
                arg: Box::new(expression_to_json(inner)),
            },
        }),
        est::Expression::Negate(inner) => {
            json::ExpressionJson::Negate(json::ExpressionNegateJson {
                neg: json::UnaryArgumentJson {
                    arg: Box::new(expression_to_json(inner)),
                },
            })
        }
        est::Expression::Or(left, right) => json::ExpressionJson::Or(json::ExpressionOrJson {
            or: binary_arguments(left, right),
        }),
        est::Expression::And(left, right) => json::ExpressionJson::And(json::ExpressionAndJson {
            and: binary_arguments(left, right),
        }),
        est::Expression::Equal(left, right) => {
            json::ExpressionJson::Equal(json::ExpressionEqualJson {
                eq: binary_arguments(left, right),
            })
        }
        est::Expression::NotEqual(left, right) => {
            json::ExpressionJson::NotEqual(json::ExpressionNotEqualJson {
                neq: binary_arguments(left, right),
            })
        }
        est::Expression::LessThan(left, right) => {
            json::ExpressionJson::LessThan(json::ExpressionLessThanJson {
                lt: binary_arguments(left, right),
            })
        }
        est::Expression::LessThanOrEqual(left, right) => {
            json::ExpressionJson::LessThanOrEqual(json::ExpressionLessThanOrEqualJson {
                lte: binary_arguments(left, right),
            })
        }
        est::Expression::GreaterThan(left, right) => {
            json::ExpressionJson::GreaterThan(json::ExpressionGreaterThanJson {
                gt: binary_arguments(left, right),
            })
        }
        est::Expression::GreaterThanOrEqual(left, right) => {
            json::ExpressionJson::GreaterThanOrEqual(json::ExpressionGreaterThanOrEqualJson {
                gte: binary_arguments(left, right),
            })
        }
        est::Expression::In(left, right) => json::ExpressionJson::In(json::ExpressionInJson {
            in_op: binary_arguments(left, right),
        }),
        est::Expression::Add(left, right) => json::ExpressionJson::Add(json::ExpressionAddJson {
            add: binary_arguments(left, right),
        }),
        est::Expression::Subtract(left, right) => {
            json::ExpressionJson::Subtract(json::ExpressionSubtractJson {
                sub: binary_arguments(left, right),
            })
        }
        est::Expression::Multiply(left, right) => {
            json::ExpressionJson::Multiply(json::ExpressionMultiplyJson {
                mul: binary_arguments(left, right),
            })
        }
        est::Expression::GetAttribute {
            expression,
            attribute,
        } => json::ExpressionJson::GetAttribute(json::ExpressionGetAttributeJson {
            get_attr: json::AttributeArgumentJson {
                left: Box::new(expression_to_json(expression)),
                attr: (*attribute).to_owned(),
            },
        }),
        est::Expression::HasAttribute {
            expression,
            attribute,
        } => json::ExpressionJson::HasAttribute(json::ExpressionHasAttributeJson {
            has: json::AttributeArgumentJson {
                left: Box::new(expression_to_json(expression)),
                attr: (*attribute).to_owned(),
            },
        }),
        est::Expression::Index { expression, index } => {
            json::ExpressionJson::Index(json::ExpressionIndexJson {
                index: json::IndexArgumentJson {
                    left: Box::new(expression_to_json(expression)),
                    index: Box::new(expression_to_json(index)),
                },
            })
        }
        est::Expression::Like {
            expression,
            pattern,
        } => json::ExpressionJson::Like(json::ExpressionLikeJson {
            like: json::LikeArgumentJson {
                left: Box::new(expression_to_json(expression)),
                pattern: pattern.iter().map(pattern_element_to_json).collect(),
            },
        }),
        est::Expression::Is {
            expression,
            entity_type,
            in_expression,
        } => json::ExpressionJson::Is(json::ExpressionIsJson {
            is: json::IsArgumentJson {
                left: Box::new(expression_to_json(expression)),
                entity_type: (*entity_type).to_owned(),
                in_expr: in_expression
                    .as_ref()
                    .map(|inner| Box::new(expression_to_json(inner))),
            },
        }),
        est::Expression::If {
            condition,
            then_expression,
            else_expression,
        } => json::ExpressionJson::IfThenElse(json::ExpressionIfJson {
            if_then_else: json::IfArgumentJson {
                cond: Box::new(expression_to_json(condition)),
                then_expr: Box::new(expression_to_json(then_expression)),
                else_expr: Box::new(expression_to_json(else_expression)),
            },
        }),
        est::Expression::MethodCall {
            receiver,
            method,
            arguments,
        } => method_call_to_json(receiver, method, arguments),
        est::Expression::ExtensionCall { name, arguments } => {
            let mut map = BTreeMap::new();
            map.insert(
                (*name).to_owned(),
                arguments.iter().map(expression_to_json).collect(),
            );
            json::ExpressionJson::ExtensionFunction(json::ExpressionExtensionFunctionJson(map))
        }
    }
}

fn pattern_element_to_json(element: &est::PatternElement<'_>) -> json::PatternElementJson {
    match element {
        est::PatternElement::Literal(literal) => {
            json::PatternElementJson::Literal(json::PatternLiteralJson {
                literal: (*literal).to_owned(),
            })
        }
        est::PatternElement::Wildcard => {
            json::PatternElementJson::Wildcard(String::from("Wildcard"))
        }
    }
}

fn binary_arguments(
    left: &est::Expression<'_>,
    right: &est::Expression<'_>,
) -> json::BinaryArgumentJson {
    json::BinaryArgumentJson {
        left: Box::new(expression_to_json(left)),
        right: Box::new(expression_to_json(right)),
    }
}

fn method_binary_arguments(
    receiver: &est::Expression<'_>,
    arguments: &[est::Expression<'_>],
) -> json::BinaryArgumentJson {
    json::BinaryArgumentJson {
        left: Box::new(expression_to_json(receiver)),
        right: arguments.first().map_or_else(
            || {
                Box::new(json::ExpressionJson::Value(json::ExpressionValueJson {
                    value: json::ValueJson::Bool(false),
                }))
            },
            |argument| Box::new(expression_to_json(argument)),
        ),
    }
}

fn method_call_to_json(
    receiver: &est::Expression<'_>,
    method: &str,
    arguments: &[est::Expression<'_>],
) -> json::ExpressionJson {
    match method {
        "contains" => json::ExpressionJson::Contains(json::ExpressionContainsJson {
            contains: method_binary_arguments(receiver, arguments),
        }),
        "containsAll" => json::ExpressionJson::ContainsAll(json::ExpressionContainsAllJson {
            contains_all: method_binary_arguments(receiver, arguments),
        }),
        "containsAny" => json::ExpressionJson::ContainsAny(json::ExpressionContainsAnyJson {
            contains_any: method_binary_arguments(receiver, arguments),
        }),
        "hasTag" => json::ExpressionJson::HasTag(json::ExpressionHasTagJson {
            has_tag: method_binary_arguments(receiver, arguments),
        }),
        "getTag" => json::ExpressionJson::GetTag(json::ExpressionGetTagJson {
            get_tag: method_binary_arguments(receiver, arguments),
        }),
        "isEmpty" => json::ExpressionJson::IsEmpty(json::ExpressionIsEmptyJson {
            is_empty: json::UnaryArgumentJson {
                arg: Box::new(expression_to_json(receiver)),
            },
        }),
        _ => {
            let mut all_arguments = vec![expression_to_json(receiver)];
            all_arguments.extend(arguments.iter().map(expression_to_json));

            let mut map = BTreeMap::new();
            map.insert(String::from(method), all_arguments);

            json::ExpressionJson::ExtensionMethod(json::ExpressionExtensionMethodJson(map))
        }
    }
}
