use alloc::string::String;
use alloc::vec::Vec;

use bumpalo::Bump;
use bumpalo::collections::Vec as BumpVec;

use super::error::ConvertError;
use super::types as est;
use crate::escape::LazyEscape;
use crate::policy::ast::{self, AstToken as _, BinaryOperator, UnaryOperator};

pub fn convert_policies<'a>(
    bump: &'a Bump,
    policies: &ast::Policies<'_>,
    source: &'a str,
) -> Result<BumpVec<'a, est::Policy<'a>>, ConvertError> {
    let mut result = BumpVec::new_in(bump);
    for policy in policies.policies() {
        result.push(convert_policy(bump, &policy, source)?);
    }

    Ok(result)
}

fn convert_policy<'a>(
    bump: &'a Bump,
    policy: &ast::Policy<'_>,
    source: &'a str,
) -> Result<est::Policy<'a>, ConvertError> {
    let effect = match policy.effect() {
        Some(ast::Effect::Permit) => est::Effect::Permit,
        Some(ast::Effect::Forbid) => est::Effect::Forbid,
        None => return Err(ConvertError::MissingNode("effect")),
    };

    let mut principal = est::PrincipalOrResourceConstraint::Any;
    let mut action = est::ActionConstraint::Any;
    let mut resource = est::PrincipalOrResourceConstraint::Any;

    for variable_def in policy.variables() {
        let variable = variable_def.variable();
        match variable {
            Some(ast::Variable::Principal) => {
                principal = convert_principal_or_resource_constraint(bump, &variable_def, source)?;
            }
            Some(ast::Variable::Action) => {
                action = convert_action_constraint(bump, &variable_def, source)?;
            }
            Some(ast::Variable::Resource) => {
                resource = convert_principal_or_resource_constraint(bump, &variable_def, source)?;
            }
            Some(ast::Variable::Context) | None => {}
        }
    }

    let mut conditions = BumpVec::new_in(bump);
    for condition in policy.conditions() {
        let kind = match condition.kind() {
            Some(ast::ConditionKind::When) => est::ConditionKind::When,
            Some(ast::ConditionKind::Unless) => est::ConditionKind::Unless,
            None => continue,
        };

        let Some(expression) = condition.expr() else {
            continue;
        };

        let body = convert_expression(bump, &expression, source)?;
        conditions.push(est::Condition { kind, body });
    }

    let mut annotations = BumpVec::new_in(bump);
    for annotation in policy.annotations() {
        let Some(name_token) = annotation.name() else {
            continue;
        };

        let name: &str = bump.alloc_str(name_token.text(source));
        let value = annotation
            .value()
            .map(|value| LazyEscape::new(value.text(source)));

        annotations.push((name, value));
    }

    Ok(est::Policy {
        effect,
        principal,
        action,
        resource,
        conditions,
        annotations,
    })
}

fn convert_principal_or_resource_constraint<'a>(
    bump: &'a Bump,
    variable_def: &ast::VariableDefinition<'_>,
    source: &'a str,
) -> Result<est::PrincipalOrResourceConstraint<'a>, ConvertError> {
    if variable_def.has_is_constraint() {
        let entity_type_name = variable_def
            .is_type_name()
            .ok_or(ConvertError::MissingNode("is entity type"))?;

        let entity_type = build_type_name(bump, &entity_type_name, source);

        if variable_def.has_is_in_constraint() {
            let in_entity = variable_def
                .is_in_entity()
                .ok_or(ConvertError::MissingNode("is in entity"))?;

            let in_entity_expression = convert_expression(bump, &in_entity, source)?;
            return Ok(est::PrincipalOrResourceConstraint::IsIn {
                entity_type,
                in_entity: in_entity_expression,
            });
        }

        return Ok(est::PrincipalOrResourceConstraint::Is { entity_type });
    }

    let Some(constraint) = variable_def.constraint() else {
        return Ok(est::PrincipalOrResourceConstraint::Any);
    };

    if let ast::Expression::Is(is_expression) = constraint {
        return convert_is_constraint(bump, &is_expression, source);
    }

    let Some(relation) = variable_def.relation() else {
        return Ok(est::PrincipalOrResourceConstraint::Any);
    };

    let entity_expression = convert_expression(bump, &constraint, source)?;

    match relation {
        BinaryOperator::Eq => Ok(est::PrincipalOrResourceConstraint::Equal(entity_expression)),
        BinaryOperator::In => Ok(est::PrincipalOrResourceConstraint::In(entity_expression)),
        _ => Ok(est::PrincipalOrResourceConstraint::Any),
    }
}

