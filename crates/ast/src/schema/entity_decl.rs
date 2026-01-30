use alloc::vec::Vec;

use super::types::{RecordType, Type};
use crate::common::{Annotations, EntityType, Id};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EntityDecl {
    Standard(StandardEntityDecl),
    Enum(EnumEntityDecl),
}

impl EntityDecl {
    #[must_use]
    pub fn standard(
        names: Vec<Id>,
        member_of: Vec<EntityType>,
        attributes: RecordType,
        tags: Option<Type>,
        annotations: Annotations,
    ) -> Self {
        Self::Standard(StandardEntityDecl::new(
            names,
            member_of,
            attributes,
            tags,
            annotations,
        ))
    }

    #[must_use]
    pub fn enum_entity(names: Vec<Id>, choices: Vec<Id>, annotations: Annotations) -> Self {
        Self::Enum(EnumEntityDecl::new(names, choices, annotations))
    }

    #[must_use]
    pub fn names(&self) -> &[Id] {
        match self {
            Self::Standard(decl) => decl.names(),
            Self::Enum(decl) => decl.names(),
        }
    }

    #[must_use]
    pub fn primary_name(&self) -> &Id {
        match self {
            Self::Standard(decl) => decl.primary_name(),
            Self::Enum(decl) => decl.primary_name(),
        }
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        match self {
            Self::Standard(decl) => decl.annotations(),
            Self::Enum(decl) => decl.annotations(),
        }
    }

    #[must_use]
    pub const fn as_standard(&self) -> Option<&StandardEntityDecl> {
        match self {
            Self::Standard(decl) => Some(decl),
            Self::Enum(_) => None,
        }
    }

    #[must_use]
    pub const fn as_enum(&self) -> Option<&EnumEntityDecl> {
        match self {
            Self::Standard(_) => None,
            Self::Enum(decl) => Some(decl),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StandardEntityDecl {
    names: Vec<Id>,
    member_of: Vec<EntityType>,
    attributes: RecordType,
    tags: Option<Type>,
    annotations: Annotations,
}

impl StandardEntityDecl {
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct EnumEntityDecl {
    names: Vec<Id>,
    choices: Vec<Id>,
    annotations: Annotations,
}

impl EnumEntityDecl {
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "TODO")]
    pub fn new(names: Vec<Id>, choices: Vec<Id>, annotations: Annotations) -> Self {
        assert!(
            !names.is_empty(),
            "entity declaration must have at least one name"
        );

        Self {
            names,
            choices,
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
    pub fn choices(&self) -> &[Id] {
        &self.choices
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }
}
