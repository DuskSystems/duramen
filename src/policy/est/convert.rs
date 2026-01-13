use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::string::{String, ToString as _};
use alloc::vec;
use alloc::vec::Vec;

use memchr::memchr;

use super::error::ConvertError;
use super::types as est;
use crate::policy::ast::{self, AstToken as _, BinaryOperator, UnaryOperator};

type Result<T> = core::result::Result<T, ConvertError>;

pub fn convert_policies(policies: &ast::Policies<'_>, source: &str) -> Result<Vec<est::Policy>> {
    policies
        .policies()
        .map(|policy| convert_policy(&policy, source))
        .collect()
}

fn convert_policy(policy: &ast::Policy<'_>, source: &str) -> Result<est::Policy> {
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
                principal = convert_principal_or_resource_constraint(&variable_def, source)?;
            }
            Some(ast::Variable::Action) => {
                action = convert_action_constraint(&variable_def, source)?;
            }
            Some(ast::Variable::Resource) => {
                resource = convert_principal_or_resource_constraint(&variable_def, source)?;
            }
            Some(ast::Variable::Context) | None => {}
        }
    }

    let mut conditions = Vec::new();
    for condition in policy.conditions() {
        let kind = match condition.kind() {
            Some(ast::ConditionKind::When) => est::ConditionKind::When,
            Some(ast::ConditionKind::Unless) => est::ConditionKind::Unless,
            None => continue,
        };

        let Some(expression) = condition.expr() else {
            continue;
        };
        let body = convert_expression(&expression, source)?;

        conditions.push(est::Condition { kind, body });
    }

    let annotations = policy
        .annotations()
        .filter_map(|annotation| {
            let name = annotation.name()?.text(source).to_owned();
            let value = annotation
                .value()
                .map(|value| unescape_string_content(value.text(source)));
            Some((name, value))
        })
        .collect();

    Ok(est::Policy {
        effect,
        principal,
        action,
        resource,
        conditions,
        annotations,
    })
}

