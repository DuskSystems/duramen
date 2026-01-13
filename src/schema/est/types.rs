use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub type SchemaFragment = BTreeMap<String, NamespaceDefinition>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NamespaceDefinition {
    pub annotations: BTreeMap<String, String>,
    pub common_types: BTreeMap<String, TypeDef>,
    pub entity_types: BTreeMap<String, EntityType>,
    pub actions: BTreeMap<String, ActionType>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EntityType {
    pub annotations: BTreeMap<String, String>,
    pub member_of_types: Vec<String>,
    pub shape: Option<RecordType>,
    pub tags: Option<TypeDef>,
    pub enum_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ActionType {
    pub annotations: BTreeMap<String, String>,
    pub member_of: Vec<ActionRef>,
    pub applies_to: Option<AppliesTo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRef {
    pub id: String,
    pub ty: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppliesTo {
    pub principal_types: Vec<String>,
    pub resource_types: Vec<String>,
    pub context: Option<TypeDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDef {
    EntityOrCommon {
        name: String,
        required: bool,
        annotations: BTreeMap<String, String>,
    },
    Set {
        element: Box<Self>,
        required: bool,
        annotations: BTreeMap<String, String>,
    },
    Record {
        record: RecordType,
        required: bool,
        annotations: BTreeMap<String, String>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecordType {
    pub attributes: BTreeMap<String, TypeDef>,
}
