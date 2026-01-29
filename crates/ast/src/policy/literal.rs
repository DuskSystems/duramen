use alloc::string::String;
use alloc::sync::Arc;

use super::integer::Integer;
use crate::common::EntityUid;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Literal {
    Bool(bool),
    Long(Integer),
    String(String),
    EntityUid(Arc<EntityUid>),
}

impl Literal {
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    #[must_use]
    pub const fn long(value: Integer) -> Self {
        Self::Long(value)
    }

    #[must_use]
    pub const fn string(value: String) -> Self {
        Self::String(value)
    }

    #[must_use]
    pub fn entity_uid(value: EntityUid) -> Self {
        Self::EntityUid(Arc::new(value))
    }

    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_long(&self) -> Option<Integer> {
        match self {
            Self::Long(long) => Some(*long),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_entity_uid(&self) -> Option<&EntityUid> {
        match self {
            Self::EntityUid(uid) => Some(uid),
            _ => None,
        }
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Integer> for Literal {
    fn from(value: Integer) -> Self {
        Self::Long(value)
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self::Long(Integer::new(value))
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
