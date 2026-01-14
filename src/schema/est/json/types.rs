use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub type SchemaFragmentJson = BTreeMap<String, NamespaceDefinitionJson>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NamespaceDefinitionJson {
    #[cfg_attr(feature = "serde", serde(rename = "commonTypes"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(rename = "commonTypes"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub common_types: BTreeMap<String, TypeDefJson>,

    #[cfg_attr(feature = "serde", serde(rename = "entityTypes"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(rename = "entityTypes"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub entity_types: BTreeMap<String, EntityTypeJson>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub actions: BTreeMap<String, ActionTypeJson>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EntityTypeJson {
    #[cfg_attr(feature = "serde", serde(rename = "enum"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(rename = "enum"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub enum_values: Option<Vec<String>>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub annotations: BTreeMap<String, String>,

    #[cfg_attr(feature = "serde", serde(rename = "memberOfTypes"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(feature = "facet", facet(rename = "memberOfTypes"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Vec::is_empty))]
    pub member_of_types: Vec<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub shape: Option<RecordTypeJson>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub tags: Option<TypeDefJson>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ActionTypeJson {
    #[cfg_attr(feature = "serde", serde(rename = "appliesTo"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(rename = "appliesTo"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub applies_to: Option<AppliesToJson>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub annotations: BTreeMap<String, String>,

    #[cfg_attr(feature = "serde", serde(rename = "memberOf"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(feature = "facet", facet(rename = "memberOf"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Vec::is_empty))]
    pub member_of: Vec<ActionRefJson>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AppliesToJson {
    #[cfg_attr(feature = "serde", serde(rename = "resourceTypes"))]
    #[cfg_attr(feature = "facet", facet(rename = "resourceTypes"))]
    pub resource_types: Vec<String>,

    #[cfg_attr(feature = "serde", serde(rename = "principalTypes"))]
    #[cfg_attr(feature = "facet", facet(rename = "principalTypes"))]
    pub principal_types: Vec<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub context: Option<TypeDefJson>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ActionRefJson {
    pub id: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(rename = "type"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub ty: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "facet", facet(untagged))]
#[repr(u8)]
#[cfg_attr(
    all(feature = "facet", not(feature = "serde")),
    expect(dead_code, reason = "fields used by facet via reflection")
)]
pub enum TypeDefJson {
    EntityOrCommon(EntityOrCommonJson),
    Set(SetTypeJson),
    Record(RecordTypeDefJson),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EntityOrCommonJson {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "facet", facet(rename = "type"))]
    pub type_name: String,

    pub name: String,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub annotations: BTreeMap<String, String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "is_true"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = is_true))]
    pub required: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct SetTypeJson {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "facet", facet(rename = "type"))]
    pub type_name: String,

    pub element: Box<TypeDefJson>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub annotations: BTreeMap<String, String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "is_true"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = is_true))]
    pub required: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct RecordTypeDefJson {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "facet", facet(rename = "type"))]
    pub type_name: String,

    pub attributes: BTreeMap<String, TypeDefJson>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "BTreeMap::is_empty"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = BTreeMap::is_empty))]
    pub annotations: BTreeMap<String, String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "is_true"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = is_true))]
    pub required: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct RecordTypeJson {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "facet", facet(rename = "type"))]
    pub type_name: String,

    pub attributes: BTreeMap<String, TypeDefJson>,
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Required by skip_serializing_if"
)]
const fn is_true(value: &bool) -> bool {
    *value
}
