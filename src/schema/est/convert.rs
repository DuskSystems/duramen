use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem;

use super::types::{
    ActionRef, ActionType, AppliesTo, EntityType, NamespaceDefinition, RecordType, SchemaFragment,
    TypeDef,
};
use crate::schema::ast::{
    self, ActionDeclaration, AstNode as _, AstToken as _, EntityDeclaration, Namespace, SchemaNode,
    TypeDeclaration, TypeExpr,
};
use crate::schema::{Schema, SchemaSyntax};

#[must_use]
pub fn convert_schema(schema: &Schema<'_>) -> SchemaFragment {
    let mut result = SchemaFragment::new();
    let source = schema.source();

    if let Some(root) = schema.root() {
        let mut pending_annotations: Vec<ast::Annotation<'_>> = Vec::new();
        let mut top_level_namespace_def: Option<NamespaceDefinition> = None;

        for child in root.syntax().children() {
            match child.value() {
                SchemaSyntax::Annotation => {
                    if let Some(ann) = ast::Annotation::cast(child) {
                        pending_annotations.push(ann);
                    }
                }
                SchemaSyntax::Namespace => {
                    if let Some(namespace) = Namespace::cast(child) {
                        let name = namespace_name(source, &namespace);
                        let mut namespace_def =
                            convert_node_children(source, namespace.syntax(), &name);
                        namespace_def.annotations = collect_annotations(
                            source,
                            mem::take(&mut pending_annotations).into_iter(),
                        );
                        result.insert(name, namespace_def);
                    }
                }
                SchemaSyntax::EntityDeclaration
                | SchemaSyntax::ActionDeclaration
                | SchemaSyntax::CommonTypeDeclaration => {
                    if top_level_namespace_def.is_none() {
                        top_level_namespace_def =
                            Some(convert_node_children(source, root.syntax(), ""));
                    }
                    pending_annotations.clear();
                }
                _ => {}
            }
        }

        if let Some(namespace_def) = top_level_namespace_def {
            result.insert(String::new(), namespace_def);
        }
    }

    result
}

fn namespace_name(source: &str, namespace: &Namespace<'_>) -> String {
    namespace
        .name()
        .map(|name| name_to_string(source, &name))
        .unwrap_or_default()
}

fn convert_node_children(
    source: &str,
    node: &SchemaNode<'_>,
    namespace_path: &str,
) -> NamespaceDefinition {
    let mut def = NamespaceDefinition::default();
    let mut pending_annotations: Vec<ast::Annotation<'_>> = Vec::new();

    for child in node.children() {
        match child.value() {
            SchemaSyntax::Annotation => {
                if let Some(ann) = ast::Annotation::cast(child) {
                    pending_annotations.push(ann);
                }
            }
            SchemaSyntax::EntityDeclaration => {
                if let Some(entity) = EntityDeclaration::cast(child) {
                    let annotations = collect_annotations(
                        source,
                        mem::take(&mut pending_annotations).into_iter(),
                    );
                    for (name, mut entity_type) in convert_entity_declaration(source, &entity) {
                        for (key, value) in &annotations {
                            entity_type.annotations.insert(key.clone(), value.clone());
                        }
                        def.entity_types.insert(name, entity_type);
                    }
                }
            }
            SchemaSyntax::ActionDeclaration => {
                if let Some(action) = ActionDeclaration::cast(child) {
                    let annotations = collect_annotations(
                        source,
                        mem::take(&mut pending_annotations).into_iter(),
                    );
                    for (name, mut action_type) in
                        convert_action_declaration(source, &action, namespace_path)
                    {
                        for (key, value) in &annotations {
                            action_type.annotations.insert(key.clone(), value.clone());
                        }
                        def.actions.insert(name, action_type);
                    }
                }
            }
            SchemaSyntax::CommonTypeDeclaration => {
                if let Some(type_decl) = TypeDeclaration::cast(child) {
                    pending_annotations.clear();
                    if let Some((name, type_def)) = convert_type_declaration(source, &type_decl) {
                        def.common_types.insert(name, type_def);
                    }
                }
            }
            _ => {}
        }
    }

    def
}

