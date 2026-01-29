use alloc::sync::Arc;
use alloc::vec::Vec;

use super::attribute::AttributeDecl;
use crate::common::{EntityType, Id, Name};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveType {
    Bool,
    Long,
    String,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct RecordType {
    attributes: Vec<AttributeDecl>,
}

impl RecordType {
    #[must_use]
    pub const fn new(attributes: Vec<AttributeDecl>) -> Self {
        Self { attributes }
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    #[must_use]
    pub fn attributes(&self) -> &[AttributeDecl] {
        &self.attributes
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.attributes.len()
    }

    #[must_use]
    pub fn into_attributes(self) -> Vec<AttributeDecl> {
        self.attributes
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[expect(clippy::len_without_is_empty, reason = "TODO")]
pub struct EnumType {
    variants: Vec<Id>,
}

impl EnumType {
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "TODO")]
    pub fn new(variants: Vec<Id>) -> Self {
        assert!(!variants.is_empty(), "enum must have at least one variant");
        Self { variants }
    }

    #[must_use]
    pub fn variants(&self) -> &[Id] {
        &self.variants
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.variants.len()
    }

    #[must_use]
    pub fn into_variants(self) -> Vec<Id> {
        self.variants
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    Set(Arc<Self>),
    Record(RecordType),
    Entity(EntityType),
    Named(Name),
    Enum(EnumType),
    Extension(Name),
}

impl Type {
    #[must_use]
    pub const fn primitive(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }

    #[must_use]
    pub const fn bool() -> Self {
        Self::Primitive(PrimitiveType::Bool)
    }

    #[must_use]
    pub const fn long() -> Self {
        Self::Primitive(PrimitiveType::Long)
    }

    #[must_use]
    pub const fn string() -> Self {
        Self::Primitive(PrimitiveType::String)
    }

    #[must_use]
    pub fn set(element: Self) -> Self {
        Self::Set(Arc::new(element))
    }

    #[must_use]
    pub const fn record(attributes: Vec<AttributeDecl>) -> Self {
        Self::Record(RecordType::new(attributes))
    }

    #[must_use]
    pub const fn empty_record() -> Self {
        Self::Record(RecordType::empty())
    }

    #[must_use]
    pub const fn entity(entity_type: EntityType) -> Self {
        Self::Entity(entity_type)
    }

    #[must_use]
    pub const fn named(name: Name) -> Self {
        Self::Named(name)
    }

    #[must_use]
    pub fn enum_type(variants: Vec<Id>) -> Self {
        Self::Enum(EnumType::new(variants))
    }

    #[must_use]
    pub const fn extension(name: Name) -> Self {
        Self::Extension(name)
    }

    #[must_use]
    pub const fn is_primitive(&self) -> bool {
        matches!(self, Self::Primitive(_))
    }

    #[must_use]
    pub const fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    #[must_use]
    pub const fn is_record(&self) -> bool {
        matches!(self, Self::Record(_))
    }

    #[must_use]
    pub fn as_set(&self) -> Option<&Self> {
        match self {
            Self::Set(element) => Some(element),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_record(&self) -> Option<&RecordType> {
        match self {
            Self::Record(record) => Some(record),
            _ => None,
        }
    }
}
