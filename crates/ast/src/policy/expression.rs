use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::common::Name;
use crate::policy::{
    BinaryOperator, BoolLiteral, EntityReference, IntegerLiteral, Literal, Pattern,
    RecordExpression, SlotKind, StringLiteral, UnaryOperator, Variable,
};

/// An expression node.
#[derive(Clone, Debug)]
pub struct Expression<'a> {
    kind: ExpressionKind<'a>,
}

impl<'a> Expression<'a> {
    /// Creates a new expression.
    #[must_use]
    pub const fn new(kind: ExpressionKind<'a>) -> Self {
        Self { kind }
    }

    /// Returns the expression kind.
    #[must_use]
    pub const fn kind(&self) -> &ExpressionKind<'a> {
        &self.kind
    }

    /// Creates a boolean literal expression.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::new(ExpressionKind::Literal(Literal::Bool(BoolLiteral::new(
            value,
        ))))
    }

    /// Creates an integer literal expression.
    #[must_use]
    pub const fn integer(literal: IntegerLiteral) -> Self {
        Self::new(ExpressionKind::Literal(Literal::Integer(literal)))
    }

    /// Creates a string literal expression.
    #[must_use]
    pub const fn string(value: Cow<'a, str>) -> Self {
        Self::new(ExpressionKind::Literal(Literal::String(
            StringLiteral::new(value),
        )))
    }

    /// Creates an entity literal expression.
    #[must_use]
    pub const fn entity(reference: EntityReference<'a>) -> Self {
        Self::new(ExpressionKind::Literal(Literal::Entity(reference)))
    }

    /// Creates a variable expression.
    #[must_use]
    pub const fn variable(variable: Variable) -> Self {
        Self::new(ExpressionKind::Variable(variable))
    }

    /// Creates a slot expression.
    #[must_use]
    pub const fn slot(kind: SlotKind) -> Self {
        Self::new(ExpressionKind::Slot(kind))
    }

    /// Creates a logical and expression.
    #[must_use]
    pub fn and(left: Self, right: Self) -> Self {
        Self::new(ExpressionKind::And {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// Creates a logical or expression.
    #[must_use]
    pub fn or(left: Self, right: Self) -> Self {
        Self::new(ExpressionKind::Or {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// Creates a binary operator expression.
    #[must_use]
    pub fn binary(operator: BinaryOperator, left: Self, right: Self) -> Self {
        Self::new(ExpressionKind::BinaryApp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /// Creates a unary operator expression.
    #[must_use]
    pub fn unary(operator: UnaryOperator, operand: Self) -> Self {
        Self::new(ExpressionKind::UnaryApp {
            operator,
            operand: Box::new(operand),
        })
    }

    /// Creates an if-then-else expression.
    #[must_use]
    pub fn if_then_else(test: Self, consequent: Self, alternate: Self) -> Self {
        Self::new(ExpressionKind::If {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
        })
    }

    /// Creates a get-attribute expression.
    #[must_use]
    pub fn get_attribute(expression: Self, attribute: Cow<'a, str>) -> Self {
        Self::new(ExpressionKind::GetAttribute {
            expression: Box::new(expression),
            attribute,
        })
    }

    /// Creates a has-attribute expression.
    #[must_use]
    pub fn has_attribute(expression: Self, attribute: Cow<'a, str>) -> Self {
        Self::new(ExpressionKind::HasAttribute {
            expression: Box::new(expression),
            attribute,
        })
    }

    /// Creates a like expression.
    #[must_use]
    pub fn like(expression: Self, pattern: Pattern<'a>) -> Self {
        Self::new(ExpressionKind::Like {
            expression: Box::new(expression),
            pattern,
        })
    }

    /// Creates an is expression.
    #[must_use]
    pub fn is(expression: Self, kind: Name<'a>) -> Self {
        Self::new(ExpressionKind::Is {
            expression: Box::new(expression),
            kind,
        })
    }

    /// Creates an is-in expression.
    #[must_use]
    pub fn is_in(expression: Self, kind: Name<'a>, target: Self) -> Self {
        Self::new(ExpressionKind::IsIn {
            expression: Box::new(expression),
            kind,
            target: Box::new(target),
        })
    }

    /// Creates a set expression.
    #[must_use]
    pub const fn set(elements: Vec<Self>) -> Self {
        Self::new(ExpressionKind::Set(elements))
    }

    /// Creates a record expression.
    #[must_use]
    pub const fn record(record: RecordExpression<'a>) -> Self {
        Self::new(ExpressionKind::Record(record))
    }

    /// Creates an extension function call expression.
    #[must_use]
    pub const fn extension_call(function: Name<'a>, arguments: Vec<Self>) -> Self {
        Self::new(ExpressionKind::ExtensionCall {
            function,
            arguments,
        })
    }
}

/// The kind of expression.
#[derive(Clone, Debug)]
pub enum ExpressionKind<'a> {
    Literal(Literal<'a>),
    Variable(Variable),
    Slot(SlotKind),
    If {
        test: Box<Expression<'a>>,
        consequent: Box<Expression<'a>>,
        alternate: Box<Expression<'a>>,
    },
    And {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
    Or {
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
    UnaryApp {
        operator: UnaryOperator,
        operand: Box<Expression<'a>>,
    },
    BinaryApp {
        operator: BinaryOperator,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
    GetAttribute {
        expression: Box<Expression<'a>>,
        attribute: Cow<'a, str>,
    },
    HasAttribute {
        expression: Box<Expression<'a>>,
        attribute: Cow<'a, str>,
    },
    Like {
        expression: Box<Expression<'a>>,
        pattern: Pattern<'a>,
    },
    Is {
        expression: Box<Expression<'a>>,
        kind: Name<'a>,
    },
    IsIn {
        expression: Box<Expression<'a>>,
        kind: Name<'a>,
        target: Box<Expression<'a>>,
    },
    ExtensionCall {
        function: Name<'a>,
        arguments: Vec<Expression<'a>>,
    },
    Set(Vec<Expression<'a>>),
    Record(RecordExpression<'a>),
}
