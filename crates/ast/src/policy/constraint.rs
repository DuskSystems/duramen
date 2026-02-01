use alloc::vec::Vec;

use crate::common::{EntityType, EntityUid};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum EntityReference {
    Euid(EntityUid),
    Slot,
}

impl EntityReference {
    #[must_use]
    pub const fn euid(uid: EntityUid) -> Self {
        Self::Euid(uid)
    }

    #[must_use]
    pub const fn slot() -> Self {
        Self::Slot
    }

    #[must_use]
    pub const fn as_euid(&self) -> Option<&EntityUid> {
        match self {
            Self::Euid(uid) => Some(uid),
            Self::Slot => None,
        }
    }

    #[must_use]
    pub const fn is_slot(&self) -> bool {
        matches!(self, Self::Slot)
    }

    #[must_use]
    pub const fn is_euid(&self) -> bool {
        matches!(self, Self::Euid(_))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum PrincipalOrResourceConstraint {
    #[default]
    Any,
    Eq(EntityReference),
    In(EntityReference),
    Is(EntityType),
    IsIn(EntityType, EntityReference),
}

impl PrincipalOrResourceConstraint {
    #[must_use]
    pub const fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    #[must_use]
    pub const fn has_slot(&self) -> bool {
        match self {
            Self::Eq(entity_ref) | Self::In(entity_ref) | Self::IsIn(_, entity_ref) => {
                entity_ref.is_slot()
            }
            Self::Any | Self::Is(_) => false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PrincipalConstraint(PrincipalOrResourceConstraint);

impl PrincipalConstraint {
    #[must_use]
    pub const fn new(constraint: PrincipalOrResourceConstraint) -> Self {
        Self(constraint)
    }

    #[must_use]
    pub const fn any() -> Self {
        Self(PrincipalOrResourceConstraint::Any)
    }

    #[must_use]
    pub const fn equal(entity: EntityReference) -> Self {
        Self(PrincipalOrResourceConstraint::Eq(entity))
    }

    #[must_use]
    pub const fn equal_slot() -> Self {
        Self(PrincipalOrResourceConstraint::Eq(EntityReference::Slot))
    }

    #[must_use]
    pub const fn is_in(entity: EntityReference) -> Self {
        Self(PrincipalOrResourceConstraint::In(entity))
    }

    #[must_use]
    pub const fn in_slot() -> Self {
        Self(PrincipalOrResourceConstraint::In(EntityReference::Slot))
    }

    #[must_use]
    pub const fn is(entity_type: EntityType) -> Self {
        Self(PrincipalOrResourceConstraint::Is(entity_type))
    }

    #[must_use]
    pub const fn is_in_type(entity_type: EntityType, entity: EntityReference) -> Self {
        Self(PrincipalOrResourceConstraint::IsIn(entity_type, entity))
    }

    #[must_use]
    pub const fn is_in_type_slot(entity_type: EntityType) -> Self {
        Self(PrincipalOrResourceConstraint::IsIn(
            entity_type,
            EntityReference::Slot,
        ))
    }

    #[must_use]
    pub const fn constraint(&self) -> &PrincipalOrResourceConstraint {
        &self.0
    }

    #[must_use]
    pub fn into_constraint(self) -> PrincipalOrResourceConstraint {
        self.0
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ResourceConstraint(PrincipalOrResourceConstraint);

impl ResourceConstraint {
    #[must_use]
    pub const fn new(constraint: PrincipalOrResourceConstraint) -> Self {
        Self(constraint)
    }

    #[must_use]
    pub const fn any() -> Self {
        Self(PrincipalOrResourceConstraint::Any)
    }

    #[must_use]
    pub const fn equal(entity: EntityReference) -> Self {
        Self(PrincipalOrResourceConstraint::Eq(entity))
    }

    #[must_use]
    pub const fn equal_slot() -> Self {
        Self(PrincipalOrResourceConstraint::Eq(EntityReference::Slot))
    }

    #[must_use]
    pub const fn is_in(entity: EntityReference) -> Self {
        Self(PrincipalOrResourceConstraint::In(entity))
    }

    #[must_use]
    pub const fn in_slot() -> Self {
        Self(PrincipalOrResourceConstraint::In(EntityReference::Slot))
    }

    #[must_use]
    pub const fn is(entity_type: EntityType) -> Self {
        Self(PrincipalOrResourceConstraint::Is(entity_type))
    }

    #[must_use]
    pub const fn is_in_type(entity_type: EntityType, entity: EntityReference) -> Self {
        Self(PrincipalOrResourceConstraint::IsIn(entity_type, entity))
    }

    #[must_use]
    pub const fn is_in_type_slot(entity_type: EntityType) -> Self {
        Self(PrincipalOrResourceConstraint::IsIn(
            entity_type,
            EntityReference::Slot,
        ))
    }

    #[must_use]
    pub const fn constraint(&self) -> &PrincipalOrResourceConstraint {
        &self.0
    }

    #[must_use]
    pub fn into_constraint(self) -> PrincipalOrResourceConstraint {
        self.0
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum ActionConstraint {
    #[default]
    Any,
    Eq(EntityUid),
    In(Vec<EntityUid>),
}

impl ActionConstraint {
    #[must_use]
    pub const fn any() -> Self {
        Self::Any
    }

    #[must_use]
    pub const fn equal(action: EntityUid) -> Self {
        Self::Eq(action)
    }

    #[must_use]
    pub const fn is_in(actions: Vec<EntityUid>) -> Self {
        Self::In(actions)
    }

    #[must_use]
    pub const fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }
}
