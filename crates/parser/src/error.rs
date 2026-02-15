use alloc::format;
use core::ops::Range;

use duramen_diagnostic::Diagnostic;
use duramen_lexer::TokenKind;

/// Errors reported during parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
    /// Expression nesting exceeds the maximum depth.
    NestingTooDeep { span: Range<usize> },

    /// A required token was missing.
    Missing {
        span: Range<usize>,
        expected: TokenKind,
    },

    /// An unexpected token was encountered.
    Unexpected { span: Range<usize> },
}

impl From<ParseError> for Diagnostic {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::NestingTooDeep { span } => {
                Self::error("nesting too deep").with_label(span, "maximum nesting depth exceeded")
            }
            ParseError::Missing { span, expected } => Self::error(format!("expected {expected}"))
                .with_label(span, format!("expected {expected}")),
            ParseError::Unexpected { span } => {
                Self::error("unexpected token").with_label(span, "unexpected token")
            }
        }
    }
}
