use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;

use super::types::{
    ActionRefJson, ActionTypeJson, AppliesToJson, EntityOrCommonJson, EntityTypeJson,
    NamespaceDefinitionJson, RecordTypeDefJson, RecordTypeJson, SchemaFragmentJson, SetTypeJson,
    TypeDefJson,
};
use crate::schema::est::types::{
    ActionRef, ActionType, AppliesTo, EntityType, NamespaceDefinition, RecordType, SchemaFragment,
    TypeDef,
};

#[must_use]
pub fn convert_to_json(schema: &SchemaFragment) -> SchemaFragmentJson {
    schema
        .iter()
        .map(|(namespace, def)| (namespace.clone(), convert_namespace_def(def)))
        .collect()
}

fn convert_namespace_def(def: &NamespaceDefinition) -> NamespaceDefinitionJson {
    NamespaceDefinitionJson {
        common_types: def
            .common_types
            .iter()
            .map(|(name, type_def)| (name.clone(), convert_type_def(type_def)))
            .collect(),
        entity_types: def
            .entity_types
            .iter()
            .map(|(name, entity)| (name.clone(), convert_entity_type(entity)))
            .collect(),
        actions: def
            .actions
            .iter()
            .map(|(name, action)| (name.clone(), convert_action_type(action)))
            .collect(),
        annotations: def.annotations.clone(),
    }
}

fn convert_entity_type(entity: &EntityType) -> EntityTypeJson {
    EntityTypeJson {
        enum_values: entity.enum_values.clone(),
        annotations: entity.annotations.clone(),
        member_of_types: entity.member_of_types.clone(),
        shape: entity.shape.as_ref().map(convert_record_type),
        tags: entity.tags.as_ref().map(convert_type_def),
    }
}

fn convert_action_type(action: &ActionType) -> ActionTypeJson {
    ActionTypeJson {
        applies_to: action.applies_to.as_ref().map(convert_applies_to),
        annotations: action.annotations.clone(),
        member_of: action.member_of.iter().map(convert_action_ref).collect(),
    }
}

fn convert_applies_to(applies_to: &AppliesTo) -> AppliesToJson {
    let context = applies_to.context.as_ref().and_then(|ctx| {
        if let TypeDef::Record { record, .. } = ctx
            && record.attributes.is_empty()
        {
            return None;
        }
        Some(convert_type_def(ctx))
    });

    AppliesToJson {
        resource_types: applies_to.resource_types.clone(),
        principal_types: applies_to.principal_types.clone(),
        context,
    }
}

fn convert_action_ref(action_ref: &ActionRef) -> ActionRefJson {
    ActionRefJson {
        id: action_ref.id.clone(),
        ty: action_ref.ty.clone(),
    }
}

fn convert_type_def(type_def: &TypeDef) -> TypeDefJson {
    match type_def {
        TypeDef::EntityOrCommon {
            name,
            required,
            annotations,
        } => TypeDefJson::EntityOrCommon(EntityOrCommonJson {
            type_name: String::from("EntityOrCommon"),
            name: name.clone(),
            annotations: annotations.clone(),
            required: *required,
        }),
        TypeDef::Set {
            element,
            required,
            annotations,
        } => TypeDefJson::Set(SetTypeJson {
            type_name: String::from("Set"),
            element: Box::new(convert_type_def(element)),
            annotations: annotations.clone(),
            required: *required,
        }),
        TypeDef::Record {
            record,
            required,
            annotations,
        } => TypeDefJson::Record(RecordTypeDefJson {
            type_name: String::from("Record"),
            attributes: convert_attributes(&record.attributes),
            annotations: annotations.clone(),
            required: *required,
        }),
    }
}

fn convert_record_type(record: &RecordType) -> RecordTypeJson {
    RecordTypeJson {
        type_name: String::from("Record"),
        attributes: convert_attributes(&record.attributes),
    }
}

fn convert_attributes(attrs: &BTreeMap<String, TypeDef>) -> BTreeMap<String, TypeDefJson> {
    attrs
        .iter()
        .map(|(key, value)| (key.clone(), convert_type_def(value)))
        .collect()
}