fn convert_is_constraint<'a>(
    bump: &'a Bump,
    is_expression: &ast::IsExpression<'_>,
    source: &'a str,
) -> Result<est::PrincipalOrResourceConstraint<'a>, ConvertError> {
    let entity_type_name = is_expression
        .entity_type()
        .ok_or(ConvertError::MissingNode("is entity type"))?;

    let entity_type = build_type_name(bump, &entity_type_name, source);

    if let Some(in_entity) = is_expression.in_entity() {
        let in_entity_expression = convert_expression(bump, &in_entity, source)?;
        return Ok(est::PrincipalOrResourceConstraint::IsIn {
            entity_type,
            in_entity: in_entity_expression,
        });
    }

    Ok(est::PrincipalOrResourceConstraint::Is { entity_type })
}

fn convert_action_constraint<'a>(
    bump: &'a Bump,
    variable_def: &ast::VariableDefinition<'_>,
    source: &'a str,
) -> Result<est::ActionConstraint<'a>, ConvertError> {
    let Some(constraint) = variable_def.constraint() else {
        return Ok(est::ActionConstraint::Any);
    };

    let Some(relation) = variable_def.relation() else {
        return Ok(est::ActionConstraint::Any);
    };

    match relation {
        BinaryOperator::Eq => {
            let entity_expression = convert_expression(bump, &constraint, source)?;
            Ok(est::ActionConstraint::Equal(entity_expression))
        }
        BinaryOperator::In => {
            if let ast::Expression::List(list_expression) = constraint {
                let mut entities = BumpVec::new_in(bump);
                for element in list_expression.elements() {
                    entities.push(convert_expression(bump, &element, source)?);
                }

                Ok(est::ActionConstraint::In(entities))
            } else {
                let entity_expression = convert_expression(bump, &constraint, source)?;

                let mut entities = BumpVec::new_in(bump);
                entities.push(entity_expression);
                Ok(est::ActionConstraint::In(entities))
            }
        }
        _ => Ok(est::ActionConstraint::Any),
    }
}

fn build_type_name<'a>(bump: &'a Bump, name: &ast::Name<'_>, source: &'a str) -> &'a str {
    let segments: Vec<&str> = name.segments().map(|seg| seg.text(source)).collect();
    if let [single] = segments.as_slice() {
        return bump.alloc_str(single);
    }

    let joined: String = segments.join("::");
    bump.alloc_str(&joined)
}

fn convert_expression<'a>(
    bump: &'a Bump,
    expression: &ast::Expression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    match expression {
        ast::Expression::Literal(literal) => convert_literal(literal, source),
        ast::Expression::EntityRef(entity) => convert_entity_ref(bump, entity, source),
        ast::Expression::Slot(slot) => convert_slot(slot, source),
        ast::Expression::Paren(paren) => {
            let inner = paren
                .inner()
                .ok_or(ConvertError::MissingNode("paren inner"))?;

            convert_expression(bump, &inner, source)
        }
        ast::Expression::List(list) => {
            let mut elements = BumpVec::new_in(bump);
            for element in list.elements() {
                elements.push(convert_expression(bump, &element, source)?);
            }

            Ok(est::Expression::Set(elements))
        }
        ast::Expression::Record(record) => {
            let mut entries = BumpVec::new_in(bump);
            for entry in record.entries() {
                let key = entry
                    .key()
                    .map(|key| extract_attr_key(bump, key, source))
                    .ok_or(ConvertError::MissingNode("record key"))?;

                let value = entry
                    .value()
                    .ok_or(ConvertError::MissingNode("record value"))?;

                let value_expression = convert_expression(bump, &value, source)?;
                entries.push((key, value_expression));
            }
            Ok(est::Expression::Record(entries))
        }
        ast::Expression::Unary(unary) => convert_unary(bump, unary, source),
        ast::Expression::Binary(binary) => convert_binary(bump, binary, source),
        ast::Expression::Has(has) => convert_has(bump, has, source),
        ast::Expression::Like(like) => convert_like(bump, like, source),
        ast::Expression::Is(is_expression) => convert_is(bump, is_expression, source),
        ast::Expression::If(if_expression) => convert_if(bump, if_expression, source),
        ast::Expression::Field(field) => convert_field(bump, field, source),
        ast::Expression::MethodCall(method) => convert_method_call(bump, method, source),
        ast::Expression::Index(index) => convert_index(bump, index, source),
        ast::Expression::FunctionCall(func) => convert_function_call(bump, func, source),
        ast::Expression::Path(path) => convert_path(bump, path, source),
        ast::Expression::Unknown(_) => Err(ConvertError::MissingNode("unknown expression")),
    }
}

