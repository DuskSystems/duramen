use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SchemaFragment(pub BTreeMap<String, NamespaceDefinition>);

impl SchemaFragment {
    #[must_use]
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add_namespace(&mut self, name: String, ns: NamespaceDefinition) {
        self.0.insert(name, ns);
    }

    #[must_use]
    pub const fn namespaces(&self) -> &BTreeMap<String, NamespaceDefinition> {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct NamespaceDefinition {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub common_types: BTreeMap<String, SchemaType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub entity_types: BTreeMap<String, EntityType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub actions: BTreeMap<String, ActionType>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub annotations: BTreeMap<String, String>,
}

impl NamespaceDefinition {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct EntityType {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub member_of_types: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub shape: Option<SchemaType>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub tags: Option<SchemaType>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")
    )]
    pub enum_values: Vec<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub annotations: BTreeMap<String, String>,
}

impl EntityType {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ActionType {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub applies_to: Option<AppliesTo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub member_of: Option<Vec<ActionEntityUid>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub annotations: BTreeMap<String, String>,
}

impl ActionType {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AppliesTo {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resource_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub principal_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub context: Option<SchemaType>,
}

impl AppliesTo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionEntityUid {
    pub id: String,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "type", skip_serializing_if = "Option::is_none")
    )]
    pub entity_type: Option<String>,
}

impl ActionEntityUid {
    #[must_use]
    pub fn new<S: Into<String>>(id: S, entity_type: Option<String>) -> Self {
        Self {
            id: id.into(),
            entity_type,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "PascalCase"))]
#[repr(u8)]
pub enum SchemaType {
    String,
    Long,
    Boolean,
    Set {
        element: Box<Self>,
    },
    Record {
        attributes: BTreeMap<String, TypeOfAttribute>,
    },
    Entity {
        name: String,
    },
    Extension {
        name: String,
    },
    #[cfg_attr(feature = "serde", serde(rename = "EntityOrCommon"))]
    EntityOrCommon {
        name: String,
    },
}

impl SchemaType {
    #[must_use]
    pub const fn string() -> Self {
        Self::String
    }

    #[must_use]
    pub const fn long() -> Self {
        Self::Long
    }

    #[must_use]
    pub const fn boolean() -> Self {
        Self::Boolean
    }

    #[must_use]
    pub fn set(element: Self) -> Self {
        Self::Set {
            element: Box::new(element),
        }
    }

    #[must_use]
    pub const fn record(attributes: BTreeMap<String, TypeOfAttribute>) -> Self {
        Self::Record { attributes }
    }

    #[must_use]
    pub fn entity<S: Into<String>>(name: S) -> Self {
        Self::Entity { name: name.into() }
    }

    #[must_use]
    pub fn extension<S: Into<String>>(name: S) -> Self {
        Self::Extension { name: name.into() }
    }

    #[must_use]
    pub fn entity_or_common<S: Into<String>>(name: S) -> Self {
        Self::EntityOrCommon { name: name.into() }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeOfAttribute {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub attr_type: SchemaType,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub annotations: BTreeMap<String, String>,
    #[cfg_attr(
        feature = "serde",
        serde(default = "default_required", skip_serializing_if = "is_required")
    )]
    pub required: bool,
}

#[cfg(feature = "serde")]
const fn default_required() -> bool {
    true
}

#[cfg(feature = "serde")]
#[expect(clippy::trivially_copy_pass_by_ref, reason = "Required API")]
const fn is_required(value: &bool) -> bool {
    *value
}

impl TypeOfAttribute {
    #[must_use]
    pub const fn new(
        attr_type: SchemaType,
        required: bool,
        annotations: BTreeMap<String, String>,
    ) -> Self {
        Self {
            attr_type,
            annotations,
            required,
        }
    }

    #[must_use]
    pub const fn required(attr_type: SchemaType) -> Self {
        Self::new(attr_type, true, BTreeMap::new())
    }

    #[must_use]
    pub const fn optional(attr_type: SchemaType) -> Self {
        Self::new(attr_type, false, BTreeMap::new())
    }
}
