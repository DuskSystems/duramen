use core::fmt;

/// The kind of template slot.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum SlotKind {
    Principal,
    Resource,
}

impl fmt::Display for SlotKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Principal => f.write_str("?principal"),
            Self::Resource => f.write_str("?resource"),
        }
    }
}