fn convert_literal<'a>(
    literal: &ast::LiteralExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    if let Some(bool_val) = literal.as_bool() {
        return Ok(est::Expression::Boolean(bool_val));
    }

    if let Some(int_token) = literal.as_integer() {
        let text = int_token.text(source);
        let value = parse_integer(text)?;
        return Ok(est::Expression::Integer(value));
    }

    if let Some(str_token) = literal.as_string() {
        let text = str_token.text(source);
        return Ok(est::Expression::String(LazyEscape::new(text)));
    }

    Err(ConvertError::MissingNode("literal value"))
}

fn convert_entity_ref<'a>(
    bump: &'a Bump,
    entity: &ast::EntityRefExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let entity_type = entity
        .entity_type()
        .ok_or(ConvertError::MissingNode("entity type"))?;

    let id = entity
        .entity_id()
        .ok_or(ConvertError::MissingNode("entity id"))?;

    let type_str = build_type_name(bump, &entity_type, source);

    Ok(est::Expression::Entity {
        entity_type: type_str,
        id: LazyEscape::new(id.text(source)),
    })
}

fn convert_slot(
    slot: &ast::SlotExpression<'_>,
    source: &str,
) -> Result<est::Expression<'static>, ConvertError> {
    let slot_kind = slot
        .slot_kind(source)
        .ok_or(ConvertError::MissingNode("slot kind"))?;

    let slot_id = match slot_kind {
        ast::SlotKind::Principal => est::SlotId::Principal,
        ast::SlotKind::Resource => est::SlotId::Resource,
    };

    Ok(est::Expression::Slot(slot_id))
}

fn convert_unary<'a>(
    bump: &'a Bump,
    unary: &ast::UnaryExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let operator = unary
        .operator()
        .ok_or(ConvertError::MissingNode("unary operator"))?;

    let operand = unary
        .operand()
        .ok_or(ConvertError::MissingNode("unary operand"))?;

    match operator {
        UnaryOperator::Not => {
            let operand_expression = convert_expression(bump, &operand, source)?;
            Ok(est::Expression::Not(bump.alloc(operand_expression)))
        }
        UnaryOperator::Neg => {
            if let ast::Expression::Literal(literal) = &operand
                && let Some(int_token) = literal.as_integer()
            {
                let text = int_token.text(source);
                if let Ok(value) = parse_integer(text)
                    && value >= 0
                    && let Some(negated) = value.checked_neg()
                {
                    return Ok(est::Expression::Integer(negated));
                }

                if text.parse::<u64>() == Ok(i64::MIN.unsigned_abs()) {
                    return Ok(est::Expression::Integer(i64::MIN));
                }
            }

            let operand_expression = convert_expression(bump, &operand, source)?;
            Ok(est::Expression::Negate(bump.alloc(operand_expression)))
        }
    }
}

