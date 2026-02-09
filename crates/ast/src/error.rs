use alloc::string::String;
use core::fmt;
use core::ops::Range;

/// An error produced during AST node construction.
#[derive(Clone, Debug)]
pub enum Error {
    /// A collection that must be non-empty was empty.
    Empty { span: Range<usize> },
    /// A map or set contained a duplicate key.
    DuplicateKey { key: String, span: Range<usize> },
}

impl Error {
    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        match self {
            Self::Empty { span } | Self::DuplicateKey { span, .. } => span,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { .. } => f.write_str("expected at least one element"),
            Self::DuplicateKey { key, .. } => write!(f, "duplicate key: {key}"),
        }
    }
}

impl core::error::Error for Error {}
