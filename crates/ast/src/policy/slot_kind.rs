use alloc::string::String;
use core::fmt;

use crate::error::Error;

/// The kind of template slot.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum SlotKind {
    Principal,
    Resource,
}

impl SlotKind {
    /// Creates a slot kind from a name.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is invalid.
    pub fn new(name: &str) -> Result<Self, Error> {
        match name {
            "principal" => Ok(Self::Principal),
            "resource" => Ok(Self::Resource),
            _ => Err(Error::InvalidSlot {
                name: String::from(name),
            }),
        }
    }
}

impl fmt::Display for SlotKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Principal => f.write_str("?principal"),
            Self::Resource => f.write_str("?resource"),
        }
    }
}
