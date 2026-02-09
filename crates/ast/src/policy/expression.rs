use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::common::Name;
use crate::policy::{
    BinaryOperator, Literal, Pattern, RecordExpression, SlotKind, UnaryOperator, Variable,
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
        pattern: Pattern,
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
