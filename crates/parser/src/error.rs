use alloc::format;
use core::ops::Range;

use duramen_diagnostic::Diagnostic;
use duramen_syntax::Syntax;

/// Errors reported during parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
    /// Expression nesting exceeds the maximum depth.
    NestingTooDeep { span: Range<usize> },
    /// Expected a specific syntax element that was not found.
    Missing {
        span: Range<usize>,
        expected: Syntax,
    },
    /// Encountered a token that does not belong here.
    Unexpected { span: Range<usize> },
}

impl From<ParseError> for Diagnostic {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::NestingTooDeep { span } => {
                Self::error("nesting too deep").with_label(span, "maximum nesting depth exceeded")
            }
            ParseError::Missing { span, expected } => {
                let message = format!("expected `{expected}`");
                Self::error(&message).with_label(span, &message)
            }
            ParseError::Unexpected { span } => {
                Self::error("unexpected token").with_label(span, "unexpected token")
            }
        }
    }
}