fn convert_principal_or_resource_constraint(
    variable_def: &ast::VariableDefinition<'_>,
    source: &str,
) -> Result<est::PrincipalOrResourceConstraint> {
    if variable_def.has_is_constraint() {
        let entity_type_name = variable_def
            .is_type_name()
            .ok_or(ConvertError::MissingNode("is entity type"))?;
        let entity_type = build_type_name(&entity_type_name, source);

        if variable_def.has_is_in_constraint() {
            let in_entity = variable_def
                .is_in_entity()
                .ok_or(ConvertError::MissingNode("is in entity"))?;
            let in_entity_expression = convert_expression(&in_entity, source)?;
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
        return convert_is_constraint(&is_expression, source);
    }

    let Some(relation) = variable_def.relation() else {
        return Ok(est::PrincipalOrResourceConstraint::Any);
    };

    let entity_expression = convert_expression(&constraint, source)?;

    match relation {
        BinaryOperator::Eq => Ok(est::PrincipalOrResourceConstraint::Equal(entity_expression)),
        BinaryOperator::In => Ok(est::PrincipalOrResourceConstraint::In(entity_expression)),
        _ => Ok(est::PrincipalOrResourceConstraint::Any),
    }
}

fn convert_is_constraint(
    is_expression: &ast::IsExpression<'_>,
    source: &str,
) -> Result<est::PrincipalOrResourceConstraint> {
    let entity_type_name = is_expression
        .entity_type()
        .ok_or(ConvertError::MissingNode("is entity type"))?;
    let entity_type = build_type_name(&entity_type_name, source);

    if let Some(in_entity) = is_expression.in_entity() {
        let in_entity_expression = convert_expression(&in_entity, source)?;
        return Ok(est::PrincipalOrResourceConstraint::IsIn {
            entity_type,
            in_entity: in_entity_expression,
        });
    }

    Ok(est::PrincipalOrResourceConstraint::Is { entity_type })
}

fn convert_action_constraint(
    variable_def: &ast::VariableDefinition<'_>,
    source: &str,
) -> Result<est::ActionConstraint> {
    let Some(constraint) = variable_def.constraint() else {
        return Ok(est::ActionConstraint::Any);
    };

    let Some(relation) = variable_def.relation() else {
        return Ok(est::ActionConstraint::Any);
    };

    match relation {
        BinaryOperator::Eq => {
            let entity_expression = convert_expression(&constraint, source)?;
            Ok(est::ActionConstraint::Equal(entity_expression))
        }
        BinaryOperator::In => {
            if let ast::Expression::List(list_expression) = constraint {
                let entities: Result<Vec<_>> = list_expression
                    .elements()
                    .map(|element| convert_expression(&element, source))
                    .collect();
                Ok(est::ActionConstraint::In(entities?))
            } else {
                let entity_expression = convert_expression(&constraint, source)?;
                Ok(est::ActionConstraint::In(vec![entity_expression]))
            }
        }
        _ => Ok(est::ActionConstraint::Any),
    }
}

fn build_type_name(name: &ast::Name<'_>, source: &str) -> String {
    let mut result = String::new();
    for (index, segment) in name.segments().enumerate() {
        if index > 0 {
            result.push_str("::");
        }
        result.push_str(segment.text(source));
    }
    result
}

fn convert_expression(expression: &ast::Expression<'_>, source: &str) -> Result<est::Expression> {
    match expression {
        ast::Expression::Literal(literal) => convert_literal(literal, source),
        ast::Expression::EntityRef(entity) => convert_entity_ref(entity, source),
        ast::Expression::Slot(slot) => convert_slot(slot, source),
        ast::Expression::Paren(paren) => {
            let inner = paren
                .inner()
                .ok_or(ConvertError::MissingNode("paren inner"))?;
            convert_expression(&inner, source)
        }
        ast::Expression::List(list) => {
            let elements: Result<Vec<_>> = list
                .elements()
                .map(|element| convert_expression(&element, source))
                .collect();
            Ok(est::Expression::Set(elements?))
        }
        ast::Expression::Record(record) => {
            let entries: Result<Vec<_>> = record
                .entries()
                .map(|entry| {
                    let key = entry
                        .key()
                        .map(|key| extract_attr_key(key, source))
                        .ok_or(ConvertError::MissingNode("record key"))?;
                    let value = entry
                        .value()
                        .ok_or(ConvertError::MissingNode("record value"))?;
                    let value_expression = convert_expression(&value, source)?;
                    Ok((key, value_expression))
                })
                .collect();
            Ok(est::Expression::Record(entries?))
        }
        ast::Expression::Unary(unary) => convert_unary(unary, source),
        ast::Expression::Binary(binary) => convert_binary(binary, source),
        ast::Expression::Has(has) => convert_has(has, source),
        ast::Expression::Like(like) => convert_like(like, source),
        ast::Expression::Is(is_expression) => convert_is(is_expression, source),
        ast::Expression::If(if_expression) => convert_if(if_expression, source),
        ast::Expression::Field(field) => convert_field(field, source),
        ast::Expression::MethodCall(method) => convert_method_call(method, source),
        ast::Expression::Index(index) => convert_index(index, source),
        ast::Expression::FunctionCall(func) => convert_function_call(func, source),
        ast::Expression::Path(path) => convert_path(path, source),
        ast::Expression::Unknown(_) => Err(ConvertError::MissingNode("unknown expression")),
    }
}

fn convert_literal(literal: &ast::LiteralExpression<'_>, source: &str) -> Result<est::Expression> {
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
        let value = unescape_string_content(text);
        return Ok(est::Expression::String(value));
    }

    Err(ConvertError::MissingNode("literal value"))
}

fn convert_entity_ref(
    entity: &ast::EntityRefExpression<'_>,
    source: &str,
) -> Result<est::Expression> {
    let entity_type = entity
        .entity_type()
        .ok_or(ConvertError::MissingNode("entity type"))?;

    let type_str = build_type_name(&entity_type, source);

    let id = entity
        .entity_id()
        .ok_or(ConvertError::MissingNode("entity id"))?;
    let id_str = unescape_string_content(id.text(source));

    Ok(est::Expression::Entity {
        entity_type: type_str,
        id: id_str,
    })
}

fn convert_slot(slot: &ast::SlotExpression<'_>, source: &str) -> Result<est::Expression> {
    let slot_kind = slot
        .slot_kind(source)
        .ok_or(ConvertError::MissingNode("slot kind"))?;

    let slot_id = match slot_kind {
        ast::SlotKind::Principal => est::SlotId::Principal,
        ast::SlotKind::Resource => est::SlotId::Resource,
    };

    Ok(est::Expression::Slot(slot_id))
}

