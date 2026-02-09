use core::fmt;

/// A unary operator.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum UnaryOperator {
    Not,
    Negate,
    IsEmpty,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => f.write_str("!"),
            Self::Negate => f.write_str("-"),
            Self::IsEmpty => f.write_str("isEmpty"),
        }
    }
}
