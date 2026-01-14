use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! json_struct {
    ($name:ident { $field:ident : $ty:ty }) => {
        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[cfg_attr(feature = "facet", derive(facet::Facet))]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        pub struct $name {
            pub $field: $ty,
        }
    };

    ($name:ident { $field:ident : $ty:ty => $rename:literal }) => {
        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[cfg_attr(feature = "facet", derive(facet::Facet))]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        pub struct $name {
            #[cfg_attr(feature = "serde", serde(rename = $rename))]
            #[cfg_attr(feature = "facet", facet(rename = $rename))]
            pub $field: $ty,
        }
    };
}

macro_rules! binary_operation {
    ($name:ident, $field:ident) => {
        json_struct!($name { $field: BinaryArgumentJson });
    };

    ($name:ident, $field:ident, $rename:literal) => {
        json_struct!($name { $field: BinaryArgumentJson => $rename });
    };
}

macro_rules! unary_operation {
    ($name:ident, $field:ident) => {
        json_struct!($name { $field: UnaryArgumentJson });
    };

    ($name:ident, $field:ident, $rename:literal) => {
        json_struct!($name { $field: UnaryArgumentJson => $rename });
    };
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "facet", facet(rename_all = "camelCase"))]
pub struct PolicySetJson {
    pub templates: BTreeMap<String, PolicyJson>,
    pub static_policies: BTreeMap<String, PolicyJson>,
    pub template_links: Vec<TemplateLinkJson>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct TemplateLinkJson {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct PolicyJson {
    pub effect: String,
    pub principal: ScopeJson,
    pub action: ScopeJson,
    pub resource: ScopeJson,
    pub conditions: Vec<ConditionJson>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub annotations: Option<BTreeMap<String, Option<String>>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ScopeJson {
    pub op: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub entity: Option<EntityUidJson>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub entities: Option<Vec<EntityUidJson>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub slot: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none))]
    pub entity_type: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", rename = "in")
    )]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none, rename = "in"))]
    pub in_entity: Option<ScopeInJson>,
}