fn convert_unary(unary: &ast::UnaryExpression<'_>, source: &str) -> Result<est::Expression> {
    let operator = unary
        .operator()
        .ok_or(ConvertError::MissingNode("unary operator"))?;
    let operand = unary
        .operand()
        .ok_or(ConvertError::MissingNode("unary operand"))?;

    match operator {
        UnaryOperator::Not => {
            let operand_expression = convert_expression(&operand, source)?;
            Ok(est::Expression::Not(Box::new(operand_expression)))
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
                if text == "9223372036854775808" {
                    return Ok(est::Expression::Integer(i64::MIN));
                }
            }
            let operand_expression = convert_expression(&operand, source)?;
            Ok(est::Expression::Negate(Box::new(operand_expression)))
        }
    }
}

fn convert_binary(binary: &ast::BinaryExpression<'_>, source: &str) -> Result<est::Expression> {
    let operator = binary
        .operator()
        .ok_or(ConvertError::MissingNode("binary operator"))?;
    let left = binary
        .left()
        .ok_or(ConvertError::MissingNode("binary left"))?;
    let right = binary
        .right()
        .ok_or(ConvertError::MissingNode("binary right"))?;

    let left_expression = Box::new(convert_expression(&left, source)?);
    let right_expression = Box::new(convert_expression(&right, source)?);

    macro_rules! binary_expression {
        ($variant:ident) => {
            Ok(est::Expression::$variant(left_expression, right_expression))
        };
    }

    match operator {
        BinaryOperator::Or => binary_expression!(Or),
        BinaryOperator::And => binary_expression!(And),
        BinaryOperator::Eq => binary_expression!(Equal),
        BinaryOperator::Neq => binary_expression!(NotEqual),
        BinaryOperator::Lt => binary_expression!(LessThan),
        BinaryOperator::Lte => binary_expression!(LessThanOrEqual),
        BinaryOperator::Gt => binary_expression!(GreaterThan),
        BinaryOperator::Gte => binary_expression!(GreaterThanOrEqual),
        BinaryOperator::In => binary_expression!(In),
        BinaryOperator::Add => binary_expression!(Add),
        BinaryOperator::Sub => binary_expression!(Subtract),
        BinaryOperator::Mul => binary_expression!(Multiply),
    }
}

fn convert_has(has: &ast::HasExpression<'_>, source: &str) -> Result<est::Expression> {
    let target = has
        .target()
        .ok_or(ConvertError::MissingNode("has target"))?;
    let attr_key = has
        .attribute()
        .ok_or(ConvertError::MissingNode("has attribute"))?;

    let target_expression = convert_expression(&target, source)?;
    let attribute = extract_attr_key(attr_key, source);

    Ok(est::Expression::HasAttribute {
        expression: Box::new(target_expression),
        attribute,
    })
}

fn convert_like(like: &ast::LikeExpression<'_>, source: &str) -> Result<est::Expression> {
    let target = like
        .target()
        .ok_or(ConvertError::MissingNode("like target"))?;
    let pattern_token = like
        .pattern()
        .ok_or(ConvertError::MissingNode("like pattern"))?;

    let target_expression = convert_expression(&target, source)?;
    let pattern_text = pattern_token.text(source);
    let pattern = parse_pattern(pattern_text);

    Ok(est::Expression::Like {
        expression: Box::new(target_expression),
        pattern,
    })
}

fn convert_is(is_expr: &ast::IsExpression<'_>, source: &str) -> Result<est::Expression> {
    let target = is_expr
        .target()
        .ok_or(ConvertError::MissingNode("is target"))?;
    let entity_type_name = is_expr
        .entity_type()
        .ok_or(ConvertError::MissingNode("is entity type"))?;

    let target_expression = convert_expression(&target, source)?;
    let entity_type = build_type_name(&entity_type_name, source);

    let in_expression = if let Some(in_entity) = is_expr.in_entity() {
        Some(Box::new(convert_expression(&in_entity, source)?))
    } else {
        None
    };

    Ok(est::Expression::Is {
        expression: Box::new(target_expression),
        entity_type,
        in_expression,
    })
}

fn convert_if(if_expression: &ast::IfExpression<'_>, source: &str) -> Result<est::Expression> {
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
        condition: Box::new(convert_expression(&cond, source)?),
        then_expression: Box::new(convert_expression(&then_branch, source)?),
        else_expression: Box::new(convert_expression(&else_branch, source)?),
    })
}

