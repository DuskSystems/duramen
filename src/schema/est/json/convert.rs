use alloc::borrow::ToOwned as _;
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
pub fn convert_to_json(schema: &SchemaFragment<'_>) -> SchemaFragmentJson {
    schema
        .iter()
        .map(|(namespace, def)| ((*namespace).to_owned(), convert_namespace_def(def)))
        .collect()
}

fn convert_namespace_def(def: &NamespaceDefinition<'_>) -> NamespaceDefinitionJson {
    NamespaceDefinitionJson {
        common_types: def
            .common_types
            .iter()
            .map(|(name, type_def)| ((*name).to_owned(), convert_type_def(type_def)))
            .collect(),
        entity_types: def
            .entity_types
            .iter()
            .map(|(name, entity)| ((*name).to_owned(), convert_entity_type(entity)))
            .collect(),
        actions: def
            .actions
            .iter()
            .map(|(name, action)| ((*name).to_owned(), convert_action_type(action)))
            .collect(),
        annotations: def
            .annotations
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect(),
    }
}

fn convert_entity_type(entity: &EntityType<'_>) -> EntityTypeJson {
    let shape = entity.shape.as_ref().and_then(|record| {
        if record.attributes.is_empty() {
            None
        } else {
            Some(convert_record_type(record))
        }
    });

    EntityTypeJson {
        enum_values: entity
            .enum_values
            .as_ref()
            .map(|values| values.iter().map(|v| (*v).to_owned()).collect()),
        annotations: entity
            .annotations
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect(),
        member_of_types: entity
            .member_of_types
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        shape,
        tags: entity.tags.as_ref().map(convert_type_def),
    }
}

fn convert_action_type(action: &ActionType<'_>) -> ActionTypeJson {
    ActionTypeJson {
        applies_to: action.applies_to.as_ref().map(convert_applies_to),
        annotations: action
            .annotations
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect(),
        member_of: action.member_of.iter().map(convert_action_ref).collect(),
    }
}

fn convert_applies_to(applies_to: &AppliesTo<'_>) -> AppliesToJson {
    let context = applies_to.context.as_ref().and_then(|ctx| {
        if let TypeDef::Record { record, .. } = ctx
            && record.attributes.is_empty()
        {
            return None;
        }
        Some(convert_context_type_def(ctx))
    });

    AppliesToJson {
        resource_types: applies_to
            .resource_types
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        principal_types: applies_to
            .principal_types
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        context,
    }
}

fn convert_context_type_def(type_def: &TypeDef<'_>) -> TypeDefJson {
    match type_def {
        TypeDef::EntityOrCommon {
            name,
            required,
            annotations,
        } => TypeDefJson::EntityOrCommon(EntityOrCommonJson {
            type_name: (*name).to_owned(),
            name: String::new(),
            annotations: annotations
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            required: *required,
        }),
        _ => convert_type_def(type_def),
    }
}

fn convert_action_ref(action_ref: &ActionRef<'_>) -> ActionRefJson {
    ActionRefJson {
        id: action_ref.id.to_owned(),
        ty: action_ref.ty.map(str::to_owned),
    }
}

fn convert_type_def(type_def: &TypeDef<'_>) -> TypeDefJson {
    match type_def {
        TypeDef::EntityOrCommon {
            name,
            required,
            annotations,
        } => TypeDefJson::EntityOrCommon(EntityOrCommonJson {
            type_name: String::from("EntityOrCommon"),
            name: (*name).to_owned(),
            annotations: annotations
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            required: *required,
        }),
        TypeDef::Set {
            element,
            required,
            annotations,
        } => TypeDefJson::Set(SetTypeJson {
            type_name: String::from("Set"),
            element: Box::new(convert_type_def(element)),
            annotations: annotations
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            required: *required,
        }),
        TypeDef::Record {
            record,
            required,
            annotations,
        } => TypeDefJson::Record(RecordTypeDefJson {
            type_name: String::from("Record"),
            attributes: convert_attributes(&record.attributes),
            annotations: annotations
                .iter()
                .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                .collect(),
            required: *required,
        }),
    }
}

fn convert_record_type(record: &RecordType<'_>) -> RecordTypeJson {
    RecordTypeJson {
        type_name: String::from("Record"),
        attributes: convert_attributes(&record.attributes),
    }
}

fn convert_attributes(attrs: &BTreeMap<&str, TypeDef<'_>>) -> BTreeMap<String, TypeDefJson> {
    attrs
        .iter()
        .map(|(key, value)| ((*key).to_owned(), convert_type_def(value)))
        .collect()
}
