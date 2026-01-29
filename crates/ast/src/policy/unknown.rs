use alloc::string::String;

use crate::common::{EntityType, Name};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TypeHint {
    Bool,
    Long,
    String,
    Set,
    Record,
    Entity(EntityType),
    Extension(Name),
}

impl TypeHint {
    #[must_use]
    pub const fn bool() -> Self {
        Self::Bool
    }

    #[must_use]
    pub const fn long() -> Self {
        Self::Long
    }

    #[must_use]
    pub const fn string() -> Self {
        Self::String
    }

    #[must_use]
    pub const fn set() -> Self {
        Self::Set
    }

    #[must_use]
    pub const fn record() -> Self {
        Self::Record
    }

    #[must_use]
    pub const fn entity(entity_type: EntityType) -> Self {
        Self::Entity(entity_type)
    }

    #[must_use]
    pub const fn extension(name: Name) -> Self {
        Self::Extension(name)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Unknown {
    name: String,
    type_hint: Option<TypeHint>,
}

impl Unknown {
    #[must_use]
    pub const fn new(name: String, type_hint: Option<TypeHint>) -> Self {
        Self { name, type_hint }
    }

    #[must_use]
    pub const fn untyped(name: String) -> Self {
        Self {
            name,
            type_hint: None,
        }
    }

    #[must_use]
    pub const fn typed(name: String, type_hint: TypeHint) -> Self {
        Self {
            name,
            type_hint: Some(type_hint),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn type_hint(&self) -> Option<&TypeHint> {
        self.type_hint.as_ref()
    }

    #[must_use]
    pub const fn has_type_hint(&self) -> bool {
        self.type_hint.is_some()
    }

    #[must_use]
    pub fn into_parts(self) -> (String, Option<TypeHint>) {
        (self.name, self.type_hint)
    }
}
