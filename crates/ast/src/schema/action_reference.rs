use alloc::borrow::Cow;

use crate::common::Name;

/// A reference to an action entity.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct ActionReference<'a> {
    kind: Option<Name<'a>>,
    id: Cow<'a, str>,
}

impl<'a> ActionReference<'a> {
    /// Creates a new action reference.
    #[must_use]
    pub const fn new(kind: Option<Name<'a>>, id: Cow<'a, str>) -> Self {
        Self { kind, id }
    }

    /// Returns the action kind, if specified.
    #[must_use]
    pub const fn kind(&self) -> Option<&Name<'a>> {
        self.kind.as_ref()
    }

    /// Returns the action identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}
