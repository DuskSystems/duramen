use alloc::string::String;

use super::name::Name;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Eid(String);

impl Eid {
    #[must_use]
    pub const fn new(eid: String) -> Self {
        Self(eid)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Eid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EntityType(Name);

impl EntityType {
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    #[must_use]
    pub const fn name(&self) -> &Name {
        &self.0
    }

    #[must_use]
    pub fn into_name(self) -> Name {
        self.0
    }
}

impl From<Name> for EntityType {
    fn from(value: Name) -> Self {
        Self(value)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EntityUid {
    entity_type: EntityType,
    eid: Eid,
}

impl EntityUid {
    #[must_use]
    pub const fn new(entity_type: EntityType, eid: Eid) -> Self {
        Self { entity_type, eid }
    }

    #[must_use]
    pub const fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }

    #[must_use]
    pub const fn eid(&self) -> &Eid {
        &self.eid
    }

    #[must_use]
    pub fn into_parts(self) -> (EntityType, Eid) {
        (self.entity_type, self.eid)
    }
}
