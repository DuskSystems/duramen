use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::Range;

use crate::common::Name;
use crate::policy::{
    BinaryOperator, Literal, Pattern, RecordExpression, SlotKind, UnaryOperator, Variable,
};

/// An expression with source span.
#[derive(Clone, Debug)]
pub struct Expression<'a> {
    kind: ExpressionKind<'a>,
    span: Range<usize>,
}

impl<'a> Expression<'a> {
    /// Creates a new expression.
    #[must_use]
    pub const fn new(kind: ExpressionKind<'a>, span: Range<usize>) -> Self {
        Self { kind, span }
    }

    /// Returns the expression kind.
    #[must_use]
    pub const fn kind(&self) -> &ExpressionKind<'a> {
        &self.kind
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        &self.span
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
