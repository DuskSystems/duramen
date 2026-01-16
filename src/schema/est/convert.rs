use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use bumpalo::Bump;
use bumpalo::collections::Vec as BumpVec;

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
pub fn convert_schema<'a>(bump: &'a Bump, schema: &Schema<'_>) -> SchemaFragment<'a> {
    let mut result = SchemaFragment::new();
    let source = schema.source();

    if let Some(root) = schema.root() {
        let mut pending_annotations: Vec<ast::Annotation<'_>> = Vec::new();
        let mut top_level_namespace_def: Option<NamespaceDefinition<'a>> = None;

        for child in root.syntax().children() {
            match child.value() {
                SchemaSyntax::Annotation => {
                    if let Some(ann) = ast::Annotation::cast(child) {
                        pending_annotations.push(ann);
                    }
                }
                SchemaSyntax::Namespace => {
                    if let Some(namespace) = Namespace::cast(child) {
                        let name = namespace_name(bump, source, &namespace);
                        let mut namespace_def =
                            convert_node_children(bump, source, namespace.syntax(), name);

                        namespace_def.annotations = collect_annotations(
                            bump,
                            source,
                            core::mem::take(&mut pending_annotations).into_iter(),
                        );

                        result.insert(name, namespace_def);
                    }
                }
                SchemaSyntax::EntityDeclaration
                | SchemaSyntax::ActionDeclaration
                | SchemaSyntax::CommonTypeDeclaration => {
                    if top_level_namespace_def.is_none() {
                        top_level_namespace_def =
                            Some(convert_node_children(bump, source, root.syntax(), ""));
                    }

                    pending_annotations.clear();
                }
                _ => {}
            }
        }

        if let Some(namespace_def) = top_level_namespace_def {
            result.insert("", namespace_def);
        }
    }

    result
}

fn namespace_name<'a>(bump: &'a Bump, source: &str, namespace: &Namespace<'_>) -> &'a str {
    namespace
        .name()
        .map_or("", |name| name_to_str(bump, source, &name))
}

fn convert_node_children<'a>(
    bump: &'a Bump,
    source: &str,
    node: &SchemaNode<'_>,
    namespace_path: &'a str,
) -> NamespaceDefinition<'a> {
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
                        bump,
                        source,
                        core::mem::take(&mut pending_annotations).into_iter(),
                    );

                    for (name, mut entity_type) in convert_entity_declaration(bump, source, &entity)
                    {
                        for (key, value) in &annotations {
                            entity_type.annotations.insert(*key, *value);
                        }

                        def.entity_types.insert(name, entity_type);
                    }
                }
            }
            SchemaSyntax::ActionDeclaration => {
                if let Some(action) = ActionDeclaration::cast(child) {
                    let annotations = collect_annotations(
                        bump,
                        source,
                        core::mem::take(&mut pending_annotations).into_iter(),
                    );

                    for (name, mut action_type) in
                        convert_action_declaration(bump, source, &action, namespace_path)
                    {
                        for (key, value) in &annotations {
                            action_type.annotations.insert(*key, *value);
                        }

                        def.actions.insert(name, action_type);
                    }
                }
            }
            SchemaSyntax::CommonTypeDeclaration => {
                if let Some(type_decl) = TypeDeclaration::cast(child) {
                    pending_annotations.clear();
                    if let Some((name, type_def)) =
                        convert_type_declaration(bump, source, &type_decl)
                    {
                        def.common_types.insert(name, type_def);
                    }
                }
            }
            _ => {}
        }
    }

    def
}

fn convert_entity_declaration<'a>(
    bump: &'a Bump,
    source: &str,
    entity: &EntityDeclaration<'_>,
) -> Vec<(&'a str, EntityType<'a>)> {
    let names: Vec<&'a str> = extract_declaration_names(bump, source, entity.syntax());
    if names.is_empty() {
        return Vec::new();
    }

    let base_annotations = collect_annotations(bump, source, entity.annotations());

    let member_of_names: Vec<&'a str> = entity
        .parents()
        .and_then(|parents| parents.type_list())
        .map(|type_list| {
            type_list
                .types()
                .map(|name| name_to_str(bump, source, &name))
                .collect()
        })
        .unwrap_or_default();

    let shape_record = entity
        .attributes()
        .and_then(|attrs| attrs.record_type())
        .map(|record| convert_record_type(bump, source, &record));

    let tags_type = entity
        .tags()
        .and_then(|tags| tags.tag_type())
        .map(|type_expr| convert_type_expr(bump, source, &type_expr, true, BTreeMap::new()));

    let enum_variant_names: Vec<&'a str> = entity
        .enum_type()
        .map(|enum_type| {
            enum_type
                .variants()
                .map(|variant| unescape_str(bump, variant.text(source)))
                .collect()
        })
        .unwrap_or_default();

    names
        .into_iter()
        .map(|name| {
            let mut member_of_types = BumpVec::new_in(bump);
            member_of_types.extend(member_of_names.iter().copied());

            let enum_values = if enum_variant_names.is_empty() {
                None
            } else {
                let mut values = BumpVec::new_in(bump);
                values.extend(enum_variant_names.iter().copied());
                Some(values)
            };

            let entity_type = EntityType {
                annotations: base_annotations.clone(),
                member_of_types,
                shape: shape_record.clone(),
                tags: tags_type.clone(),
                enum_values,
            };

            (name, entity_type)
        })
        .collect()
}

