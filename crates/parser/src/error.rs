use alloc::format;
use core::ops::Range;

use duramen_diagnostic::Diagnostic;

/// Errors reported during parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
    /// Expression nesting exceeds the maximum depth.
    NestingTooDeep { span: Range<usize> },
    /// A string literal is missing its closing quote.
    UnterminatedString { span: Range<usize> },
    /// Single-quoted strings are not supported in Cedar.
    StringSingleQuoted { span: Range<usize> },
    /// Block comments are not supported in Cedar.
    BlockComment {
        open: Range<usize>,
        close: Option<Range<usize>>,
    },
    /// A token that doesn't belong in this position.
    UnexpectedToken { span: Range<usize> },
    /// A required token was not found.
    ExpectedToken {
        span: Range<usize>,
        expected: &'static str,
    },
    /// An annotation name contains invalid characters.
    InvalidAnnotationName { span: Range<usize> },
}

impl From<ParseError> for Diagnostic {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::NestingTooDeep { span } => {
                Self::error("nesting too deep").with_label(span, "maximum nesting depth exceeded")
            }
            ParseError::UnterminatedString { span } => {
                Self::error("unterminated string literal").with_label(span, "missing closing `\"`")
            }
            ParseError::StringSingleQuoted { span } => {
                Self::error("single-quoted strings are not supported")
                    .with_label(span, "")
                    .with_help("use double quotes for strings")
            }
            ParseError::BlockComment { open, close } => {
                let diagnostic = Self::error("block comments are not supported")
                    .with_context(open, "opened here")
                    .with_help("use `//` for comments");

                if let Some(close) = close {
                    diagnostic.with_context(close, "closed here")
                } else {
                    diagnostic
                }
            }
            ParseError::UnexpectedToken { span } => {
                Self::error("unexpected token").with_label(span, "")
            }
            ParseError::ExpectedToken { span, expected } => {
                Self::error(format!("expected {expected}")).with_label(span, "")
            }
            ParseError::InvalidAnnotationName { span } => Self::error("invalid annotation name")
                .with_label(span, "must be a valid identifier"),
        }
    }
}
