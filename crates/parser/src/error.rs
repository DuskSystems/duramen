use core::ops::Range;

use duramen_diagnostic::Diagnostic;

/// Errors reported during parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
    /// Expression nesting exceeds the maximum depth.
    NestingTooDeep { span: Range<usize> },
}

impl From<ParseError> for Diagnostic {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::NestingTooDeep { span } => {
                Self::error("nesting too deep").with_label(span, "maximum nesting depth exceeded")
            }
        }
    }
}