fn convert_action_declaration<'a>(
    bump: &'a Bump,
    source: &str,
    action: &ActionDeclaration<'_>,
    namespace_path: &'a str,
) -> Vec<(&'a str, ActionType<'a>)> {
    let names: Vec<&'a str> = extract_declaration_names(bump, source, action.syntax());
    if names.is_empty() {
        return Vec::new();
    }

    let base_annotations = collect_annotations(bump, source, action.annotations());

    let member_of_refs: Vec<ActionRef<'a>> = action
        .parents()
        .map(|parents| {
            parents
                .names()
                .map(|name| parse_action_ref(bump, source, &name))
                .collect()
        })
        .unwrap_or_default();

    let principal_type_names: Vec<&'a str> = action
        .applies_to()
        .and_then(|applies| applies.principal_types())
        .and_then(|pt| pt.type_list())
        .map(|type_list| {
            type_list
                .types()
                .map(|name| qualify_entity_name(bump, source, &name, namespace_path))
                .collect()
        })
        .unwrap_or_default();

    let resource_type_names: Vec<&'a str> = action
        .applies_to()
        .and_then(|applies| applies.resource_types())
        .and_then(|rt| rt.type_list())
        .map(|type_list| {
            type_list
                .types()
                .map(|name| qualify_entity_name(bump, source, &name, namespace_path))
                .collect()
        })
        .unwrap_or_default();

    let context_type = action
        .applies_to()
        .and_then(|applies| applies.context_type())
        .and_then(|ctx| ctx.type_expr())
        .map(|type_expr| convert_type_expr(bump, source, &type_expr, true, BTreeMap::new()));

    names
        .into_iter()
        .map(|name| {
            let mut member_of = BumpVec::new_in(bump);
            member_of.extend(member_of_refs.iter().cloned());

            let mut principal_types = BumpVec::new_in(bump);
            principal_types.extend(principal_type_names.iter().copied());

            let mut resource_types = BumpVec::new_in(bump);
            resource_types.extend(resource_type_names.iter().copied());

            let applies_to = Some(AppliesTo {
                principal_types,
                resource_types,
                context: context_type.clone(),
            });

            let action_type = ActionType {
                annotations: base_annotations.clone(),
                member_of,
                applies_to,
            };

            (name, action_type)
        })
        .collect()
}

fn convert_type_declaration<'a>(
    bump: &'a Bump,
    source: &str,
    type_decl: &TypeDeclaration<'_>,
) -> Option<(&'a str, TypeDef<'a>)> {
    let name = bump.alloc_str(type_decl.name()?.text(source));
    let definition = type_decl.definition()?;
    let type_def = convert_type_expr(bump, source, &definition, true, BTreeMap::new());
    Some((name, type_def))
}

fn convert_type_expr<'a>(
    bump: &'a Bump,
    source: &str,
    type_expr: &TypeExpr<'_>,
    required: bool,
    annotations: BTreeMap<&'a str, &'a str>,
) -> TypeDef<'a> {
    match type_expr {
        TypeExpr::Name(name) => TypeDef::EntityOrCommon {
            name: name_to_str(bump, source, name),
            required,
            annotations,
        },
        TypeExpr::Set(set_type) => {
            let element = set_type.element().map_or_else(
                || TypeDef::EntityOrCommon {
                    name: "Unknown",
                    required: true,
                    annotations: BTreeMap::new(),
                },
                |elem| convert_type_expr(bump, source, &elem, true, BTreeMap::new()),
            );
            TypeDef::Set {
                element: bump.alloc(element),
                required,
                annotations,
            }
        }
        TypeExpr::Record(record) => TypeDef::Record {
            record: convert_record_type(bump, source, record),
            required,
            annotations,
        },
        TypeExpr::Entity(entity_ref) => TypeDef::EntityOrCommon {
            name: entity_ref
                .name()
                .map_or("", |name| name_to_str(bump, source, &name)),
            required,
            annotations,
        },
    }
}