fn convert_binary<'a>(
    bump: &'a Bump,
    binary: &ast::BinaryExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let operator = binary
        .operator()
        .ok_or(ConvertError::MissingNode("binary operator"))?;

    let left = binary
        .left()
        .ok_or(ConvertError::MissingNode("binary left"))?;

    let right = binary
        .right()
        .ok_or(ConvertError::MissingNode("binary right"))?;

    let left_expression = bump.alloc(convert_expression(bump, &left, source)?);
    let right_expression = bump.alloc(convert_expression(bump, &right, source)?);

    let expression = match operator {
        BinaryOperator::Or => est::Expression::Or(left_expression, right_expression),
        BinaryOperator::And => est::Expression::And(left_expression, right_expression),
        BinaryOperator::Eq => est::Expression::Equal(left_expression, right_expression),
        BinaryOperator::Neq => est::Expression::NotEqual(left_expression, right_expression),
        BinaryOperator::Lt => est::Expression::LessThan(left_expression, right_expression),
        BinaryOperator::Lte => est::Expression::LessThanOrEqual(left_expression, right_expression),
        BinaryOperator::Gt => est::Expression::GreaterThan(left_expression, right_expression),
        BinaryOperator::Gte => {
            est::Expression::GreaterThanOrEqual(left_expression, right_expression)
        }
        BinaryOperator::In => est::Expression::In(left_expression, right_expression),
        BinaryOperator::Add => est::Expression::Add(left_expression, right_expression),
        BinaryOperator::Sub => est::Expression::Subtract(left_expression, right_expression),
        BinaryOperator::Mul => est::Expression::Multiply(left_expression, right_expression),
    };

    Ok(expression)
}

fn convert_has<'a>(
    bump: &'a Bump,
    has: &ast::HasExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let target = has
        .target()
        .ok_or(ConvertError::MissingNode("has target"))?;

    let target_expression = convert_expression(bump, &target, source)?;

    let attributes: Vec<&str> = has.attributes().map(|attr| attr.text(source)).collect();
    if attributes.is_empty() {
        let attr_key = has
            .attribute()
            .ok_or(ConvertError::MissingNode("has attribute"))?;

        let attribute = extract_attr_key(bump, attr_key, source);
        return Ok(est::Expression::HasAttribute {
            expression: bump.alloc(target_expression),
            attribute,
        });
    }

    if attributes.len() == 1 {
        let Some(attribute) = attributes.into_iter().next() else {
            return Err(ConvertError::MissingNode("has attribute"));
        };

        return Ok(est::Expression::HasAttribute {
            expression: bump.alloc(target_expression),
            attribute: bump.alloc_str(attribute),
        });
    }

    let mut result: Option<est::Expression<'a>> = None;
    let mut current_path = target_expression;

    for attribute in attributes {
        let attribute = bump.alloc_str(attribute);
        let current_path_boxed = bump.alloc(current_path);

        let has_expr = est::Expression::HasAttribute {
            expression: current_path_boxed,
            attribute,
        };

        result = Some(match result {
            Some(prev) => est::Expression::And(bump.alloc(prev), bump.alloc(has_expr)),
            None => has_expr,
        });

        current_path = est::Expression::GetAttribute {
            expression: current_path_boxed,
            attribute,
        };
    }

    result.ok_or(ConvertError::MissingNode("has attribute"))
}

fn convert_like<'a>(
    bump: &'a Bump,
    like: &ast::LikeExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let target = like
        .target()
        .ok_or(ConvertError::MissingNode("like target"))?;

    let pattern_token = like
        .pattern()
        .ok_or(ConvertError::MissingNode("like pattern"))?;

    let target_expression = convert_expression(bump, &target, source)?;
    let pattern_text = pattern_token.text(source);
    let pattern = parse_pattern(bump, pattern_text);

    Ok(est::Expression::Like {
        expression: bump.alloc(target_expression),
        pattern,
    })
}

fn convert_is<'a>(
    bump: &'a Bump,
    is_expr: &ast::IsExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let target = is_expr
        .target()
        .ok_or(ConvertError::MissingNode("is target"))?;

    let entity_type_name = is_expr
        .entity_type()
        .ok_or(ConvertError::MissingNode("is entity type"))?;

    let target_expression = convert_expression(bump, &target, source)?;
    let entity_type = build_type_name(bump, &entity_type_name, source);

    let in_expression: Option<&est::Expression<'_>> = if let Some(in_entity) = is_expr.in_entity() {
        Some(bump.alloc(convert_expression(bump, &in_entity, source)?))
    } else {
        None
    };

    Ok(est::Expression::Is {
        expression: bump.alloc(target_expression),
        entity_type,
        in_expression,
    })
}

