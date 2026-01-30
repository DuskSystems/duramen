mod policy;
mod schema;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use duramen_ast as ast;

use crate::json::value::EntityRef;

/// Converts AST annotations to a string map for JSON serialization.
pub(crate) fn ast_annotations_to_map(
    annotations: &ast::common::Annotations,
) -> BTreeMap<String, String> {
    annotations
        .iter()
        .map(|(key, ann)| (key.as_str().into(), ann.value().unwrap_or("").into()))
        .collect()
}

/// Converts a string map to AST annotations.
pub(crate) fn map_to_ast_annotations(map: &BTreeMap<String, String>) -> ast::common::Annotations {
    map.iter()
        .map(|(key, value)| {
            let annotation = if value.is_empty() {
                ast::common::Annotation::without_value()
            } else {
                ast::common::Annotation::with_value(value.clone())
            };
            (ast::common::AnyId::new(key.clone()), annotation)
        })
        .collect()
}

pub(crate) fn name_to_string(name: &ast::common::Name) -> String {
    let path = name.path();
    if path.is_empty() {
        name.basename().as_str().into()
    } else {
        let mut result = String::new();
        for segment in path {
            result.push_str(segment.as_str());
            result.push_str("::");
        }
        result.push_str(name.basename().as_str());
        result
    }
}

pub(crate) fn string_to_name(name: &str) -> ast::common::Name {
    let parts: Vec<&str> = name.split("::").collect();
    match parts.as_slice() {
        [] => ast::common::Name::unqualified(ast::common::Id::new(String::new())),
        [single] => ast::common::Name::unqualified(ast::common::Id::new((*single).into())),
        [path @ .., basename] => {
            let path_ids: Vec<ast::common::Id> = path
                .iter()
                .map(|s| ast::common::Id::new((*s).into()))
                .collect();
            ast::common::Name::new(path_ids, ast::common::Id::new((*basename).into()))
        }
    }
}

pub(crate) fn entity_uid_to_ref(uid: &ast::common::EntityUid) -> EntityRef {
    EntityRef::new(name_to_string(uid.entity_type().name()), uid.eid().as_str())
}

pub(crate) fn entity_type_to_string(entity_type: &ast::common::EntityType) -> String {
    name_to_string(entity_type.name())
}

pub(crate) fn entity_ref_to_uid(entity_ref: &EntityRef) -> ast::common::EntityUid {
    let entity_type = ast::common::EntityType::new(string_to_name(&entity_ref.entity_type));
    let eid = ast::common::Eid::new(entity_ref.id.clone());
    ast::common::EntityUid::new(entity_type, eid)
}
