use core::fmt;

/// A binary operator.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    In,
    Add,
    Subtract,
    Multiply,
    Contains,
    ContainsAll,
    ContainsAny,
    GetTag,
    HasTag,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => f.write_str("=="),
            Self::NotEqual => f.write_str("!="),
            Self::Less => f.write_str("<"),
            Self::LessEqual => f.write_str("<="),
            Self::Greater => f.write_str(">"),
            Self::GreaterEqual => f.write_str(">="),
            Self::In => f.write_str("in"),
            Self::Add => f.write_str("+"),
            Self::Subtract => f.write_str("-"),
            Self::Multiply => f.write_str("*"),
            Self::Contains => f.write_str("contains"),
            Self::ContainsAll => f.write_str("containsAll"),
            Self::ContainsAny => f.write_str("containsAny"),
            Self::GetTag => f.write_str("getTag"),
            Self::HasTag => f.write_str("hasTag"),
        }
    }
}
