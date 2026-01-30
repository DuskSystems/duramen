use alloc::vec;
use alloc::vec::Vec;

use duramen_ast as ast;

use crate::convert::{map_to_ast_annotations, string_to_name};
use crate::json::schema::{SchemaFragment, SchemaType, TypeOfAttribute};

fn type_of_attr_to_ast(name: &str, type_of_attr: &TypeOfAttribute) -> ast::schema::AttributeDecl {
    ast::schema::AttributeDecl::new(
        ast::common::Id::new(name.into()),
        type_of_attr.required,
        schema_type_to_ast_type(&type_of_attr.attr_type),
        map_to_ast_annotations(&type_of_attr.annotations),
    )
}

fn schema_type_to_ast_type(schema_type: &SchemaType) -> ast::schema::Type {
    match schema_type {
        SchemaType::Boolean => ast::schema::Type::bool(),
        SchemaType::Long => ast::schema::Type::long(),
        SchemaType::String => ast::schema::Type::string(),

        SchemaType::Set { element } => ast::schema::Type::set(schema_type_to_ast_type(element)),

        SchemaType::Record { attributes } => {
            let attrs: Vec<ast::schema::AttributeDecl> = attributes
                .iter()
                .map(|(name, type_of_attr)| type_of_attr_to_ast(name, type_of_attr))
                .collect();
            ast::schema::Type::Record(ast::schema::RecordType::new(attrs))
        }

        SchemaType::Entity { name } => {
            ast::schema::Type::entity(ast::common::EntityType::new(string_to_name(name)))
        }

        SchemaType::Extension { name } => ast::schema::Type::extension(string_to_name(name)),

        SchemaType::EntityOrCommon { name } => ast::schema::Type::named(string_to_name(name)),
    }
}

fn schema_type_to_record_type(schema_type: &SchemaType) -> ast::schema::RecordType {
    match schema_type {
        SchemaType::Record { attributes } => {
            let attrs: Vec<ast::schema::AttributeDecl> = attributes
                .iter()
                .map(|(name, type_of_attr)| type_of_attr_to_ast(name, type_of_attr))
                .collect();
            ast::schema::RecordType::new(attrs)
        }
        _ => ast::schema::RecordType::empty(),
    }
}

impl From<&SchemaFragment> for ast::schema::Schema {
    fn from(value: &SchemaFragment) -> Self {
        let namespaces: Vec<ast::schema::Namespace> = value
            .namespaces()
            .iter()
            .map(|(ns_name, ns_def)| {
                let name = if ns_name.is_empty() {
                    None
                } else {
                    Some(string_to_name(ns_name))
                };

                let types: Vec<ast::schema::TypeDecl> = ns_def
                    .common_types
                    .iter()
                    .map(|(type_name, schema_type)| {
                        ast::schema::TypeDecl::new(
                            ast::common::Id::new(type_name.clone()),
                            schema_type_to_ast_type(schema_type),
                            ast::common::Annotations::new(), // commonTypes don't have annotations in EST
                        )
                    })
                    .collect();

                let entities: Vec<ast::schema::EntityDecl> = ns_def
                    .entity_types
                    .iter()
                    .map(|(entity_name, entity_type)| {
                        let annotations = map_to_ast_annotations(&entity_type.annotations);

                        if entity_type.enum_values.is_empty() {
                            let member_of: Vec<ast::common::EntityType> = entity_type
                                .member_of_types
                                .iter()
                                .map(|s| ast::common::EntityType::new(string_to_name(s)))
                                .collect();

                            let attributes = entity_type.shape.as_ref().map_or_else(
                                ast::schema::RecordType::empty,
                                schema_type_to_record_type,
                            );

                            let tags = entity_type.tags.as_ref().map(schema_type_to_ast_type);

                            ast::schema::EntityDecl::standard(
                                vec![ast::common::Id::new(entity_name.clone())],
                                member_of,
                                attributes,
                                tags,
                                annotations,
                            )
                        } else {
                            let choices: Vec<ast::common::Id> = entity_type
                                .enum_values
                                .iter()
                                .map(|s| ast::common::Id::new(s.clone()))
                                .collect();
                            ast::schema::EntityDecl::enum_entity(
                                vec![ast::common::Id::new(entity_name.clone())],
                                choices,
                                annotations,
                            )
                        }
                    })
                    .collect();

                let actions: Vec<ast::schema::ActionDecl> = ns_def
                    .actions
                    .iter()
                    .map(|(action_name, action_type)| {
                        let member_of: Vec<ast::schema::ActionRef> = action_type
                            .member_of
                            .as_ref()
                            .map(|refs| {
                                refs.iter()
                                    .map(|action_ref| {
                                        let name = ast::common::Id::new(action_ref.id.clone());
                                        let groups: Vec<ast::common::Id> = action_ref
                                            .entity_type
                                            .as_ref()
                                            .map(|entity_type| {
                                                entity_type
                                                    .split("::")
                                                    .map(|s| ast::common::Id::new(s.into()))
                                                    .collect()
                                            })
                                            .unwrap_or_default();
                                        ast::schema::ActionRef::new(name, groups)
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        let applies_to = action_type.applies_to.as_ref().map_or_else(
                            ast::schema::AppliesTo::empty,
                            |at| {
                                let principals: Vec<ast::common::EntityType> = at
                                    .principal_types
                                    .as_ref()
                                    .map(|pts| {
                                        pts.iter()
                                            .map(|s| {
                                                ast::common::EntityType::new(string_to_name(s))
                                            })
                                            .collect()
                                    })
                                    .unwrap_or_default();

                                let resources: Vec<ast::common::EntityType> = at
                                    .resource_types
                                    .as_ref()
                                    .map(|rts| {
                                        rts.iter()
                                            .map(|s| {
                                                ast::common::EntityType::new(string_to_name(s))
                                            })
                                            .collect()
                                    })
                                    .unwrap_or_default();

                                let context = at.context.as_ref().map_or_else(
                                    ast::schema::RecordType::empty,
                                    schema_type_to_record_type,
                                );

                                ast::schema::AppliesTo::new(principals, resources, context)
                            },
                        );

                        let annotations = map_to_ast_annotations(&action_type.annotations);

                        ast::schema::ActionDecl::new(
                            vec![ast::common::Id::new(action_name.clone())],
                            member_of,
                            applies_to,
                            annotations,
                        )
                    })
                    .collect();

                let ns_annotations = map_to_ast_annotations(&ns_def.annotations);

                ast::schema::Namespace::new(name, entities, actions, types, ns_annotations)
            })
            .collect();

        Self::new(namespaces)
    }
}