fn convert_if<'a>(
    bump: &'a Bump,
    if_expression: &ast::IfExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let cond = if_expression
        .condition()
        .ok_or(ConvertError::MissingNode("if condition"))?;

    let then_branch = if_expression
        .then_branch()
        .ok_or(ConvertError::MissingNode("if then"))?;

    let else_branch = if_expression
        .else_branch()
        .ok_or(ConvertError::MissingNode("if else"))?;

    Ok(est::Expression::If {
        condition: bump.alloc(convert_expression(bump, &cond, source)?),
        then_expression: bump.alloc(convert_expression(bump, &then_branch, source)?),
        else_expression: bump.alloc(convert_expression(bump, &else_branch, source)?),
    })
}

fn convert_field<'a>(
    bump: &'a Bump,
    field: &ast::FieldExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let receiver = field
        .receiver()
        .ok_or(ConvertError::MissingNode("field receiver"))?;

    let field_name = field
        .field()
        .ok_or(ConvertError::MissingNode("field name"))?;

    let receiver_expression = convert_expression(bump, &receiver, source)?;
    let attribute = bump.alloc_str(field_name.text(source));

    Ok(est::Expression::GetAttribute {
        expression: bump.alloc(receiver_expression),
        attribute,
    })
}

fn convert_method_call<'a>(
    bump: &'a Bump,
    method_call: &ast::MethodCallExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let receiver = method_call
        .receiver()
        .ok_or(ConvertError::MissingNode("method receiver"))?;

    let method_name = method_call
        .method()
        .ok_or(ConvertError::MissingNode("method name"))?;

    let receiver_expression = convert_expression(bump, &receiver, source)?;
    let method = bump.alloc_str(method_name.text(source));

    let mut arguments = BumpVec::new_in(bump);
    for argument in method_call.arguments() {
        arguments.push(convert_expression(bump, &argument, source)?);
    }

    Ok(est::Expression::MethodCall {
        receiver: bump.alloc(receiver_expression),
        method,
        arguments,
    })
}

fn convert_index<'a>(
    bump: &'a Bump,
    index_expression: &ast::IndexExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let receiver = index_expression
        .receiver()
        .ok_or(ConvertError::MissingNode("index receiver"))?;

    let receiver_expression = convert_expression(bump, &receiver, source)?;

    if let Some(index_value) = index_expression.index() {
        if let ast::Expression::Literal(literal) = &index_value
            && let Some(string_token) = literal.as_string()
        {
            let attribute = LazyEscape::new(string_token.text(source)).unescape_in(bump);
            return Ok(est::Expression::GetAttribute {
                expression: bump.alloc(receiver_expression),
                attribute,
            });
        }

        let index_converted = convert_expression(bump, &index_value, source)?;
        return Ok(est::Expression::Index {
            expression: bump.alloc(receiver_expression),
            index: bump.alloc(index_converted),
        });
    }

    if let Some(string_token) = index_expression.index_string() {
        let attribute = LazyEscape::new(string_token.text(source)).unescape_in(bump);
        return Ok(est::Expression::GetAttribute {
            expression: bump.alloc(receiver_expression),
            attribute,
        });
    }

    Err(ConvertError::MissingNode("index"))
}

fn convert_function_call<'a>(
    bump: &'a Bump,
    function_call: &ast::FunctionCallExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    let name_node = function_call
        .name()
        .ok_or(ConvertError::MissingNode("function name"))?;

    let name = build_type_name(bump, &name_node, source);

    let mut arguments = BumpVec::new_in(bump);
    for argument in function_call.arguments() {
        arguments.push(convert_expression(bump, &argument, source)?);
    }

    Ok(est::Expression::ExtensionCall { name, arguments })
}

fn convert_path<'a>(
    bump: &'a Bump,
    path: &ast::PathExpression<'_>,
    source: &'a str,
) -> Result<est::Expression<'a>, ConvertError> {
    if let Some(variable) = path.as_variable(source) {
        let est_variable = match variable {
            ast::Variable::Principal => est::Variable::Principal,
            ast::Variable::Action => est::Variable::Action,
            ast::Variable::Resource => est::Variable::Resource,
            ast::Variable::Context => est::Variable::Context,
        };

        return Ok(est::Expression::Variable(est_variable));
    }

    let name_node = path.path().ok_or(ConvertError::MissingNode("path name"))?;
    let name = build_type_name(bump, &name_node, source);

    Ok(est::Expression::ExtensionCall {
        name,
        arguments: BumpVec::new_in(bump),
    })
}