fn convert_field(field: &ast::FieldExpression<'_>, source: &str) -> Result<est::Expression> {
    let receiver = field
        .receiver()
        .ok_or(ConvertError::MissingNode("field receiver"))?;
    let field_name = field
        .field()
        .ok_or(ConvertError::MissingNode("field name"))?;

    let receiver_expression = convert_expression(&receiver, source)?;
    let attribute = field_name.text(source).to_owned();

    Ok(est::Expression::GetAttribute {
        expression: Box::new(receiver_expression),
        attribute,
    })
}

fn convert_method_call(
    method_call: &ast::MethodCallExpression<'_>,
    source: &str,
) -> Result<est::Expression> {
    let receiver = method_call
        .receiver()
        .ok_or(ConvertError::MissingNode("method receiver"))?;
    let method_name = method_call
        .method()
        .ok_or(ConvertError::MissingNode("method name"))?;

    let receiver_expression = convert_expression(&receiver, source)?;
    let method = method_name.text(source).to_owned();

    let arguments: Result<Vec<_>> = method_call
        .arguments()
        .map(|argument| convert_expression(&argument, source))
        .collect();

    Ok(est::Expression::MethodCall {
        receiver: Box::new(receiver_expression),
        method,
        arguments: arguments?,
    })
}

fn convert_index(
    index_expression: &ast::IndexExpression<'_>,
    source: &str,
) -> Result<est::Expression> {
    let receiver = index_expression
        .receiver()
        .ok_or(ConvertError::MissingNode("index receiver"))?;

    let receiver_expression = convert_expression(&receiver, source)?;

    if let Some(index_value) = index_expression.index() {
        if let ast::Expression::Literal(literal) = &index_value
            && let Some(string_token) = literal.as_string()
        {
            let attribute = unescape_string_content(string_token.text(source));
            return Ok(est::Expression::GetAttribute {
                expression: Box::new(receiver_expression),
                attribute,
            });
        }
        let index_converted = convert_expression(&index_value, source)?;
        return Ok(est::Expression::Index {
            expression: Box::new(receiver_expression),
            index: Box::new(index_converted),
        });
    }

    if let Some(string_token) = index_expression.index_string() {
        let attribute = unescape_string_content(string_token.text(source));
        return Ok(est::Expression::GetAttribute {
            expression: Box::new(receiver_expression),
            attribute,
        });
    }

    Err(ConvertError::MissingNode("index"))
}

fn convert_function_call(
    function_call: &ast::FunctionCallExpression<'_>,
    source: &str,
) -> Result<est::Expression> {
    let name_node = function_call
        .name()
        .ok_or(ConvertError::MissingNode("function name"))?;
    let name = build_type_name(&name_node, source);

    let arguments: Result<Vec<_>> = function_call
        .arguments()
        .map(|argument| convert_expression(&argument, source))
        .collect();

    Ok(est::Expression::ExtensionCall {
        name,
        arguments: arguments?,
    })
}

fn convert_path(path: &ast::PathExpression<'_>, source: &str) -> Result<est::Expression> {
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

    let name = build_type_name(&name_node, source);

    Ok(est::Expression::ExtensionCall {
        name,
        arguments: Vec::new(),
    })
}

fn extract_attr_key(key: ast::AttrKey<'_>, source: &str) -> String {
    match key {
        ast::AttrKey::Identifier(identifier) => identifier.text(source).to_owned(),
        ast::AttrKey::String(string) => unescape_string_content(string.text(source)),
    }
}

fn parse_integer(text: &str) -> Result<i64> {
    text.parse::<i64>()
        .map_err(|_parse_error| ConvertError::IntegerOverflow)
}

fn unescape_string_content(text: &str) -> String {
    let inner = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
        &text[1..text.len().saturating_sub(1)]
    } else {
        text
    };

    if !has_escapes(inner) {
        return inner.to_owned();
    }

    unescape_string_slow(inner)
}

#[inline]
fn has_escapes(string: &str) -> bool {
    memchr(b'\\', string.as_bytes()).is_some()
}

