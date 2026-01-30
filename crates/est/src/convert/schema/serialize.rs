use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use duramen_ast as ast;

use crate::convert::{ast_annotations_to_map, entity_type_to_string, name_to_string};
use crate::json::schema::{
    ActionEntityUid, ActionType, AppliesTo, EntityType, NamespaceDefinition, SchemaFragment,
    SchemaType, TypeOfAttribute,
};

fn record_type_to_schema_type(record: &ast::schema::RecordType) -> SchemaType {
    let attributes: BTreeMap<String, TypeOfAttribute> = record
        .attributes()
        .iter()
        .map(|attr| {
            (
                attr.name().as_str().into(),
                TypeOfAttribute::new(
                    SchemaType::from(attr.attr_type()),
                    attr.is_required(),
                    ast_annotations_to_map(attr.annotations()),
                ),
            )
        })
        .collect();

    SchemaType::Record { attributes }
}

impl From<&ast::schema::Type> for SchemaType {
    fn from(value: &ast::schema::Type) -> Self {
        match value {
            ast::schema::Type::Primitive(prim) => match prim {
                ast::schema::PrimitiveType::Bool => Self::Boolean,
                ast::schema::PrimitiveType::Long => Self::Long,
                ast::schema::PrimitiveType::String => Self::String,
            },

            ast::schema::Type::Set(element) => Self::set(Self::from(element.as_ref())),

            ast::schema::Type::Record(record) => record_type_to_schema_type(record),

            ast::schema::Type::Entity(entity_type) => {
                Self::entity(entity_type_to_string(entity_type))
            }

            ast::schema::Type::Named(name) => Self::entity_or_common(name_to_string(name)),

            ast::schema::Type::Enum(_) => Self::String,

            ast::schema::Type::Extension(name) => Self::extension(name_to_string(name)),
        }
    }
}

impl From<&ast::schema::EntityDecl> for EntityType {
    fn from(value: &ast::schema::EntityDecl) -> Self {
        let annotations = ast_annotations_to_map(value.annotations());

        match value {
            ast::schema::EntityDecl::Standard(decl) => {
                let member_of_types: Vec<String> =
                    decl.member_of().iter().map(entity_type_to_string).collect();

                let shape = if decl.attributes().is_empty() {
                    None
                } else {
                    Some(record_type_to_schema_type(decl.attributes()))
                };

                let tags = decl.tags().map(SchemaType::from);

                Self {
                    member_of_types,
                    shape,
                    tags,
                    enum_values: Vec::new(),
                    annotations,
                }
            }
            ast::schema::EntityDecl::Enum(decl) => {
                let enum_values: Vec<String> =
                    decl.choices().iter().map(|id| id.as_str().into()).collect();

                Self {
                    member_of_types: Vec::new(),
                    shape: None,
                    tags: None,
                    enum_values,
                    annotations,
                }
            }
        }
    }
}

impl From<&ast::schema::ActionDecl> for ActionType {
    fn from(value: &ast::schema::ActionDecl) -> Self {
        let applies_to_ast = value.applies_to();

        let resource_types: Option<Vec<String>> = Some(
            applies_to_ast
                .resources()
                .iter()
                .map(entity_type_to_string)
                .collect(),
        );

        let principal_types: Option<Vec<String>> = Some(
            applies_to_ast
                .principals()
                .iter()
                .map(entity_type_to_string)
                .collect(),
        );

        let context = if applies_to_ast.context().is_empty() {
            None
        } else {
            Some(record_type_to_schema_type(applies_to_ast.context()))
        };

        let applies_to = Some(AppliesTo {
            resource_types,
            principal_types,
            context,
        });

        let member_of: Option<Vec<ActionEntityUid>> = if value.member_of().is_empty() {
            None
        } else {
            Some(
                value
                    .member_of()
                    .iter()
                    .map(|action_ref| {
                        let id: String = action_ref.name().as_str().into();
                        let entity_type = if action_ref.groups().is_empty() {
                            None
                        } else {
                            Some(
                                action_ref
                                    .groups()
                                    .iter()
                                    .map(ast::common::Id::as_str)
                                    .collect::<Vec<_>>()
                                    .join("::"),
                            )
                        };
                        ActionEntityUid::new(id, entity_type)
                    })
                    .collect(),
            )
        };

        Self {
            applies_to,
            member_of,
            annotations: ast_annotations_to_map(value.annotations()),
        }
    }
}

fn namespace_to_definition(namespace: &ast::schema::Namespace) -> (String, NamespaceDefinition) {
    let name = namespace.name().map(name_to_string).unwrap_or_default();

    let mut ns_def = NamespaceDefinition::new();

    for type_decl in namespace.types() {
        ns_def.common_types.insert(
            type_decl.name().as_str().into(),
            SchemaType::from(type_decl.type_def()),
        );
    }

    for entity_decl in namespace.entities() {
        for entity_name in entity_decl.names() {
            ns_def
                .entity_types
                .insert(entity_name.as_str().into(), EntityType::from(entity_decl));
        }
    }

    for action_decl in namespace.actions() {
        for action_name in action_decl.names() {
            ns_def
                .actions
                .insert(action_name.as_str().into(), ActionType::from(action_decl));
        }
    }

    ns_def.annotations = ast_annotations_to_map(namespace.annotations());

    (name, ns_def)
}

impl From<&ast::schema::Schema> for SchemaFragment {
    fn from(value: &ast::schema::Schema) -> Self {
        let mut fragment = Self::new();

        for namespace in value.namespaces() {
            let (name, ns_def) = namespace_to_definition(namespace);
            fragment.add_namespace(name, ns_def);
        }

        fragment
    }
}
