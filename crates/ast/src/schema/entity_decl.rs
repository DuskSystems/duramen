use alloc::vec::Vec;

use super::types::{RecordType, Type};
use crate::common::{Annotations, EntityType, Id};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EntityDecl {
    names: Vec<Id>,
    member_of: Vec<EntityType>,
    attributes: RecordType,
    tags: Option<Type>,
    annotations: Annotations,
}

impl EntityDecl {
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "TODO")]
    pub fn new(
        names: Vec<Id>,
        member_of: Vec<EntityType>,
        attributes: RecordType,
        tags: Option<Type>,
        annotations: Annotations,
    ) -> Self {
        assert!(
            !names.is_empty(),
            "entity declaration must have at least one name"
        );
        Self {
            names,
            member_of,
            attributes,
            tags,
            annotations,
        }
    }

    #[must_use]
    pub fn names(&self) -> &[Id] {
        &self.names
    }

    #[must_use]
    #[expect(clippy::indexing_slicing, reason = "TODO")]
    pub fn primary_name(&self) -> &Id {
        &self.names[0]
    }

    #[must_use]
    pub fn member_of(&self) -> &[EntityType] {
        &self.member_of
    }

    #[must_use]
    pub const fn attributes(&self) -> &RecordType {
        &self.attributes
    }

    #[must_use]
    pub const fn tags(&self) -> Option<&Type> {
        self.tags.as_ref()
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }
}
