use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternElement {
    Literal(String),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrincipalOrResourceConstraint {
    Any,
    Equal(Expression),
    In(Expression),
    Is {
        entity_type: String,
    },
    IsIn {
        entity_type: String,
        in_entity: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionConstraint {
    Any,
    Equal(Expression),
    In(Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub kind: ConditionKind,
    pub body: Expression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionKind {
    When,
    Unless,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Policy {
    pub effect: Effect,
    pub principal: PrincipalOrResourceConstraint,
    pub action: ActionConstraint,
    pub resource: PrincipalOrResourceConstraint,
    pub conditions: Vec<Condition>,
    pub annotations: Vec<(String, Option<String>)>,
}

impl Policy {
    #[must_use]
    pub fn is_template(&self) -> bool {
        self.principal.has_slot() || self.action.has_slot() || self.resource.has_slot()
    }
}

impl PrincipalOrResourceConstraint {
    #[must_use]
    pub fn has_slot(&self) -> bool {
        match self {
            Self::Any | Self::Is { .. } => false,
            Self::Equal(expression) | Self::In(expression) => expression.has_slot(),
            Self::IsIn { in_entity, .. } => in_entity.has_slot(),
        }
    }
}

impl ActionConstraint {
    #[must_use]
    pub fn has_slot(&self) -> bool {
        match self {
            Self::Any => false,
            Self::Equal(expression) => expression.has_slot(),
            Self::In(expressions) => expressions.iter().any(Expression::has_slot),
        }
    }
}

impl Expression {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Boolean(bool),
    Integer(i64),
    String(String),
    Variable(Variable),
    Slot(SlotId),
    Entity {
        entity_type: String,
        id: String,
    },
    Set(Vec<Self>),
    Record(Vec<(String, Self)>),
    Not(Box<Self>),
    Negate(Box<Self>),
    Or(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Equal(Box<Self>, Box<Self>),
    NotEqual(Box<Self>, Box<Self>),
    LessThan(Box<Self>, Box<Self>),
    LessThanOrEqual(Box<Self>, Box<Self>),
    GreaterThan(Box<Self>, Box<Self>),
    GreaterThanOrEqual(Box<Self>, Box<Self>),
    In(Box<Self>, Box<Self>),
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    GetAttribute {
        expression: Box<Self>,
        attribute: String,
    },
    HasAttribute {
        expression: Box<Self>,
        attribute: String,
    },
    Index {
        expression: Box<Self>,
        index: Box<Self>,
    },
    Like {
        expression: Box<Self>,
        pattern: Vec<PatternElement>,
    },
    Is {
        expression: Box<Self>,
        entity_type: String,
        in_expression: Option<Box<Self>>,
    },
    If {
        condition: Box<Self>,
        then_expression: Box<Self>,
        else_expression: Box<Self>,
    },
    MethodCall {
        receiver: Box<Self>,
        method: String,
        arguments: Vec<Self>,
    },
    ExtensionCall {
        name: String,
        arguments: Vec<Self>,
    },
}
