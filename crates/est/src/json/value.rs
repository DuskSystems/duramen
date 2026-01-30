//! Value types for EST serialization.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Reference to an entity by type and id.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityRef {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub entity_type: String,
    pub id: String,
}

impl EntityRef {
    #[must_use]
    pub fn new<S: Into<String>, T: Into<String>>(entity_type: S, id: T) -> Self {
        Self {
            entity_type: entity_type.into(),
            id: id.into(),
        }
    }
}

/// Entity value wrapper for JSON serialization.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityValue {
    #[cfg_attr(feature = "serde", serde(rename = "__entity"))]
    pub entity: EntityRef,
}

impl EntityValue {
    #[must_use]
    pub const fn new(entity: EntityRef) -> Self {
        Self { entity }
    }
}

/// Literal values that can appear in expressions.
///
/// Does NOT include Set/Record — those are expression constructs,
/// not literal values. This ensures EST → AST conversion is infallible.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LiteralValue {
    Bool(bool),
    Long(i64),
    String(String),
    Entity(EntityValue),
}

impl LiteralValue {
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    #[must_use]
    pub const fn long(value: i64) -> Self {
        Self::Long(value)
    }

    #[must_use]
    pub fn string<S: Into<String>>(value: S) -> Self {
        Self::String(value.into())
    }

    #[must_use]
    pub const fn entity(entity_ref: EntityRef) -> Self {
        Self::Entity(EntityValue::new(entity_ref))
    }
}

impl From<bool> for LiteralValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for LiteralValue {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<String> for LiteralValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for LiteralValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

/// Full value type for entity data (includes Set/Record).
///
/// Used for entity attributes, not expression literals.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Value {
    Bool(bool),
    Long(i64),
    String(String),
    Entity(EntityValue),
    Extension(ExtensionValue),
    Set(Vec<Self>),
    Record(BTreeMap<String, Self>),
}

/// Extension value wrapper for JSON serialization.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtensionValue {
    #[cfg_attr(feature = "serde", serde(rename = "__extn"))]
    pub extn: ExtensionCall,
}

/// Extension function call in a value context.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtensionCall {
    #[cfg_attr(feature = "serde", serde(rename = "fn"))]
    pub fn_name: String,
    pub arg: Box<Value>,
}

impl ExtensionCall {
    #[must_use]
    pub fn new<S: Into<String>>(fn_name: S, arg: Value) -> Self {
        Self {
            fn_name: fn_name.into(),
            arg: Box::new(arg),
        }
    }
}
