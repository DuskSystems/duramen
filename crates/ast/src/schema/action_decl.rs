use alloc::vec::Vec;

use super::types::RecordType;
use crate::common::{Annotations, EntityType, Id};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ActionRef {
    name: Id,
    groups: Vec<Id>,
}

impl ActionRef {
    #[must_use]
    pub const fn new(name: Id, groups: Vec<Id>) -> Self {
        Self { name, groups }
    }

    #[must_use]
    pub const fn simple(name: Id) -> Self {
        Self {
            name,
            groups: Vec::new(),
        }
    }

    #[must_use]
    pub const fn name(&self) -> &Id {
        &self.name
    }

    #[must_use]
    pub fn groups(&self) -> &[Id] {
        &self.groups
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct AppliesTo {
    principals: Vec<EntityType>,
    resources: Vec<EntityType>,
    context: RecordType,
}

impl AppliesTo {
    #[must_use]
    pub const fn new(
        principals: Vec<EntityType>,
        resources: Vec<EntityType>,
        context: RecordType,
    ) -> Self {
        Self {
            principals,
            resources,
            context,
        }
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            principals: Vec::new(),
            resources: Vec::new(),
            context: RecordType::empty(),
        }
    }

    #[must_use]
    pub fn principals(&self) -> &[EntityType] {
        &self.principals
    }

    #[must_use]
    pub fn resources(&self) -> &[EntityType] {
        &self.resources
    }

    #[must_use]
    pub const fn context(&self) -> &RecordType {
        &self.context
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ActionDecl {
    names: Vec<Id>,
    member_of: Vec<ActionRef>,
    applies_to: AppliesTo,
    annotations: Annotations,
}

impl ActionDecl {
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "TODO")]
    pub fn new(
        names: Vec<Id>,
        member_of: Vec<ActionRef>,
        applies_to: AppliesTo,
        annotations: Annotations,
    ) -> Self {
        assert!(
            !names.is_empty(),
            "action declaration must have at least one name"
        );
        Self {
            names,
            member_of,
            applies_to,
            annotations,
        }
    }

    #[must_use]
    pub fn names(&self) -> &[Id] {
        &self.names
    }

    #[must_use]
    pub fn primary_name(&self) -> &Id {
        &self.names[0]
    }

    #[must_use]
    pub fn member_of(&self) -> &[ActionRef] {
        &self.member_of
    }

    #[must_use]
    pub const fn applies_to(&self) -> &AppliesTo {
        &self.applies_to
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }
}
