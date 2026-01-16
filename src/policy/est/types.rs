use bumpalo::collections::Vec as BumpVec;

use crate::escape::LazyEscape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Permit,
    Forbid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variable {
    Principal,
    Action,
    Resource,
    Context,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotId {
    Principal,
    Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternElement<'a> {
    Literal(&'a str),
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub enum PrincipalOrResourceConstraint<'a> {
    Any,
    Equal(Expression<'a>),
    In(Expression<'a>),
    Is {
        entity_type: &'a str,
    },
    IsIn {
        entity_type: &'a str,
        in_entity: Expression<'a>,
    },
}

#[derive(Debug, PartialEq)]
pub enum ActionConstraint<'a> {
    Any,
    Equal(Expression<'a>),
    In(BumpVec<'a, Expression<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct Condition<'a> {
    pub kind: ConditionKind,
    pub body: Expression<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionKind {
    When,
    Unless,
}

#[derive(Debug, PartialEq)]
pub struct Policy<'a> {
    pub effect: Effect,
    pub principal: PrincipalOrResourceConstraint<'a>,
    pub action: ActionConstraint<'a>,
    pub resource: PrincipalOrResourceConstraint<'a>,
    pub conditions: BumpVec<'a, Condition<'a>>,
    pub annotations: BumpVec<'a, (&'a str, Option<LazyEscape<'a>>)>,
}

impl Policy<'_> {
    #[must_use]
    pub fn is_template(&self) -> bool {
        self.principal.has_slot() || self.action.has_slot() || self.resource.has_slot()
    }
}

impl PrincipalOrResourceConstraint<'_> {
    #[must_use]
    pub fn has_slot(&self) -> bool {
        match self {
            Self::Any | Self::Is { .. } => false,
            Self::Equal(expression) | Self::In(expression) => expression.has_slot(),
            Self::IsIn { in_entity, .. } => in_entity.has_slot(),
        }
    }
}

impl ActionConstraint<'_> {
    #[must_use]
    pub fn has_slot(&self) -> bool {
        match self {
            Self::Any => false,
            Self::Equal(expression) => expression.has_slot(),
            Self::In(expressions) => expressions.iter().any(Expression::has_slot),
        }
    }
}

impl Expression<'_> {
    #[must_use]
    pub fn has_slot(&self) -> bool {
        match self {
            Self::Slot(_) => true,
            Self::Boolean(_)
            | Self::Integer(_)
            | Self::String(_)
            | Self::Variable(_)
            | Self::Entity { .. } => false,
            Self::Set(elements) => elements.iter().any(Self::has_slot),
            Self::Record(entries) => entries.iter().any(|(_, value)| value.has_slot()),
            Self::Not(inner) | Self::Negate(inner) => inner.has_slot(),
            Self::Or(left, right)
            | Self::And(left, right)
            | Self::Equal(left, right)
            | Self::NotEqual(left, right)
            | Self::LessThan(left, right)
            | Self::LessThanOrEqual(left, right)
            | Self::GreaterThan(left, right)
            | Self::GreaterThanOrEqual(left, right)
            | Self::In(left, right)
            | Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right) => left.has_slot() || right.has_slot(),
            Self::GetAttribute { expression, .. }
            | Self::HasAttribute { expression, .. }
            | Self::Like { expression, .. } => expression.has_slot(),
            Self::Index { expression, index } => expression.has_slot() || index.has_slot(),
            Self::Is {
                expression,
                in_expression,
                ..
            } => {
                expression.has_slot()
                    || in_expression.as_ref().is_some_and(|inner| inner.has_slot())
            }
            Self::If {
                condition,
                then_expression,
                else_expression,
            } => condition.has_slot() || then_expression.has_slot() || else_expression.has_slot(),
            Self::MethodCall {
                receiver,
                arguments,
                ..
            } => receiver.has_slot() || arguments.iter().any(Self::has_slot),
            Self::ExtensionCall { arguments, .. } => arguments.iter().any(Self::has_slot),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Boolean(bool),
    Integer(i64),
    String(LazyEscape<'a>),
    Variable(Variable),
    Slot(SlotId),
    Entity {
        entity_type: &'a str,
        id: LazyEscape<'a>,
    },
    Set(BumpVec<'a, Self>),
    Record(BumpVec<'a, (&'a str, Self)>),
    Not(&'a Self),
    Negate(&'a Self),
    Or(&'a Self, &'a Self),
    And(&'a Self, &'a Self),
    Equal(&'a Self, &'a Self),
    NotEqual(&'a Self, &'a Self),
    LessThan(&'a Self, &'a Self),
    LessThanOrEqual(&'a Self, &'a Self),
    GreaterThan(&'a Self, &'a Self),
    GreaterThanOrEqual(&'a Self, &'a Self),
    In(&'a Self, &'a Self),
    Add(&'a Self, &'a Self),
    Subtract(&'a Self, &'a Self),
    Multiply(&'a Self, &'a Self),
    GetAttribute {
        expression: &'a Self,
        attribute: &'a str,
    },
    HasAttribute {
        expression: &'a Self,
        attribute: &'a str,
    },
    Index {
        expression: &'a Self,
        index: &'a Self,
    },
    Like {
        expression: &'a Self,
        pattern: BumpVec<'a, PatternElement<'a>>,
    },
    Is {
        expression: &'a Self,
        entity_type: &'a str,
        in_expression: Option<&'a Self>,
    },
    If {
        condition: &'a Self,
        then_expression: &'a Self,
        else_expression: &'a Self,
    },
    MethodCall {
        receiver: &'a Self,
        method: &'a str,
        arguments: BumpVec<'a, Self>,
    },
    ExtensionCall {
        name: &'a str,
        arguments: BumpVec<'a, Self>,
    },
}