fn extract_attr_key<'a>(bump: &'a Bump, key: ast::AttrKey<'_>, source: &'a str) -> &'a str {
    match key {
        ast::AttrKey::Identifier(identifier) => bump.alloc_str(identifier.text(source)),
        ast::AttrKey::String(string) => LazyEscape::new(string.text(source)).unescape_in(bump),
    }
}

fn parse_integer(text: &str) -> Result<i64, ConvertError> {
    text.parse::<i64>()
        .map_err(|_parse_error| ConvertError::IntegerOverflow)
}

/// Allocate a single char as a &str in the bump arena.
fn alloc_char(bump: &Bump, ch: char) -> &str {
    bump.alloc_str(ch.encode_utf8(&mut [0; 4]))
}

fn parse_pattern<'a>(bump: &'a Bump, text: &str) -> BumpVec<'a, est::PatternElement<'a>> {
    let inner = LazyEscape::new(text);
    let inner = inner.inner();

    let mut result = BumpVec::new_in(bump);
    let mut chars = inner.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '*' {
            result.push(est::PatternElement::Wildcard);
        } else if character == '\\' {
            let Some(escaped) = chars.next() else {
                result.push(est::PatternElement::Literal(bump.alloc_str("\\")));
                break;
            };

            let unescaped = match escaped {
                '*' => '*',
                '\\' => '\\',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '"' => '"',
                '\'' => '\'',
                '0' => '\0',
                'x' => {
                    if let (Some(hex_1), Some(hex_2)) = (chars.peek().copied(), {
                        let mut iter = chars.clone();
                        iter.next();
                        iter.peek().copied()
                    }) && hex_1.is_ascii_hexdigit()
                        && hex_2.is_ascii_hexdigit()
                    {
                        chars.next();
                        chars.next();
                        let hex_str: String = [hex_1, hex_2].iter().collect();
                        if let Ok(number) = u8::from_str_radix(&hex_str, 16) {
                            result.push(est::PatternElement::Literal(alloc_char(
                                bump,
                                number as char,
                            )));
                            continue;
                        }
                    }
                    result.push(est::PatternElement::Literal(bump.alloc_str("\\")));
                    result.push(est::PatternElement::Literal(bump.alloc_str("x")));
                    continue;
                }
                'u' => {
                    if chars.peek() == Some(&'{') {
                        chars.next();

                        let mut hex_chars = Vec::new();
                        while let Some(&ch) = chars.peek() {
                            if ch == '}' || !ch.is_ascii_hexdigit() {
                                break;
                            }
                            hex_chars.push(chars.next().unwrap_or_default());
                        }

                        let hex_str: String = hex_chars.iter().collect();
                        if let Ok(number) = u32::from_str_radix(&hex_str, 16)
                            && let Some(ch) = char::from_u32(number)
                        {
                            if chars.peek() == Some(&'}') {
                                chars.next();
                            }
                            result.push(est::PatternElement::Literal(alloc_char(bump, ch)));
                            continue;
                        }

                        result.push(est::PatternElement::Literal(bump.alloc_str("\\")));
                        result.push(est::PatternElement::Literal(bump.alloc_str("u")));
                        result.push(est::PatternElement::Literal(bump.alloc_str("{")));
                        continue;
                    }

                    result.push(est::PatternElement::Literal(bump.alloc_str("\\")));
                    result.push(est::PatternElement::Literal(bump.alloc_str("u")));
                    continue;
                }
                other => {
                    result.push(est::PatternElement::Literal(bump.alloc_str("\\")));
                    result.push(est::PatternElement::Literal(alloc_char(bump, other)));
                    continue;
                }
            };
            result.push(est::PatternElement::Literal(alloc_char(bump, unescaped)));
        } else {
            result.push(est::PatternElement::Literal(alloc_char(bump, character)));
        }
    }

    result
}
