use core::error::Error;
use core::fmt;
use core::ops::Range;

use rustc_literal_escaper::EscapeError as RustcEscapeError;

#[derive(Debug)]
pub struct EscapeError {
    kind: RustcEscapeError,
    span: Range<usize>,
}

impl EscapeError {
    pub(crate) const fn new(kind: RustcEscapeError, span: Range<usize>) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        &self.span
    }
}

impl fmt::Display for EscapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            RustcEscapeError::LoneSlash => f.write_str("unexpected end of escape sequence"),
            RustcEscapeError::InvalidEscape | RustcEscapeError::EscapeOnlyChar => {
                f.write_str("invalid escape sequence")
            }
            RustcEscapeError::BareCarriageReturn
            | RustcEscapeError::BareCarriageReturnInRawString => {
                f.write_str("bare carriage return not allowed")
            }
            RustcEscapeError::TooShortHexEscape | RustcEscapeError::InvalidCharInHexEscape => {
                f.write_str("invalid hex escape")
            }
            RustcEscapeError::OutOfRangeHexEscape => f.write_str("out of range hex escape"),
            RustcEscapeError::NoBraceInUnicodeEscape
            | RustcEscapeError::InvalidCharInUnicodeEscape
            | RustcEscapeError::EmptyUnicodeEscape
            | RustcEscapeError::UnclosedUnicodeEscape
            | RustcEscapeError::LeadingUnderscoreUnicodeEscape => {
                f.write_str("invalid unicode escape")
            }
            RustcEscapeError::OverlongUnicodeEscape
            | RustcEscapeError::LoneSurrogateUnicodeEscape
            | RustcEscapeError::OutOfRangeUnicodeEscape => {
                f.write_str("out of range unicode escape")
            }
            _ => f.write_str("invalid escape sequence"),
        }
    }
}

impl Error for EscapeError {}
