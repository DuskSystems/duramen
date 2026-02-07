use core::error::Error;
use core::fmt;

/// Errors that can occur when creating a lexer.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum LexerError {
    /// Input source is too large to lex.
    InputTooLarge {
        /// The actual length of the input.
        len: usize,
        /// The maximum supported length.
        max: usize,
    },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTooLarge { len, max } => {
                write!(
                    f,
                    "input too large: {len} bytes exceeds maximum of {max} bytes"
                )
            }
        }
    }
}

impl Error for LexerError {}