fn convert_record_type<'a>(
    bump: &'a Bump,
    source: &str,
    record: &ast::RecordType<'_>,
) -> RecordType<'a> {
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
                        bump,
                        source,
                        core::mem::take(&mut pending_annotations).into_iter(),
                    );

                    if let Some(key) = attr.name() {
                        let key_str = match key {
                            ast::AttrKey::Identifier(ident) => bump.alloc_str(ident.text(source)),
                            ast::AttrKey::String(string) => unescape_str(bump, string.text(source)),
                            ast::AttrKey::Keyword(node) => {
                                bump.alloc_str(&source[node.span().range()])
                            }
                        };

                        let required = !attr.is_optional();
                        if let Some(type_expr) = attr.attribute_type() {
                            let type_def =
                                convert_type_expr(bump, source, &type_expr, required, annotations);
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

fn collect_annotations<'a, 'b>(
    bump: &'a Bump,
    source: &str,
    annotations: impl Iterator<Item = ast::Annotation<'b>>,
) -> BTreeMap<&'a str, &'a str> {
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
            let key: &'a str = bump.alloc_str(key);
            let value: &'a str = annotation
                .value()
                .map_or("", |value| unescape_str(bump, value.text(source)));

            result.insert(key, value);
        }
    }

    result
}

fn name_to_str<'a>(bump: &'a Bump, source: &str, name: &ast::Name<'_>) -> &'a str {
    let node = name.syntax();
    let mut parts: Vec<&str> = Vec::new();

    for child in node.children() {
        let kind = child.value();
        match kind {
            SchemaSyntax::Identifier
            | SchemaSyntax::BoolKeyword
            | SchemaSyntax::LongKeyword
            | SchemaSyntax::StringKeyword => {
                parts.push(&source[child.span().range()]);
            }
            _ => {}
        }
    }

    if let [single] = parts.as_slice() {
        bump.alloc_str(single)
    } else {
        let joined: String = parts.join("::");
        bump.alloc_str(&joined)
    }
}

fn qualify_entity_name<'a>(
    bump: &'a Bump,
    source: &str,
    name: &ast::Name<'_>,
    _namespace_path: &str,
) -> &'a str {
    name_to_str(bump, source, name)
}

fn parse_action_ref<'a>(bump: &'a Bump, source: &str, name: &ast::Name<'_>) -> ActionRef<'a> {
    let node = name.syntax();

    let mut identifiers: Vec<&str> = Vec::new();
    let mut action_id: Option<&'a str> = None;

    for child in node.children() {
        match child.value() {
            SchemaSyntax::Identifier => {
                identifiers.push(&source[child.span().range()]);
            }
            SchemaSyntax::String => {
                action_id = Some(unescape_str(bump, &source[child.span().range()]));
            }
            _ => {}
        }
    }

    match (action_id, identifiers.as_slice()) {
        (Some(id), []) => ActionRef { id, ty: None },
        (Some(id), parts) => ActionRef {
            id,
            ty: Some(bump.alloc_str(&parts.join("::"))),
        },
        (None, []) => ActionRef { id: "", ty: None },
        (None, [single]) => ActionRef {
            id: bump.alloc_str(single),
            ty: None,
        },
        (None, [init @ .., last]) => ActionRef {
            id: bump.alloc_str(last),
            ty: Some(bump.alloc_str(&init.join("::"))),
        },
    }
}

fn strip_quotes(text: &str) -> Option<&str> {
    if let Some(inner) = text.strip_prefix('"') {
        return inner.strip_suffix('"');
    }

    if let Some(inner) = text.strip_prefix('\'') {
        return inner.strip_suffix('\'');
    }

    None
}

fn unescape_str<'a>(bump: &'a Bump, text: &str) -> &'a str {
    let inner = strip_quotes(text).unwrap_or(text);
    if !inner.contains('\\') {
        let result: &'a str = bump.alloc_str(inner);
        return result;
    }

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

    let str_ref: &'a str = bump.alloc_str(&result);
    str_ref
}

fn extract_declaration_names<'a>(
    bump: &'a Bump,
    source: &str,
    node: &SchemaNode<'_>,
) -> Vec<&'a str> {
    node.children()
        .filter_map(|child| match child.value() {
            SchemaSyntax::Identifier => {
                let str_ref: &'a str = bump.alloc_str(&source[child.span().range()]);
                Some(str_ref)
            }
            SchemaSyntax::String => Some(unescape_str(bump, &source[child.span().range()])),
            _ => None,
        })
        .collect()
}
