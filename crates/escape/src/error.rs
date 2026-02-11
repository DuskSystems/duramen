use alloc::format;
use core::error::Error;
use core::fmt;
use core::ops::Range;

use duramen_diagnostic::Diagnostic;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EscapeError {
    LoneSlash { span: Range<usize> },
    InvalidEscape { span: Range<usize> },
    BareCarriageReturn { span: Range<usize> },
    InvalidHexEscape { span: Range<usize> },
    OutOfRangeHexEscape { span: Range<usize> },
    InvalidUnicodeEscape { span: Range<usize> },
    OutOfRangeUnicodeEscape { span: Range<usize> },
}

impl EscapeError {
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        match self {
            Self::LoneSlash { span }
            | Self::InvalidEscape { span }
            | Self::BareCarriageReturn { span }
            | Self::InvalidHexEscape { span }
            | Self::OutOfRangeHexEscape { span }
            | Self::InvalidUnicodeEscape { span }
            | Self::OutOfRangeUnicodeEscape { span } => span,
        }
    }

    /// Shifts the span by the given byte offset.
    #[must_use]
    pub const fn offset(mut self, offset: usize) -> Self {
        match &mut self {
            Self::LoneSlash { span }
            | Self::InvalidEscape { span }
            | Self::BareCarriageReturn { span }
            | Self::InvalidHexEscape { span }
            | Self::OutOfRangeHexEscape { span }
            | Self::InvalidUnicodeEscape { span }
            | Self::OutOfRangeUnicodeEscape { span } => {
                span.start += offset;
                span.end += offset;
            }
        }

        self
    }
}

impl fmt::Display for EscapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoneSlash { .. } => f.write_str("unexpected end of escape sequence"),
            Self::InvalidEscape { .. } => f.write_str("invalid escape sequence"),
            Self::BareCarriageReturn { .. } => f.write_str("bare carriage return not allowed"),
            Self::InvalidHexEscape { .. } => f.write_str("invalid hex escape"),
            Self::OutOfRangeHexEscape { .. } => f.write_str("out of range hex escape"),
            Self::InvalidUnicodeEscape { .. } => f.write_str("invalid unicode escape"),
            Self::OutOfRangeUnicodeEscape { .. } => f.write_str("out of range unicode escape"),
        }
    }
}

impl Error for EscapeError {}

impl From<EscapeError> for Diagnostic {
    fn from(value: EscapeError) -> Self {
        let span = value.span().clone();
        Self::error(format!("{value}")).with_label(span, "invalid escape")
    }
}