fn convert_entity_declaration(
    source: &str,
    entity: &EntityDeclaration<'_>,
) -> Vec<(String, EntityType)> {
    let mut results = Vec::new();
    let annotations = collect_annotations(source, entity.annotations());

    let member_of_types: Vec<String> = entity
        .parents()
        .and_then(|parents| parents.type_list())
        .map(|type_list| {
            type_list
                .types()
                .map(|name| name_to_string(source, &name))
                .collect()
        })
        .unwrap_or_default();

    let shape = entity
        .attributes()
        .and_then(|attrs| attrs.record_type())
        .map(|record| convert_record_type(source, &record));

    let tags = entity
        .tags()
        .and_then(|tags| tags.tag_type())
        .map(|type_expr| convert_type_expr(source, &type_expr, true, BTreeMap::new()));

    let enum_values = entity.enum_type().map(|enum_type| {
        enum_type
            .variants()
            .map(|variant| unescape_string(variant.text(source)))
            .collect()
    });

    let entity_type = EntityType {
        annotations,
        member_of_types,
        shape,
        tags,
        enum_values,
    };

    for name in extract_declaration_names(source, entity.syntax()) {
        results.push((name, entity_type.clone()));
    }

    results
}

fn convert_action_declaration(
    source: &str,
    action: &ActionDeclaration<'_>,
    namespace_path: &str,
) -> Vec<(String, ActionType)> {
    let mut results = Vec::new();
    let annotations = collect_annotations(source, action.annotations());

    let member_of: Vec<ActionRef> = action
        .parents()
        .map(|parents| {
            parents
                .names()
                .map(|name| parse_action_ref(source, &name))
                .collect()
        })
        .unwrap_or_default();

    let applies_to = action
        .applies_to()
        .map(|applies| {
            let principal_types = applies
                .principal_types()
                .and_then(|pt| pt.type_list())
                .map(|type_list| {
                    type_list
                        .types()
                        .map(|name| qualify_entity_name(source, &name, namespace_path))
                        .collect()
                })
                .unwrap_or_default();

            let resource_types = applies
                .resource_types()
                .and_then(|rt| rt.type_list())
                .map(|type_list| {
                    type_list
                        .types()
                        .map(|name| qualify_entity_name(source, &name, namespace_path))
                        .collect()
                })
                .unwrap_or_default();

            let context = applies
                .context_type()
                .and_then(|ctx| ctx.type_expr())
                .map(|type_expr| convert_type_expr(source, &type_expr, true, BTreeMap::new()));

            AppliesTo {
                principal_types,
                resource_types,
                context,
            }
        })
        .or_else(|| Some(AppliesTo::default()));

    let action_type = ActionType {
        annotations,
        member_of,
        applies_to,
    };

    for name in extract_declaration_names(source, action.syntax()) {
        results.push((name, action_type.clone()));
    }

    results
}

fn convert_type_declaration(
    source: &str,
    type_decl: &TypeDeclaration<'_>,
) -> Option<(String, TypeDef)> {
    let name = type_decl.name()?.text(source).to_owned();
    let definition = type_decl.definition()?;
    let type_def = convert_type_expr(source, &definition, true, BTreeMap::new());
    Some((name, type_def))
}

fn convert_type_expr(
    source: &str,
    type_expr: &TypeExpr<'_>,
    required: bool,
    annotations: BTreeMap<String, String>,
) -> TypeDef {
    match type_expr {
        TypeExpr::Name(name) => TypeDef::EntityOrCommon {
            name: name_to_string(source, name),
            required,
            annotations,
        },
        TypeExpr::Set(set_type) => {
            let element = set_type.element().map_or_else(
                || TypeDef::EntityOrCommon {
                    name: String::from("Unknown"),
                    required: true,
                    annotations: BTreeMap::new(),
                },
                |elem| convert_type_expr(source, &elem, true, BTreeMap::new()),
            );
            TypeDef::Set {
                element: Box::new(element),
                required,
                annotations,
            }
        }
        TypeExpr::Record(record) => TypeDef::Record {
            record: convert_record_type(source, record),
            required,
            annotations,
        },
        TypeExpr::Entity(entity_ref) => TypeDef::EntityOrCommon {
            name: entity_ref
                .name()
                .map(|name| name_to_string(source, &name))
                .unwrap_or_default(),
            required,
            annotations,
        },
    }
}

