use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;

use super::types as json;
use crate::policy::est::types as est;

pub fn policies_to_json(policies: &[est::Policy]) -> json::PolicySetJson {
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
        template_links: alloc::vec![],
    }
}

fn policy_to_json(policy: &est::Policy) -> json::PolicyJson {
    let effect = match policy.effect {
        est::Effect::Permit => String::from("permit"),
        est::Effect::Forbid => String::from("forbid"),
    };

    let annotations = if policy.annotations.is_empty() {
        None
    } else {
        Some(policy.annotations.iter().cloned().collect())
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

fn scope_to_json(constraint: &est::PrincipalOrResourceConstraint) -> json::ScopeJson {
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
            entity_type: Some(entity_type.clone()),
            ..scope_json("is")
        },
        est::PrincipalOrResourceConstraint::IsIn {
            entity_type,
            in_entity,
        } => json::ScopeJson {
            entity_type: Some(entity_type.clone()),
            in_entity: Some(json::ScopeInJson {
                entity: expression_to_entity(in_entity),
            }),
            ..scope_json("is")
        },
    }
}

fn action_to_json(constraint: &est::ActionConstraint) -> json::ScopeJson {
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

fn entity_or_slot(expression: &est::Expression) -> (Option<json::EntityUidJson>, Option<String>) {
    match expression {
        est::Expression::Slot(est::SlotId::Principal) => (None, Some(String::from("?principal"))),
        est::Expression::Slot(est::SlotId::Resource) => (None, Some(String::from("?resource"))),
        _ => (Some(expression_to_entity(expression)), None),
    }
}

fn expression_to_entity(expression: &est::Expression) -> json::EntityUidJson {
    match expression {
        est::Expression::Entity { entity_type, id } => json::EntityUidJson {
            entity_type: entity_type.clone(),
            id: id.clone(),
        },
        _ => json::EntityUidJson {
            entity_type: String::from("Unknown"),
            id: String::from("unknown"),
        },
    }
}

fn condition_to_json(condition: &est::Condition) -> json::ConditionJson {
    json::ConditionJson {
        kind: match condition.kind {
            est::ConditionKind::When => String::from("when"),
            est::ConditionKind::Unless => String::from("unless"),
        },
        body: expression_to_json(&condition.body),
    }
}

macro_rules! unary_expression {
    ($enum_variant:ident, $struct:ident, $field:ident, $inner:expr) => {
        json::ExpressionJson::$enum_variant(json::$struct {
            $field: json::UnaryArgumentJson {
                arg: Box::new(expression_to_json($inner)),
            },
        })
    };
}

macro_rules! binary_expression {
    ($enum_variant:ident, $struct:ident, $field:ident, $left:expr, $right:expr) => {
        json::ExpressionJson::$enum_variant(json::$struct {
            $field: binary_arguments($left, $right),
        })
    };
}

macro_rules! attribute_expression {
    ($enum_variant:ident, $struct:ident, $field:ident, $expression:expr, $attribute:expr) => {
        json::ExpressionJson::$enum_variant(json::$struct {
            $field: json::AttributeArgumentJson {
                left: Box::new(expression_to_json($expression)),
                attr: $attribute.clone(),
            },
        })
    };
}

fn expression_to_json(expression: &est::Expression) -> json::ExpressionJson {
    match expression {
        est::Expression::Boolean(value) => json::ExpressionJson::Value(json::ExpressionValueJson {
            value: json::ValueJson::Bool(*value),
        }),
        est::Expression::Integer(value) => json::ExpressionJson::Value(json::ExpressionValueJson {
            value: json::ValueJson::Int(*value),
        }),
        est::Expression::String(value) => json::ExpressionJson::Value(json::ExpressionValueJson {
            value: json::ValueJson::String(value.clone()),
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
                        entity_type: entity_type.clone(),
                        id: id.clone(),
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
                    .map(|(key, value)| (key.clone(), expression_to_json(value)))
                    .collect(),
            })
        }
        est::Expression::Not(inner) => unary_expression!(Not, ExpressionNotJson, not, inner),
        est::Expression::Negate(inner) => {
            unary_expression!(Negate, ExpressionNegateJson, neg, inner)
        }
        est::Expression::Or(left, right) => {
            binary_expression!(Or, ExpressionOrJson, or, left, right)
        }
        est::Expression::And(left, right) => {
            binary_expression!(And, ExpressionAndJson, and, left, right)
        }
        est::Expression::Equal(left, right) => {
            binary_expression!(Equal, ExpressionEqualJson, eq, left, right)
        }
        est::Expression::NotEqual(left, right) => {
            binary_expression!(NotEqual, ExpressionNotEqualJson, neq, left, right)
        }
        est::Expression::LessThan(left, right) => {
            binary_expression!(LessThan, ExpressionLessThanJson, lt, left, right)
        }
        est::Expression::LessThanOrEqual(left, right) => binary_expression!(
            LessThanOrEqual,
            ExpressionLessThanOrEqualJson,
            lte,
            left,
            right
        ),
        est::Expression::GreaterThan(left, right) => {
            binary_expression!(GreaterThan, ExpressionGreaterThanJson, gt, left, right)
        }
        est::Expression::GreaterThanOrEqual(left, right) => binary_expression!(
            GreaterThanOrEqual,
            ExpressionGreaterThanOrEqualJson,
            gte,
            left,
            right
        ),
        est::Expression::In(left, right) => {
            binary_expression!(In, ExpressionInJson, in_op, left, right)
        }
        est::Expression::Add(left, right) => {
            binary_expression!(Add, ExpressionAddJson, add, left, right)
        }
        est::Expression::Subtract(left, right) => {
            binary_expression!(Subtract, ExpressionSubtractJson, sub, left, right)
        }
        est::Expression::Multiply(left, right) => {
            binary_expression!(Multiply, ExpressionMultiplyJson, mul, left, right)
        }
        est::Expression::GetAttribute {
            expression,
            attribute,
        } => attribute_expression!(
            GetAttribute,
            ExpressionGetAttributeJson,
            get_attr,
            expression,
            attribute
        ),
        est::Expression::HasAttribute {
            expression,
            attribute,
        } => attribute_expression!(
            HasAttribute,
            ExpressionHasAttributeJson,
            has,
            expression,
            attribute
        ),
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
                entity_type: entity_type.clone(),
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
                name.clone(),
                arguments.iter().map(expression_to_json).collect(),
            );
            json::ExpressionJson::ExtensionFunction(json::ExpressionExtensionFunctionJson(map))
        }
    }
}