json_struct!(ScopeInJson {
    entity: EntityUidJson
});

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct EntityUidJson {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "facet", facet(rename = "type"))]
    pub entity_type: String,
    pub id: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ConditionJson {
    pub kind: String,
    pub body: ExpressionJson,
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
pub enum ExpressionJson {
    Value(ExpressionValueJson),
    Variable(ExpressionVariableJson),
    Slot(ExpressionSlotJson),
    Set(ExpressionSetJson),
    Record(ExpressionRecordJson),
    Not(ExpressionNotJson),
    Negate(ExpressionNegateJson),
    Or(ExpressionOrJson),
    And(ExpressionAndJson),
    Equal(ExpressionEqualJson),
    NotEqual(ExpressionNotEqualJson),
    LessThan(ExpressionLessThanJson),
    LessThanOrEqual(ExpressionLessThanOrEqualJson),
    GreaterThan(ExpressionGreaterThanJson),
    GreaterThanOrEqual(ExpressionGreaterThanOrEqualJson),
    In(ExpressionInJson),
    Add(ExpressionAddJson),
    Subtract(ExpressionSubtractJson),
    Multiply(ExpressionMultiplyJson),
    GetAttribute(ExpressionGetAttributeJson),
    HasAttribute(ExpressionHasAttributeJson),
    Index(ExpressionIndexJson),
    Like(ExpressionLikeJson),
    Is(ExpressionIsJson),
    IfThenElse(ExpressionIfJson),
    Contains(ExpressionContainsJson),
    ContainsAll(ExpressionContainsAllJson),
    ContainsAny(ExpressionContainsAnyJson),
    IsEmpty(ExpressionIsEmptyJson),
    HasTag(ExpressionHasTagJson),
    GetTag(ExpressionGetTagJson),
    ExtensionMethod(ExpressionExtensionMethodJson),
    ExtensionFunction(ExpressionExtensionFunctionJson),
}

json_struct!(ExpressionValueJson { value: ValueJson => "Value" });

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
pub enum ValueJson {
    Bool(bool),
    Int(i64),
    String(String),
    Entity(EntityValueJson),
}

json_struct!(EntityValueJson { entity: EntityUidJson => "__entity" });
json_struct!(ExpressionVariableJson { var: String => "Var" });
json_struct!(ExpressionSlotJson { slot: String => "Slot" });
json_struct!(ExpressionSetJson { set: Vec<ExpressionJson> => "Set" });
json_struct!(ExpressionRecordJson { record: BTreeMap<String, ExpressionJson> => "Record" });

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct UnaryArgumentJson {
    pub arg: Box<ExpressionJson>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct BinaryArgumentJson {
    pub left: Box<ExpressionJson>,
    pub right: Box<ExpressionJson>,
}

unary_operation!(ExpressionNotJson, not, "!");
unary_operation!(ExpressionNegateJson, neg);
unary_operation!(ExpressionIsEmptyJson, is_empty, "isEmpty");

binary_operation!(ExpressionOrJson, or, "||");
binary_operation!(ExpressionAndJson, and, "&&");
binary_operation!(ExpressionEqualJson, eq, "==");
binary_operation!(ExpressionNotEqualJson, neq, "!=");
binary_operation!(ExpressionLessThanJson, lt, "<");
binary_operation!(ExpressionLessThanOrEqualJson, lte, "<=");
binary_operation!(ExpressionGreaterThanJson, gt, ">");
binary_operation!(ExpressionGreaterThanOrEqualJson, gte, ">=");
binary_operation!(ExpressionInJson, in_op, "in");
binary_operation!(ExpressionAddJson, add, "+");
binary_operation!(ExpressionSubtractJson, sub, "-");
binary_operation!(ExpressionMultiplyJson, mul, "*");
binary_operation!(ExpressionContainsJson, contains);
binary_operation!(ExpressionContainsAllJson, contains_all, "containsAll");
binary_operation!(ExpressionContainsAnyJson, contains_any, "containsAny");
binary_operation!(ExpressionHasTagJson, has_tag, "hasTag");
binary_operation!(ExpressionGetTagJson, get_tag, "getTag");

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AttributeArgumentJson {
    pub left: Box<ExpressionJson>,
    pub attr: String,
}

json_struct!(ExpressionGetAttributeJson { get_attr: AttributeArgumentJson => "." });
json_struct!(ExpressionHasAttributeJson {
    has: AttributeArgumentJson
});

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct IndexArgumentJson {
    pub left: Box<ExpressionJson>,
    pub index: Box<ExpressionJson>,
}

json_struct!(ExpressionIndexJson { index: IndexArgumentJson => "[]" });

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct LikeArgumentJson {
    pub left: Box<ExpressionJson>,
    pub pattern: Vec<PatternElementJson>,
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
pub enum PatternElementJson {
    Wildcard(String),
    Literal(PatternLiteralJson),
}

json_struct!(PatternLiteralJson { literal: String => "Literal" });
json_struct!(ExpressionLikeJson {
    like: LikeArgumentJson
});

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct IsArgumentJson {
    pub left: Box<ExpressionJson>,
    pub entity_type: String,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", rename = "in")
    )]
    #[cfg_attr(feature = "facet", facet(skip_serializing_if = Option::is_none, rename = "in"))]
    pub in_expr: Option<Box<ExpressionJson>>,
}

json_struct!(ExpressionIsJson { is: IsArgumentJson });

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct IfArgumentJson {
    #[cfg_attr(feature = "serde", serde(rename = "if"))]
    #[cfg_attr(feature = "facet", facet(rename = "if"))]
    pub cond: Box<ExpressionJson>,
    #[cfg_attr(feature = "serde", serde(rename = "then"))]
    #[cfg_attr(feature = "facet", facet(rename = "then"))]
    pub then_expr: Box<ExpressionJson>,
    #[cfg_attr(feature = "serde", serde(rename = "else"))]
    #[cfg_attr(feature = "facet", facet(rename = "else"))]
    pub else_expr: Box<ExpressionJson>,
}

json_struct!(ExpressionIfJson { if_then_else: IfArgumentJson => "if-then-else" });

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "facet", facet(transparent))]
pub struct ExpressionExtensionMethodJson(pub BTreeMap<String, Vec<ExpressionJson>>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "facet", facet(transparent))]
pub struct ExpressionExtensionFunctionJson(pub BTreeMap<String, Vec<ExpressionJson>>);
