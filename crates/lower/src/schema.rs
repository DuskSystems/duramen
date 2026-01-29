use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use duramen_ast::common::{Annotation, Annotations, AnyId, EntityType, Id, Name};
use duramen_ast::schema::{
    ActionDecl, ActionRef, AppliesTo, AttributeDecl, EntityDecl, EnumType, Namespace,
    PrimitiveType, RecordType, Schema, Type, TypeDecl,
};
use duramen_cst::SchemaTree;
use duramen_cst::accessors::{CstNode as _, schema as cst};
use duramen_diagnostics::{Diagnostic, Diagnostics};

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
    pub fn lower(mut self, tree: &SchemaTree) -> Result<(Schema, Diagnostics), Diagnostics> {
        let value = self.lower_schema(tree);

        if self.diagnostics.has_error() {
            Err(self.diagnostics)
        } else {
            Ok((value, self.diagnostics))
        }
    }

    fn lower_schema(&mut self, tree: &SchemaTree) -> Schema {
        let Some(schema) = tree.children().find_map(cst::Schema::cast) else {
            return Schema::empty();
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

        Schema::new(namespaces)
    }

    fn lower_default_namespace(&mut self, schema: &cst::Schema<'_>) -> Namespace {
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

        Namespace::default_namespace(entities, actions, types)
    }

    fn lower_namespace(&mut self, ns_decl: &cst::NamespaceDecl<'_>) -> Namespace {
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

        let annotations = lower_annotations_schema(ns_decl);

        Namespace::new(name, entities, actions, types, annotations)
    }

    fn lower_annotations<'b, I>(&mut self, annotations: I) -> Annotations
    where
        I: Iterator<Item = cst::Annotation<'b>>,
    {
        let mut map: BTreeMap<AnyId, Annotation> = BTreeMap::new();
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

            let annotation_value =
                value.map_or_else(Annotation::without_value, Annotation::with_value);
            map.insert(AnyId::new(name.into()), annotation_value);
        }

        Annotations::from_map(map)
    }

    fn lower_entity(&mut self, entity_decl: &cst::EntityDecl<'_>) -> Option<EntityDecl> {
        let names: Vec<Id> = entity_decl
            .names()
            .filter_map(|name| {
                name.basename(self.source)
                    .map(|basename| Id::new(String::from(basename)))
            })
            .collect();

        if names.is_empty() {
            self.diagnostics
                .push(Diagnostic::empty_node("entity name", entity_decl.range()));
            return None;
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
            .map_or_else(RecordType::empty, |attrs| {
                self.lower_attributes(attrs.attributes())
            });

        let tags = entity_decl
            .tags()
            .and_then(|tags| tags.type_expr())
            .and_then(|type_expr| self.lower_type_expr(&type_expr));

        let annotations = self.lower_annotations(entity_decl.annotations());

        Some(EntityDecl::new(
            names,
            member_of,
            attributes,
            tags,
            annotations,
        ))
    }

    fn lower_action(&mut self, action_decl: &cst::ActionDecl<'_>) -> Option<ActionDecl> {
        let names: Vec<Id> = action_decl
            .action_names(self.source)
            .map(|name| Id::new(String::from(name)))
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
            .map_or_else(AppliesTo::empty, |clause| self.lower_applies_to(&clause));

        let annotations = self.lower_annotations(action_decl.annotations());

        Some(ActionDecl::new(names, member_of, applies_to, annotations))
    }

    fn lower_type_decl(&mut self, type_decl: &cst::TypeDecl<'_>) -> Option<TypeDecl> {
        let name = type_decl
            .name()
            .and_then(|name| name.basename(self.source))
            .map(|basename| Id::new(String::from(basename)))?;

        let type_expr = type_decl.type_expr()?;
        let type_def = self.lower_type_expr(&type_expr)?;

        let annotations = self.lower_annotations(type_decl.annotations());

        Some(TypeDecl::new(name, type_def, annotations))
    }

    fn lower_action_parents(&self, parents: &cst::ActionParents<'_>) -> Vec<ActionRef> {
        let mut result = Vec::new();

        for (name, _eid) in parents.qualified_names(self.source) {
            let Some(basename) = name.basename(self.source) else {
                continue;
            };

            let action_name = Id::new(String::from(basename));

            let groups: Vec<Id> = name
                .namespace(self.source)
                .map(|segment| Id::new(String::from(segment)))
                .collect();

            result.push(ActionRef::new(action_name, groups));
        }

        result
    }

    fn lower_applies_to(&mut self, applies_to: &cst::AppliesToClause<'_>) -> AppliesTo {
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
                Some(Type::Record(record)) => Some(record),
                Some(_) => {
                    self.diagnostics.push(
                        Diagnostic::error("context must be a record type")
                            .with_label(type_expr.range(), "expected record type"),
                    );
                    None
                }
                None => None,
            })
            .unwrap_or_else(RecordType::empty);

        AppliesTo::new(principals, resources, context)
    }

    fn lower_attributes<'b>(
        &mut self,
        attributes: impl Iterator<Item = cst::AttributeDecl<'b>>,
    ) -> RecordType {
        let attrs: Vec<AttributeDecl> = attributes
            .filter_map(|attr| self.lower_attribute(&attr))
            .collect();

        RecordType::new(attrs)
    }

    fn lower_attribute(&mut self, attr: &cst::AttributeDecl<'_>) -> Option<AttributeDecl> {
        let name_str = attr.name(self.source)?;
        let name = StringUnescaper::new(name_str)
            .unescape()
            .map(Id::new)
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

        Some(AttributeDecl::new(name, required, attr_type))
    }

    fn lower_type_expr(&mut self, type_expr: &cst::TypeExpr<'_>) -> Option<Type> {
        match type_expr {
            cst::TypeExpr::Primitive(primitive) => lower_primitive(primitive),
            cst::TypeExpr::Set(set_type) => self.lower_set_type(set_type),
            cst::TypeExpr::Record(record_type) => Some(self.lower_record_type(record_type)),
            cst::TypeExpr::Entity(entity_type) => self.lower_entity_type_expr(entity_type),
            cst::TypeExpr::Enum(enum_type) => self.lower_enum_type(enum_type),
            cst::TypeExpr::Reference(name) => lower_type_reference(name, self.source),
        }
    }

    fn lower_set_type(&mut self, set_type: &cst::SetType<'_>) -> Option<Type> {
        let element_type = set_type.element_type()?;
        let element = self.lower_type_expr(&element_type)?;
        Some(Type::set(element))
    }

    fn lower_record_type(&mut self, record_type: &cst::RecordType<'_>) -> Type {
        let attrs: Vec<AttributeDecl> = record_type
            .attributes()
            .filter_map(|attr| self.lower_attribute(&attr))
            .collect();

        Type::Record(RecordType::new(attrs))
    }

    fn lower_entity_type_expr(&self, entity_type: &cst::EntityType<'_>) -> Option<Type> {
        let name = entity_type.name()?;
        let entity_type = lower_name(&name, self.source).map(EntityType::new)?;
        Some(Type::entity(entity_type))
    }

    fn lower_enum_type(&mut self, enum_type: &cst::EnumType<'_>) -> Option<Type> {
        let span = enum_type.range();
        let variants: Vec<Id> = enum_type
            .variants(self.source)
            .filter_map(|variant| {
                StringUnescaper::new(variant)
                    .unescape()
                    .map(Id::new)
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

        Some(Type::Enum(EnumType::new(variants)))
    }
}

fn is_namespace_empty(namespace: &Namespace) -> bool {
    namespace.entities().is_empty()
        && namespace.actions().is_empty()
        && namespace.types().is_empty()
}

fn lower_name(name: &cst::Name<'_>, source: &str) -> Option<Name> {
    let segments: Vec<&str> = name.segments(source).collect();

    if segments.is_empty() {
        return None;
    }

    let path: Vec<Id> = segments
        .iter()
        .take(segments.len().saturating_sub(1))
        .map(|segment| Id::new(String::from(*segment)))
        .collect();

    let basename = Id::new(String::from(*segments.last()?));

    Some(Name::new(path, basename))
}

const fn lower_annotations_schema(_ns_decl: &cst::NamespaceDecl<'_>) -> Annotations {
    Annotations::new()
}

fn lower_entity_type(name: &cst::Name<'_>, source: &str) -> Option<EntityType> {
    lower_name(name, source).map(EntityType::new)
}

fn lower_primitive(primitive: &cst::PrimitiveType<'_>) -> Option<Type> {
    let kind = primitive.kind()?;

    let primitive_type = match kind {
        cst::PrimitiveKind::Bool => PrimitiveType::Bool,
        cst::PrimitiveKind::Long => PrimitiveType::Long,
        cst::PrimitiveKind::String => PrimitiveType::String,
    };

    Some(Type::primitive(primitive_type))
}

fn lower_type_reference(name: &cst::Name<'_>, source: &str) -> Option<Type> {
    let name = lower_name(name, source)?;
    Some(Type::named(name))
}
