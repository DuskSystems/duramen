use alloc::format;
use alloc::string::String;
use core::fmt;

use duramen_diagnostic::Diagnostic;

/// An error produced during AST node construction.
#[derive(Clone, Debug)]
pub enum Error {
    /// A collection that must be non-empty was empty.
    Empty,
    /// A map or set contained a duplicate key.
    DuplicateKey { key: String },
    /// An identifier uses the reserved `__cedar` prefix.
    ReservedPrefix { name: String },
    /// A type name conflicts with a built-in type.
    ReservedTypeName { name: String },
    /// A slot name is not recognized.
    InvalidSlot { name: String },
    /// An integer literal overflows `i64`.
    IntegerOverflow { text: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("expected at least one element"),
            Self::DuplicateKey { key } => write!(f, "duplicate key `{key}`"),
            Self::ReservedPrefix { name } => {
                write!(f, "identifier `{name}` uses reserved `__cedar` prefix")
            }
            Self::ReservedTypeName { name } => write!(f, "`{name}` is a reserved type name"),
            Self::InvalidSlot { name } => {
                write!(f, "invalid slot `?{name}`")
            }
            Self::IntegerOverflow { text } => {
                write!(f, "integer literal `{text}` is out of range")
            }
        }
    }
}

impl core::error::Error for Error {}

impl From<Error> for Diagnostic {
    fn from(value: Error) -> Self {
        match &value {
            Error::InvalidSlot { .. } => Self::error(format!("{value}"))
                .with_note("only `?principal` and `?resource` are allowed"),
            _ => Self::error(format!("{value}")),
        }
    }
}
