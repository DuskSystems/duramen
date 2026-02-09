use alloc::borrow::Cow;
use core::fmt;

use crate::common::Name;

/// A concrete entity reference like `User::"alice"`.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct EntityReference<'a> {
    kind: Name<'a>,
    id: Cow<'a, str>,
}

impl<'a> EntityReference<'a> {
    /// Creates a new entity reference.
    #[must_use]
    pub const fn new(kind: Name<'a>, id: Cow<'a, str>) -> Self {
        Self { kind, id }
    }

    /// Returns the entity kind (e.g. `User`).
    #[must_use]
    pub const fn kind(&self) -> &Name<'a> {
        &self.kind
    }

    /// Returns the entity identifier (e.g. `"alice"`).
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl fmt::Display for EntityReference<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::\"{}\"", self.kind, self.id)
    }
}
