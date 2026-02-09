use core::fmt;
use core::ops::Range;

use crate::policy::Expression;

/// Whether a condition is `when` or `unless`.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum ConditionKind {
    When,
    Unless,
}

impl fmt::Display for ConditionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::When => f.write_str("when"),
            Self::Unless => f.write_str("unless"),
        }
    }
}

/// A condition clause on a policy.
#[derive(Clone, Debug)]
pub struct Condition<'a> {
    kind: ConditionKind,
    body: Expression<'a>,
    span: Range<usize>,
}

impl<'a> Condition<'a> {
    /// Creates a new condition.
    #[must_use]
    pub const fn new(kind: ConditionKind, body: Expression<'a>, span: Range<usize>) -> Self {
        Self { kind, body, span }
    }

    /// Returns the condition kind.
    #[must_use]
    pub const fn kind(&self) -> ConditionKind {
        self.kind
    }

    /// Returns the condition body expression.
    #[must_use]
    pub const fn body(&self) -> &Expression<'a> {
        &self.body
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        &self.span
    }
}
