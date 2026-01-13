use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    MissingNode(&'static str),
    IntegerOverflow,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNode(node) => write!(f, "missing required node: {node}"),
            Self::IntegerOverflow => write!(f, "integer literal out of range"),
        }
    }
}

impl core::error::Error for ConvertError {}
