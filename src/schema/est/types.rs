use alloc::collections::BTreeMap;

use bumpalo::collections::Vec as BumpVec;

pub type SchemaFragment<'a> = BTreeMap<&'a str, NamespaceDefinition<'a>>;

#[derive(Debug, PartialEq, Default)]
pub struct NamespaceDefinition<'a> {
    pub annotations: BTreeMap<&'a str, &'a str>,
    pub common_types: BTreeMap<&'a str, TypeDef<'a>>,
    pub entity_types: BTreeMap<&'a str, EntityType<'a>>,
    pub actions: BTreeMap<&'a str, ActionType<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct EntityType<'a> {
    pub annotations: BTreeMap<&'a str, &'a str>,
    pub member_of_types: BumpVec<'a, &'a str>,
    pub shape: Option<RecordType<'a>>,
    pub tags: Option<TypeDef<'a>>,
    pub enum_values: Option<BumpVec<'a, &'a str>>,
}

#[derive(Debug, PartialEq)]
pub struct ActionType<'a> {
    pub annotations: BTreeMap<&'a str, &'a str>,
    pub member_of: BumpVec<'a, ActionRef<'a>>,
    pub applies_to: Option<AppliesTo<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRef<'a> {
    pub id: &'a str,
    pub ty: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct AppliesTo<'a> {
    pub principal_types: BumpVec<'a, &'a str>,
    pub resource_types: BumpVec<'a, &'a str>,
    pub context: Option<TypeDef<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDef<'a> {
    EntityOrCommon {
        name: &'a str,
        required: bool,
        annotations: BTreeMap<&'a str, &'a str>,
    },
    Set {
        element: &'a Self,
        required: bool,
        annotations: BTreeMap<&'a str, &'a str>,
    },
    Record {
        record: RecordType<'a>,
        required: bool,
        annotations: BTreeMap<&'a str, &'a str>,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecordType<'a> {
    pub attributes: BTreeMap<&'a str, TypeDef<'a>>,
}