fn convert_record_type(source: &str, record: &ast::RecordType<'_>) -> RecordType {
    let mut attributes = BTreeMap::new();
    let mut pending_annotations: Vec<ast::Annotation<'_>> = Vec::new();

    for child in record.syntax().children() {
        match child.value() {
            SchemaSyntax::Annotation => {
                if let Some(ann) = ast::Annotation::cast(child) {
                    pending_annotations.push(ann);
                }
            }
            SchemaSyntax::AttributeDeclaration => {
                if let Some(attr) = ast::AttributeDeclaration::cast(child) {
                    let annotations = collect_annotations(
                        source,
                        mem::take(&mut pending_annotations).into_iter(),
                    );

                    if let Some(key) = attr.name() {
                        let key_str = match key {
                            ast::AttrKey::Identifier(ident) => ident.text(source).to_owned(),
                            ast::AttrKey::String(string) => unescape_string(string.text(source)),
                            ast::AttrKey::Keyword(node) => source[node.span().range()].to_owned(),
                        };

                        let required = !attr.is_optional();
                        if let Some(type_expr) = attr.attribute_type() {
                            let type_def =
                                convert_type_expr(source, &type_expr, required, annotations);
                            attributes.insert(key_str, type_def);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    RecordType { attributes }
}

fn collect_annotations<'a>(
    source: &str,
    annotations: impl Iterator<Item = ast::Annotation<'a>>,
) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();

    for annotation in annotations {
        let name_text = annotation.syntax().children().find_map(|child| {
            let kind = child.value();
            if kind == SchemaSyntax::Identifier || kind.is_keyword() {
                Some(&source[child.span().range()])
            } else {
                None
            }
        });

        if let Some(key) = name_text {
            let value = annotation
                .value()
                .map(|value| {
                    let text = value.text(source);
                    unescape_string(text)
                })
                .unwrap_or_default();
            result.insert(key.to_owned(), value);
        }
    }

    result
}

fn name_to_string(source: &str, name: &ast::Name<'_>) -> String {
    let node = name.syntax();
    let mut parts = Vec::new();

    for child in node.children() {
        let kind = child.value();
        match kind {
            SchemaSyntax::Identifier
            | SchemaSyntax::BoolKeyword
            | SchemaSyntax::LongKeyword
            | SchemaSyntax::StringKeyword => {
                let text = &source[child.span().range()];
                parts.push(text);
            }
            _ => {}
        }
    }

    parts.join("::")
}

fn qualify_entity_name(source: &str, name: &ast::Name<'_>, _namespace_path: &str) -> String {
    name_to_string(source, name)
}

fn parse_action_ref(source: &str, name: &ast::Name<'_>) -> ActionRef {
    let node = name.syntax();
    let mut identifiers = Vec::new();
    let mut action_id = None;

    for child in node.children() {
        match child.value() {
            SchemaSyntax::Identifier => {
                let text = &source[child.span().range()];
                identifiers.push(text);
            }
            SchemaSyntax::String => {
                let text = &source[child.span().range()];
                action_id = Some(unescape_string(text));
            }
            _ => {}
        }
    }

    match (action_id, identifiers.as_slice()) {
        (Some(id), []) => ActionRef { id, ty: None },
        (Some(id), parts) => ActionRef {
            id,
            ty: Some(parts.join("::")),
        },
        (None, []) => ActionRef {
            id: String::new(),
            ty: None,
        },
        (None, [single]) => ActionRef {
            id: (*single).to_owned(),
            ty: None,
        },
        (None, [init @ .., last]) => ActionRef {
            id: (*last).to_owned(),
            ty: Some(init.join("::")),
        },
    }
}

fn strip_quotes(text: &str) -> Option<&str> {
    let inner = text.strip_prefix('"')?;
    if let Some(stripped) = inner.strip_suffix('"') {
        return Some(stripped);
    }
    let inner = text.strip_prefix('\'')?;
    inner.strip_suffix('\'')
}

fn unescape_string(text: &str) -> String {
    let inner = strip_quotes(text).unwrap_or(text);
    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();

    while let Some(char) = chars.next() {
        if char == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') | None => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('0') => result.push('\0'),
                Some('u') => {
                    if chars.peek() == Some(&'{') {
                        chars.next();
                        let mut hex = String::new();
                        while let Some(&digit) = chars.peek() {
                            if digit == '}' {
                                chars.next();
                                break;
                            }
                            hex.push(digit);
                            chars.next();
                        }
                        if let Ok(code) = u32::from_str_radix(&hex, 16)
                            && let Some(unicode_char) = char::from_u32(code)
                        {
                            result.push(unicode_char);
                        }
                    } else {
                        result.push('\\');
                        result.push('u');
                    }
                }
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
            }
        } else {
            result.push(char);
        }
    }

    result
}

fn extract_declaration_names(source: &str, node: &SchemaNode<'_>) -> Vec<String> {
    node.children()
        .filter_map(|child| match child.value() {
            SchemaSyntax::Identifier => {
                let text = &source[child.span().range()];
                Some(text.to_owned())
            }
            SchemaSyntax::String => {
                let text = &source[child.span().range()];
                Some(unescape_string(text))
            }
            _ => None,
        })
        .collect()
}
