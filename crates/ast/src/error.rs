use alloc::string::String;
use core::fmt;

/// An error produced during AST node construction.
#[derive(Clone, Debug)]
pub enum Error {
    /// A collection that must be non-empty was empty.
    Empty,
    /// A map or set contained a duplicate key.
    DuplicateKey { key: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("expected at least one element"),
            Self::DuplicateKey { key } => write!(f, "duplicate key `{key}`"),
        }
    }
}

impl core::error::Error for Error {}