fn pattern_element_to_json(element: &est::PatternElement) -> json::PatternElementJson {
    match element {
        est::PatternElement::Literal(literal) => {
            json::PatternElementJson::Literal(json::PatternLiteralJson {
                literal: literal.clone(),
            })
        }
        est::PatternElement::Wildcard => {
            json::PatternElementJson::Wildcard(String::from("Wildcard"))
        }
    }
}

fn binary_arguments(left: &est::Expression, right: &est::Expression) -> json::BinaryArgumentJson {
    json::BinaryArgumentJson {
        left: Box::new(expression_to_json(left)),
        right: Box::new(expression_to_json(right)),
    }
}

fn method_binary_arguments(
    receiver: &est::Expression,
    arguments: &[est::Expression],
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

macro_rules! method_binary_expression {
    ($enum_variant:ident, $struct:ident, $field:ident, $receiver:expr, $arguments:expr) => {
        json::ExpressionJson::$enum_variant(json::$struct {
            $field: method_binary_arguments($receiver, $arguments),
        })
    };
}

fn method_call_to_json(
    receiver: &est::Expression,
    method: &str,
    arguments: &[est::Expression],
) -> json::ExpressionJson {
    match method {
        "contains" => method_binary_expression!(
            Contains,
            ExpressionContainsJson,
            contains,
            receiver,
            arguments
        ),
        "containsAll" => method_binary_expression!(
            ContainsAll,
            ExpressionContainsAllJson,
            contains_all,
            receiver,
            arguments
        ),
        "containsAny" => method_binary_expression!(
            ContainsAny,
            ExpressionContainsAnyJson,
            contains_any,
            receiver,
            arguments
        ),
        "hasTag" => {
            method_binary_expression!(HasTag, ExpressionHasTagJson, has_tag, receiver, arguments)
        }
        "getTag" => {
            method_binary_expression!(GetTag, ExpressionGetTagJson, get_tag, receiver, arguments)
        }
        "isEmpty" => unary_expression!(IsEmpty, ExpressionIsEmptyJson, is_empty, receiver),
        _ => {
            let mut all_arguments = alloc::vec![expression_to_json(receiver)];
            all_arguments.extend(arguments.iter().map(expression_to_json));
            let mut map = BTreeMap::new();
            map.insert(String::from(method), all_arguments);
            json::ExpressionJson::ExtensionMethod(json::ExpressionExtensionMethodJson(map))
        }
    }
}