fn unescape_string_slow(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    let bytes = string.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let Some(slice) = bytes.get(index..) else {
            break;
        };
        if let Some(position) = memchr(b'\\', slice) {
            if position > 0
                && let Some(chunk) = string.get(index..index.saturating_add(position))
            {
                result.push_str(chunk);
            }
            index = index.saturating_add(position).saturating_add(1);

            let Some(&escape_byte) = bytes.get(index) else {
                result.push('\\');
                break;
            };
            index = index.saturating_add(1);

            match escape_byte {
                b'n' => result.push('\n'),
                b'r' => result.push('\r'),
                b't' => result.push('\t'),
                b'"' => result.push('"'),
                b'\'' => result.push('\''),
                b'0' => result.push('\0'),
                b'\\' => result.push('\\'),
                b'*' => result.push('*'),
                b'x' => {
                    if let (Some(&hex_1), Some(&hex_2)) =
                        (bytes.get(index), bytes.get(index.saturating_add(1)))
                        && hex_1.is_ascii_hexdigit()
                        && hex_2.is_ascii_hexdigit()
                        && let Some(hex_str) = string.get(index..index.saturating_add(2))
                        && let Ok(number) = u8::from_str_radix(hex_str, 16)
                    {
                        result.push(number as char);
                        index = index.saturating_add(2);
                        continue;
                    }
                    result.push('\\');
                    result.push('x');
                }
                b'u' => {
                    if bytes.get(index) == Some(&b'{') {
                        index = index.saturating_add(1);
                        let start = index;
                        while let Some(&byte) = bytes.get(index) {
                            if byte == b'}' || !byte.is_ascii_hexdigit() {
                                break;
                            }
                            index = index.saturating_add(1);
                        }
                        if let Some(hex_str) = string.get(start..index)
                            && let Ok(number) = u32::from_str_radix(hex_str, 16)
                            && let Some(character) = char::from_u32(number)
                        {
                            result.push(character);
                            if bytes.get(index) == Some(&b'}') {
                                index = index.saturating_add(1);
                            }
                            continue;
                        }
                        result.push_str("\\u{");
                    } else {
                        result.push('\\');
                        result.push('u');
                    }
                }
                character => {
                    result.push('\\');
                    result.push(character as char);
                }
            }
        } else {
            if let Some(rest) = string.get(index..) {
                result.push_str(rest);
            }
            break;
        }
    }

    result
}

fn parse_pattern(text: &str) -> Vec<est::PatternElement> {
    let inner = if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
        &text[1..text.len().saturating_sub(1)]
    } else {
        text
    };

    let mut result = Vec::new();
    let mut chars = inner.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '*' {
            result.push(est::PatternElement::Wildcard);
        } else if character == '\\' {
            let Some(escaped) = chars.next() else {
                result.push(est::PatternElement::Literal("\\".into()));
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
                            result.push(est::PatternElement::Literal((number as char).to_string()));
                            continue;
                        }
                    }
                    result.push(est::PatternElement::Literal("\\".into()));
                    result.push(est::PatternElement::Literal("x".into()));
                    continue;
                }
                'u' => {
                    if chars.peek() == Some(&'{') {
                        chars.next();
                        let mut hex_chars = Vec::new();
                        while let Some(&character) = chars.peek() {
                            if character == '}' || !character.is_ascii_hexdigit() {
                                break;
                            }
                            hex_chars.push(chars.next().unwrap_or_default());
                        }
                        let hex_str: String = hex_chars.iter().collect();
                        if let Ok(number) = u32::from_str_radix(&hex_str, 16)
                            && let Some(character) = char::from_u32(number)
                        {
                            if chars.peek() == Some(&'}') {
                                chars.next();
                            }
                            result.push(est::PatternElement::Literal(character.to_string()));
                            continue;
                        }
                        result.push(est::PatternElement::Literal("\\".into()));
                        result.push(est::PatternElement::Literal("u".into()));
                        result.push(est::PatternElement::Literal("{".into()));
                        continue;
                    }
                    result.push(est::PatternElement::Literal("\\".into()));
                    result.push(est::PatternElement::Literal("u".into()));
                    continue;
                }
                _ => {
                    result.push(est::PatternElement::Literal("\\".into()));
                    result.push(est::PatternElement::Literal(escaped.to_string()));
                    continue;
                }
            };
            result.push(est::PatternElement::Literal(unescaped.to_string()));
        } else {
            result.push(est::PatternElement::Literal(character.to_string()));
        }
    }

    result
}
