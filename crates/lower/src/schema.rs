use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use duramen_cst::accessors::CstNode as _;
use duramen_diagnostics::{Diagnostic, Diagnostics};
use {duramen_ast as ast, duramen_cst as cst};

use crate::unescape::StringUnescaper;

pub struct SchemaLowerer<'a> {
    source: &'a str,
    diagnostics: Diagnostics,
}

impl<'a> SchemaLowerer<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            diagnostics: Diagnostics::new(),
        }
    }

    /// # Errors
    pub fn lower(
        mut self,
        tree: &cst::SchemaTree,
    ) -> Result<(ast::schema::Schema, Diagnostics), Diagnostics> {
        self.collect_parse_errors(tree);
        let value = self.lower_schema(tree);

        if self.diagnostics.has_error() {
            Err(self.diagnostics)
        } else if value.is_empty() && has_non_trivial_content(self.source) {
            self.diagnostics
                .push(Diagnostic::error("expected schema declaration"));

            Err(self.diagnostics)
        } else {
            Ok((value, self.diagnostics))
        }
    }

    fn collect_parse_errors(&mut self, tree: &cst::SchemaTree) {
        for node in tree.children() {
            self.collect_errors_in_subtree(&node);
        }
    }

    fn collect_errors_in_subtree(&mut self, node: &cst::SchemaNode<'_>) {
        if node.value() == cst::syntax::schema::SchemaSyntax::Error {
            self.diagnostics.push(
                Diagnostic::error("syntax error").with_label(node.range(), "unexpected token"),
            );
        }

        for child in node.children() {
            self.collect_errors_in_subtree(&child);
        }
    }

    fn lower_schema(&mut self, tree: &cst::SchemaTree) -> ast::schema::Schema {
        let Some(schema) = tree
            .children()
            .find_map(cst::accessors::schema::Schema::cast)
        else {
            return ast::schema::Schema::empty();
        };

        let mut namespaces = Vec::new();

        let default_ns = self.lower_default_namespace(&schema);
        if !is_namespace_empty(&default_ns) {
            namespaces.push(default_ns);
        }

        for ns_decl in schema.namespaces() {
            let namespace = self.lower_namespace(&ns_decl);
            namespaces.push(namespace);
        }

        ast::schema::Schema::new(namespaces)
    }

    fn lower_default_namespace(
        &mut self,
        schema: &cst::accessors::schema::Schema<'_>,
    ) -> ast::schema::Namespace {
        let mut entities = Vec::new();
        let mut actions = Vec::new();
        let mut types = Vec::new();

        for entity_decl in schema.entities() {
            if let Some(entity) = self.lower_entity(&entity_decl) {
                entities.push(entity);
            }
        }

        for action_decl in schema.actions() {
            if let Some(action) = self.lower_action(&action_decl) {
                actions.push(action);
            }
        }

        for type_decl in schema.types() {
            if let Some(type_def) = self.lower_type_decl(&type_decl) {
                types.push(type_def);
            }
        }

        ast::schema::Namespace::default_namespace(entities, actions, types)
    }

    fn lower_namespace(
        &mut self,
        ns_decl: &cst::accessors::schema::NamespaceDecl<'_>,
    ) -> ast::schema::Namespace {
        let name = ns_decl
            .name()
            .and_then(|name| lower_name(&name, self.source));

        let mut entities = Vec::new();
        let mut actions = Vec::new();
        let mut types = Vec::new();

        for entity_decl in ns_decl.entities() {
            if let Some(entity) = self.lower_entity(&entity_decl) {
                entities.push(entity);
            }
        }

        for action_decl in ns_decl.actions() {
            if let Some(action) = self.lower_action(&action_decl) {
                actions.push(action);
            }
        }

        for type_decl in ns_decl.types() {
            if let Some(type_def) = self.lower_type_decl(&type_decl) {
                types.push(type_def);
            }
        }

        for nested_ns in ns_decl.namespaces() {
            self.diagnostics.push(
                Diagnostic::error("nested namespaces are not supported")
                    .with_label(nested_ns.range(), "nested namespace"),
            );
        }

        let annotations = self.lower_annotations(ns_decl.annotations());

        ast::schema::Namespace::new(name, entities, actions, types, annotations)
    }

    fn lower_annotations<'b, I>(&mut self, annotations: I) -> ast::common::Annotations
    where
        I: Iterator<Item = cst::accessors::schema::Annotation<'b>>,
    {
        let mut map: BTreeMap<ast::common::AnyId, ast::common::Annotation> = BTreeMap::new();
        let mut seen: BTreeMap<String, Range<usize>> = BTreeMap::new();

        for annotation in annotations {
            let Some(name) = annotation.name(self.source) else {
                continue;
            };

            let span = annotation.range();

            if let Some(first) = seen.get(name) {
                self.diagnostics.push(Diagnostic::duplicate_annotation(
                    name,
                    span.clone(),
                    first.clone(),
                ));
                continue;
            }

            seen.insert(name.into(), span.clone());

            let value: Option<String> = annotation.value(self.source).and_then(|val| {
                StringUnescaper::new(val).unescape().or_else(|| {
                    self.diagnostics.push(Diagnostic::invalid_string_escape(
                        "invalid escape",
                        span.clone(),
                    ));
                    None
                })
            });

            let annotation_value = value.map_or_else(
                ast::common::Annotation::without_value,
                ast::common::Annotation::with_value,
            );
            map.insert(ast::common::AnyId::new(name.into()), annotation_value);
        }

        ast::common::Annotations::from_map(map)
    }

    fn lower_entity(
        &mut self,
        entity_decl: &cst::accessors::schema::EntityDecl<'_>,
    ) -> Option<ast::schema::EntityDecl> {
        let names: Vec<ast::common::Id> = entity_decl
            .names()
            .filter_map(|name| {
                name.basename(self.source)
                    .map(|basename| ast::common::Id::new(String::from(basename)))
            })
            .collect();

        if names.is_empty() {
            self.diagnostics
                .push(Diagnostic::empty_node("entity name", entity_decl.range()));
            return None;
        }

        let annotations = self.lower_annotations(entity_decl.annotations());

        if entity_decl.enum_type().is_some() {
            let choices = self.lower_enum_choices(entity_decl);
            return Some(ast::schema::EntityDecl::enum_entity(
                names,
                choices,
                annotations,
            ));
        }

        let member_of = entity_decl
            .parents()
            .map(|parents| {
                parents
                    .types()
                    .map(|type_list| {
                        type_list
                            .names()
                            .filter_map(|name| lower_entity_type(&name, self.source))
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let attributes = entity_decl
            .attributes()
            .map_or_else(ast::schema::RecordType::empty, |attrs| {
                self.lower_attributes(attrs.attributes())
            });

        let tags = entity_decl
            .tags()
            .and_then(|tags| tags.type_expr())
            .and_then(|type_expr| self.lower_type_expr(&type_expr));

        Some(ast::schema::EntityDecl::standard(
            names,
            member_of,
            attributes,
            tags,
            annotations,
        ))
    }

    fn lower_enum_choices(
        &mut self,
        entity_decl: &cst::accessors::schema::EntityDecl<'_>,
    ) -> Vec<ast::common::Id> {
        let Some(enum_type) = entity_decl.enum_type() else {
            return Vec::new();
        };

        let span = enum_type.range();
        enum_type
            .variants(self.source)
            .filter_map(|variant| {
                StringUnescaper::new(variant)
                    .unescape()
                    .map(ast::common::Id::new)
                    .or_else(|| {
                        self.diagnostics.push(Diagnostic::invalid_string_escape(
                            "invalid escape",
                            span.clone(),
                        ));
                        None
                    })
            })
            .collect()
    }

    fn lower_action(
        &mut self,
        action_decl: &cst::accessors::schema::ActionDecl<'_>,
    ) -> Option<ast::schema::ActionDecl> {
        let names: Vec<ast::common::Id> = action_decl
            .action_names(self.source)
            .filter_map(|name| {
                StringUnescaper::new(name)
                    .unescape()
                    .map(ast::common::Id::new)
            })
            .collect();

        if names.is_empty() {
            self.diagnostics
                .push(Diagnostic::empty_node("action name", action_decl.range()));
            return None;
        }

        let member_of = action_decl
            .parents()
            .map(|parents| self.lower_action_parents(&parents))
            .unwrap_or_default();

        let applies_to = action_decl
            .applies_to()
            .map_or_else(ast::schema::AppliesTo::empty, |clause| {
                self.lower_applies_to(&clause)
            });

        let annotations = self.lower_annotations(action_decl.annotations());

        Some(ast::schema::ActionDecl::new(
            names,
            member_of,
            applies_to,
            annotations,
        ))
    }

    fn lower_type_decl(
        &mut self,
        type_decl: &cst::accessors::schema::TypeDecl<'_>,
    ) -> Option<ast::schema::TypeDecl> {
        let name = type_decl
            .name()
            .and_then(|name| name.basename(self.source))
            .map(|basename| ast::common::Id::new(String::from(basename)))?;

        let type_expr = type_decl.type_expr()?;
        let type_def = self.lower_type_expr(&type_expr)?;

        let annotations = self.lower_annotations(type_decl.annotations());

        Some(ast::schema::TypeDecl::new(name, type_def, annotations))
    }

    fn lower_action_parents(
        &mut self,
        parents: &cst::accessors::schema::ActionParents<'_>,
    ) -> Vec<ast::schema::ActionRef> {
        let mut result = Vec::new();

        for (name, eid) in parents.qualified_names(self.source) {
            match (name, eid) {
                (None, Some(eid)) => {
                    let unescaped_eid = StringUnescaper::new(eid).unescape().unwrap_or_else(|| {
                        self.diagnostics.push(Diagnostic::invalid_string_escape(
                            "invalid escape sequence in action member",
                            parents.range(),
                        ));

                        String::from(eid)
                    });

                    let action_name = ast::common::Id::new(unescaped_eid);
                    result.push(ast::schema::ActionRef::simple(action_name));
                }

                (Some(name), Some(eid)) => {
                    let unescaped_eid = StringUnescaper::new(eid).unescape().unwrap_or_else(|| {
                        self.diagnostics.push(Diagnostic::invalid_string_escape(
                            "invalid escape sequence in action member",
                            name.range(),
                        ));

                        String::from(eid)
                    });
                    let action_name = ast::common::Id::new(unescaped_eid);

                    let groups: Vec<ast::common::Id> = name
                        .segments(self.source)
                        .map(|segment| ast::common::Id::new(String::from(segment)))
                        .collect();

                    result.push(ast::schema::ActionRef::new(action_name, groups));
                }

                (Some(name), None) => {
                    let Some(basename) = name.basename(self.source) else {
                        continue;
                    };

                    let action_name = ast::common::Id::new(String::from(basename));

                    let groups: Vec<ast::common::Id> = name
                        .namespace(self.source)
                        .map(|segment| ast::common::Id::new(String::from(segment)))
                        .collect();

                    result.push(ast::schema::ActionRef::new(action_name, groups));
                }

                (None, None) => {}
            }
        }

        result
    }

    fn lower_applies_to(
        &mut self,
        applies_to: &cst::accessors::schema::AppliesToClause<'_>,
    ) -> ast::schema::AppliesTo {
        let principals = applies_to
            .principal_types()
            .and_then(|principal_types| principal_types.types())
            .map(|type_list| {
                type_list
                    .names()
                    .filter_map(|name| lower_entity_type(&name, self.source))
                    .collect()
            })
            .unwrap_or_default();

        let resources = applies_to
            .resource_types()
            .and_then(|resource_types| resource_types.types())
            .map(|type_list| {
                type_list
                    .names()
                    .filter_map(|name| lower_entity_type(&name, self.source))
                    .collect()
            })
            .unwrap_or_default();

        let context = applies_to
            .context_type()
            .and_then(|context_type| context_type.type_expr())
            .and_then(|type_expr| match self.lower_type_expr(&type_expr) {
                Some(ast::schema::Type::Record(record)) => Some(record),
                Some(_) => {
                    self.diagnostics.push(
                        Diagnostic::error("context must be a record type")
                            .with_label(type_expr.range(), "expected record type"),
                    );
                    None
                }
                None => None,
            })
            .unwrap_or_else(ast::schema::RecordType::empty);

        ast::schema::AppliesTo::new(principals, resources, context)
    }

    fn lower_attributes<'b>(
        &mut self,
        attributes: impl Iterator<Item = cst::accessors::schema::AttributeDecl<'b>>,
    ) -> ast::schema::RecordType {
        let attrs: Vec<ast::schema::AttributeDecl> = attributes
            .filter_map(|attr| self.lower_attribute(&attr))
            .collect();

        ast::schema::RecordType::new(attrs)
    }

    fn lower_attribute(
        &mut self,
        attr: &cst::accessors::schema::AttributeDecl<'_>,
    ) -> Option<ast::schema::AttributeDecl> {
        let name_str = attr.name(self.source)?;
        let name = StringUnescaper::new(name_str)
            .unescape()
            .map(ast::common::Id::new)
            .or_else(|| {
                self.diagnostics.push(Diagnostic::invalid_string_escape(
                    "invalid escape",
                    attr.range(),
                ));

                None
            })?;

        let required = !attr.is_optional();

        let type_expr = attr.type_expr()?;
        let attr_type = self.lower_type_expr(&type_expr)?;

        let annotations = self.lower_annotations(attr.annotations());

        Some(ast::schema::AttributeDecl::new(
            name,
            required,
            attr_type,
            annotations,
        ))
    }

    fn lower_type_expr(
        &mut self,
        type_expr: &cst::accessors::schema::TypeExpr<'_>,
    ) -> Option<ast::schema::Type> {
        match type_expr {
            cst::accessors::schema::TypeExpr::Set(set_type) => self.lower_set_type(set_type),
            cst::accessors::schema::TypeExpr::Record(record_type) => {
                Some(self.lower_record_type(record_type))
            }
            cst::accessors::schema::TypeExpr::Entity(entity_type) => {
                self.lower_entity_type_expr(entity_type)
            }
            cst::accessors::schema::TypeExpr::Enum(enum_type) => self.lower_enum_type(enum_type),
            cst::accessors::schema::TypeExpr::Reference(name) => {
                lower_type_reference(name, self.source)
            }
        }
    }

    fn lower_set_type(
        &mut self,
        set_type: &cst::accessors::schema::SetType<'_>,
    ) -> Option<ast::schema::Type> {
        let element_type = set_type.element_type()?;
        let element = self.lower_type_expr(&element_type)?;
        Some(ast::schema::Type::set(element))
    }

    fn lower_record_type(
        &mut self,
        record_type: &cst::accessors::schema::RecordType<'_>,
    ) -> ast::schema::Type {
        let attrs: Vec<ast::schema::AttributeDecl> = record_type
            .attributes()
            .filter_map(|attr| self.lower_attribute(&attr))
            .collect();

        ast::schema::Type::Record(ast::schema::RecordType::new(attrs))
    }

    fn lower_entity_type_expr(
        &self,
        entity_type: &cst::accessors::schema::EntityType<'_>,
    ) -> Option<ast::schema::Type> {
        let name = entity_type.name()?;
        let entity_type = lower_name(&name, self.source).map(ast::common::EntityType::new)?;
        Some(ast::schema::Type::entity(entity_type))
    }

    fn lower_enum_type(
        &mut self,
        enum_type: &cst::accessors::schema::EnumType<'_>,
    ) -> Option<ast::schema::Type> {
        let span = enum_type.range();
        let variants: Vec<ast::common::Id> = enum_type
            .variants(self.source)
            .filter_map(|variant| {
                StringUnescaper::new(variant)
                    .unescape()
                    .map(ast::common::Id::new)
                    .or_else(|| {
                        self.diagnostics.push(Diagnostic::invalid_string_escape(
                            "invalid escape",
                            span.clone(),
                        ));
                        None
                    })
            })
            .collect();

        if variants.is_empty() {
            self.diagnostics
                .push(Diagnostic::empty_node("enum variant", enum_type.range()));
            return None;
        }

        Some(ast::schema::Type::Enum(ast::schema::EnumType::new(
            variants,
        )))
    }
}

fn is_namespace_empty(namespace: &ast::schema::Namespace) -> bool {
    namespace.entities().is_empty()
        && namespace.actions().is_empty()
        && namespace.types().is_empty()
}

fn lower_name(name: &cst::accessors::schema::Name<'_>, source: &str) -> Option<ast::common::Name> {
    let segments: Vec<&str> = name.segments(source).collect();

    if segments.is_empty() {
        return None;
    }

    let path: Vec<ast::common::Id> = segments
        .iter()
        .take(segments.len().saturating_sub(1))
        .map(|segment| ast::common::Id::new(String::from(*segment)))
        .collect();

    let basename = ast::common::Id::new(String::from(*segments.last()?));

    Some(ast::common::Name::new(path, basename))
}

fn lower_entity_type(
    name: &cst::accessors::schema::Name<'_>,
    source: &str,
) -> Option<ast::common::EntityType> {
    lower_name(name, source).map(ast::common::EntityType::new)
}

fn lower_type_reference(
    name: &cst::accessors::schema::Name<'_>,
    source: &str,
) -> Option<ast::schema::Type> {
    let name = lower_name(name, source)?;
    Some(ast::schema::Type::named(name))
}

fn has_non_trivial_content(source: &str) -> bool {
    use duramen_lexer::Lexer;
    Lexer::new(source).any(|token| !token.kind.is_trivial())
}
