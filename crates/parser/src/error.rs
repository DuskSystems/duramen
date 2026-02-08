use core::ops::Range;

use duramen_diagnostic::{Diagnostic, Suggestion};
use duramen_lexer::TokenKind;

/// Errors reported during parsing.
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Single `&` used instead of `&&`.
    SingleAmpersand { span: Range<usize> },
    /// Single `|` used instead of `||`.
    SinglePipe { span: Range<usize> },
    /// Single `=` used instead of `==`.
    SingleEquals { span: Range<usize> },
    /// Unterminated string literal.
    UnterminatedString { span: Range<usize> },
    /// Unrecognized character.
    UnknownCharacter { span: Range<usize> },
    /// Unexpected token where identifier was expected.
    UnexpectedToken { span: Range<usize> },
    /// Missing expected token.
    Expected {
        span: Range<usize>,
        expected: TokenKind,
    },
}

impl From<ParseError> for Diagnostic {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::SingleAmpersand { span } => Self::error("invalid operator `&`")
                .with_label(span.clone(), "not a valid operator")
                .with_suggestion(
                    Suggestion::fix(span, "&&").with_message("use `&&` for logical AND"),
                ),
            ParseError::SinglePipe { span } => Self::error("invalid operator `|`")
                .with_label(span.clone(), "not a valid operator")
                .with_suggestion(
                    Suggestion::fix(span, "||").with_message("use `||` for logical OR"),
                ),
            ParseError::UnterminatedString { span } => {
                Self::error("unterminated string literal").with_label(span, "missing closing `\"`")
            }
            ParseError::UnknownCharacter { span } => {
                Self::error("unrecognized character").with_label(span, "not valid in Cedar")
            }
            ParseError::SingleEquals { span } => Self::error("invalid operator `=`")
                .with_label(span.clone(), "not a valid operator")
                .with_suggestion(Suggestion::fix(span, "==").with_message("use `==` for equality")),
            ParseError::UnexpectedToken { span } => {
                Self::error("unexpected token").with_label(span, "expected identifier")
            }
            ParseError::Expected { span, expected } => {
                Self::error(alloc::format!("expected {expected}"))
                    .with_label(span, alloc::format!("expected {expected}"))
            }
        }
    }
}
