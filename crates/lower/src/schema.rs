use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use duramen_ast as ast;
use duramen_cst::{self as cst, CstNode as _};
use duramen_diagnostic::Diagnostics;
use duramen_escape::Escaper;
use duramen_syntax::Syntax;

use crate::LowerContext;
use crate::error::LowerError;

/// Schema lowerer for CST-to-AST transformation.
pub struct SchemaLowerer<'a, 'src> {
    ctx: LowerContext<'a, 'src>,
}

impl<'a, 'src> SchemaLowerer<'a, 'src> {
    /// Creates a new schema lowerer.
    #[must_use]
    pub const fn new(source: &'src str, diagnostics: &'a mut Diagnostics) -> Self {
        Self {
            ctx: LowerContext::new(source, diagnostics),
        }
    }

    /// Lowers a CST schema node to an AST.
    #[must_use]
    pub fn lower(mut self, schema: cst::Schema<'src>) -> ast::Schema<'src> {
        let mut namespaces = Vec::new();
        let mut top_declarations = Vec::new();

        for child in schema.syntax().children() {
            match child.kind() {
                Syntax::EntityDeclaration => {
                    if let Some(entity) = cst::EntityDeclaration::cast(child)
                        && let Some(declaration) = self.lower_entity_declaration(&entity)
                    {
                        top_declarations.push(ast::Declaration::Entity(declaration));
                    }
                }
                Syntax::ActionDeclaration => {
                    if let Some(action) = cst::ActionDeclaration::cast(child)
                        && let Some(declaration) = self.lower_action_declaration(&action)
                    {
                        top_declarations.push(ast::Declaration::Action(declaration));
                    }
                }
                Syntax::TypeDeclaration => {
                    if let Some(type_declaration) = cst::TypeDeclaration::cast(child)
                        && let Some(declaration) = self.lower_type_declaration(&type_declaration)
                    {
                        top_declarations.push(ast::Declaration::Type(declaration));
                    }
                }
                _ => {}
            }
        }

        if !top_declarations.is_empty()
            && let Some(annotations) = self.ctx.lower_annotations(schema.annotations())
        {
            match ast::Namespace::new(annotations, None, top_declarations) {
                Ok(namespace) => namespaces.push(namespace),
                Err(error) => self.ctx.diagnostic(error),
            }
        }

        for namespace in schema.namespaces() {
            if let Some(ns) = self.lower_namespace(&namespace) {
                namespaces.push(ns);
            }
        }

        ast::Schema::new(namespaces)
    }

    /// Lowers a namespace declaration.
    fn lower_namespace(
        &mut self,
        namespace: &cst::Namespace<'src>,
    ) -> Option<ast::Namespace<'src>> {
        let mut declarations = Vec::new();

        for child in namespace.syntax().children() {
            match child.kind() {
                Syntax::EntityDeclaration => {
                    if let Some(entity) = cst::EntityDeclaration::cast(child)
                        && let Some(declaration) = self.lower_entity_declaration(&entity)
                    {
                        declarations.push(ast::Declaration::Entity(declaration));
                    }
                }
                Syntax::ActionDeclaration => {
                    if let Some(action) = cst::ActionDeclaration::cast(child)
                        && let Some(declaration) = self.lower_action_declaration(&action)
                    {
                        declarations.push(ast::Declaration::Action(declaration));
                    }
                }
                Syntax::TypeDeclaration => {
                    if let Some(type_declaration) = cst::TypeDeclaration::cast(child)
                        && let Some(declaration) = self.lower_type_declaration(&type_declaration)
                    {
                        declarations.push(ast::Declaration::Type(declaration));
                    }
                }
                Syntax::NamespaceDeclaration => {
                    if let Some(nested) = cst::Namespace::cast(child) {
                        self.ctx.diagnostic(LowerError::NestedNamespace {
                            span: nested.range(),
                        });
                    }
                }
                _ => {}
            }
        }

        let annotations = self.ctx.lower_annotations(namespace.annotations())?;
        let name = namespace.name().and_then(|n| self.ctx.lower_name(&n));

        match ast::Namespace::new(annotations, name, declarations) {
            Ok(namespace) => Some(namespace),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers an entity declaration.
    fn lower_entity_declaration(
        &mut self,
        entity: &cst::EntityDeclaration<'src>,
    ) -> Option<ast::EntityDeclaration<'src>> {
        let annotations = self.ctx.lower_annotations(entity.annotations())?;

        let mut names: Vec<ast::Identifier<'src>> = Vec::new();
        for name in entity.names() {
            if name.is_qualified() {
                self.ctx
                    .diagnostic(LowerError::QualifiedEntityName { span: name.range() });

                continue;
            }

            if let Some(identifier) = self.ctx.lower_identifier(&name) {
                names.push(identifier);
            }
        }

        if names.is_empty() {
            return None;
        }

        if let Some(enumeration) = entity.enumeration() {
            let enum_type = self.lower_enum_type(&enumeration)?;
            let kind = ast::EntityKind::Enum(enum_type);

            return match ast::EntityDeclaration::new(annotations, names, kind) {
                Ok(declaration) => Some(declaration),
                Err(error) => {
                    self.ctx.diagnostic(error);
                    None
                }
            };
        }

        let parents = entity.parents().map_or_else(Vec::new, |parents| {
            self.lower_name_list(parents.types(), parents.name())
        });

        let attributes = entity.attributes().map_or_else(Vec::new, |attributes| {
            self.lower_schema_attributes(attributes.attributes())
        });

        let tags = entity.tags().and_then(|tags| {
            let definition = tags.definition()?;
            self.lower_type_expression(&definition)
        });

        let standard = match ast::StandardEntity::new(parents, attributes, tags) {
            Ok(standard) => standard,
            Err(error) => {
                self.ctx.diagnostic(error);
                return None;
            }
        };

        let kind = ast::EntityKind::Standard(standard);

        match ast::EntityDeclaration::new(annotations, names, kind) {
            Ok(declaration) => Some(declaration),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers an action declaration.
    fn lower_action_declaration(
        &mut self,
        action: &cst::ActionDeclaration<'src>,
    ) -> Option<ast::ActionDeclaration<'src>> {
        let annotations = self.ctx.lower_annotations(action.annotations())?;

        let mut names: Vec<Cow<'src, str>> = Vec::new();
        for name in action.names() {
            let text = name.basename(self.ctx.source)?;
            names.push(Cow::Borrowed(text));
        }

        if names.is_empty() {
            return None;
        }

        let parents = action
            .parents()
            .map_or_else(Vec::new, |parents| self.lower_action_parents(&parents));

        let applies_to = action
            .applies_to()
            .and_then(|applies_to| self.lower_applies_to(&applies_to));

        let attributes = action.attributes().map_or_else(Vec::new, |attributes| {
            self.lower_schema_attributes(attributes.attributes())
        });

        match ast::ActionDeclaration::new(annotations, names, parents, applies_to, attributes) {
            Ok(declaration) => Some(declaration),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers action parents.
    fn lower_action_parents(
        &mut self,
        parents: &cst::ActionParents<'src>,
    ) -> Vec<ast::ActionReference<'src>> {
        let mut result = Vec::new();

        for entity_reference in parents.entity_references() {
            let kind = entity_reference
                .kind()
                .and_then(|n| self.ctx.lower_name(&n));
            if let Some(id_node) = entity_reference.id()
                && let Some(id) = self.ctx.lower_string(id_node)
            {
                result.push(ast::ActionReference::new(kind, id));
            }
        }

        // Action parents can also appear as bare names without entity reference syntax
        for name in parents.names() {
            let Some(text) = name.basename(self.ctx.source) else {
                continue;
            };

            let kind_name = self.ctx.lower_name(&name);
            result.push(ast::ActionReference::new(kind_name, Cow::Borrowed(text)));
        }

        result
    }

    /// Lowers an applies-to clause.
    fn lower_applies_to(
        &mut self,
        applies_to: &cst::AppliesTo<'_>,
    ) -> Option<ast::AppliesTo<'src>> {
        let principals = applies_to.principals().map_or_else(Vec::new, |principals| {
            self.lower_name_list(principals.types(), principals.name())
        });

        let resources = applies_to.resources().map_or_else(Vec::new, |resources| {
            self.lower_name_list(resources.types(), resources.name())
        });

        let context = applies_to
            .context()
            .and_then(|context| self.lower_context_type(&context));

        match ast::AppliesTo::new(principals, resources, context) {
            Ok(applies_to) => Some(applies_to),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers a context type.
    fn lower_context_type(
        &mut self,
        context: &cst::ContextType<'_>,
    ) -> Option<ast::ContextType<'src>> {
        let definition = context.definition()?;

        match &definition {
            cst::TypeExpression::Record(record) => {
                let record_type = self.lower_record_type(record)?;
                Some(ast::ContextType::Record(record_type))
            }
            cst::TypeExpression::Reference(name) => {
                let ast_name = self.ctx.lower_name(name)?;
                Some(ast::ContextType::Reference(ast_name))
            }
            cst::TypeExpression::Set(_)
            | cst::TypeExpression::Entity(_)
            | cst::TypeExpression::Enum(_) => {
                self.ctx.diagnostic(LowerError::InvalidContextType {
                    span: definition.range(),
                });

                None
            }
        }
    }

    /// Lowers a type declaration.
    fn lower_type_declaration(
        &mut self,
        type_declaration: &cst::TypeDeclaration<'src>,
    ) -> Option<ast::TypeDeclaration<'src>> {
        let annotations = self.ctx.lower_annotations(type_declaration.annotations())?;

        let cst_name = type_declaration.name()?;
        if cst_name.is_qualified() {
            self.ctx.diagnostic(LowerError::QualifiedTypeName {
                span: cst_name.range(),
            });
            return None;
        }

        let identifier = self.ctx.lower_identifier(&cst_name)?;

        let definition = type_declaration.definition()?;
        let definition = self.lower_type_expression(&definition)?;

        match ast::TypeDeclaration::new(annotations, identifier, definition) {
            Ok(declaration) => Some(declaration),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers a type expression.
    fn lower_type_expression(
        &mut self,
        type_expr: &cst::TypeExpression<'_>,
    ) -> Option<ast::TypeExpression<'src>> {
        match type_expr {
            cst::TypeExpression::Set(set_type) => {
                let element = set_type.element()?;
                let element = self.lower_type_expression(&element)?;
                Some(ast::TypeExpression::Set(Box::new(element)))
            }
            cst::TypeExpression::Record(record) => {
                let record_type = self.lower_record_type(record)?;
                Some(ast::TypeExpression::Record(record_type))
            }
            cst::TypeExpression::Entity(_entity) => {
                self.ctx.diagnostic(LowerError::MissingTypeExpression {
                    span: type_expr.range(),
                });

                None
            }
            cst::TypeExpression::Enum(enum_type) => {
                let ast_enum = self.lower_enum_type(enum_type)?;
                Some(ast::TypeExpression::Enum(ast_enum))
            }
            cst::TypeExpression::Reference(name) => {
                let ast_name = self.ctx.lower_name(name)?;
                Some(ast::TypeExpression::Reference(ast_name))
            }
        }
    }

    /// Lowers a record type from attribute declarations.
    fn lower_record_type(&mut self, record: &cst::RecordType<'_>) -> Option<ast::RecordType<'src>> {
        let attributes = self.lower_schema_attributes(record.attributes());

        match ast::RecordType::new(attributes) {
            Ok(record_type) => Some(record_type),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Collects names from a type list or single name.
    fn lower_name_list(
        &mut self,
        types: Option<cst::Types<'_>>,
        single: Option<cst::Name<'_>>,
    ) -> Vec<ast::Name<'src>> {
        let mut result = Vec::new();

        if let Some(types) = types {
            for name in types.names() {
                if let Some(ast_name) = self.ctx.lower_name(&name) {
                    result.push(ast_name);
                }
            }
        } else if let Some(name) = single
            && let Some(ast_name) = self.ctx.lower_name(&name)
        {
            result.push(ast_name);
        }

        result
    }

    /// Lowers an enum type.
    fn lower_enum_type(&mut self, enum_type: &cst::EnumType<'_>) -> Option<ast::EnumType<'src>> {
        let mut variants = Vec::new();

        for variant_node in enum_type.variants() {
            let raw = self.ctx.text(variant_node);
            let start = variant_node.range().start;

            match Escaper::new(raw, start).unescape_str() {
                Ok(unescaped) => variants.push(unescaped),
                Err(errors) => {
                    for error in errors {
                        self.ctx.diagnostic(error);
                    }
                }
            }
        }

        match ast::EnumType::new(variants) {
            Ok(enum_type) => Some(enum_type),
            Err(error) => {
                self.ctx.diagnostic(error);
                None
            }
        }
    }

    /// Lowers schema attribute declarations.
    fn lower_schema_attributes<'cst>(
        &mut self,
        attributes: impl Iterator<Item = cst::AttributeDeclaration<'cst>>,
    ) -> Vec<(Cow<'src, str>, ast::AttributeDeclaration<'src>)> {
        let mut result = Vec::new();

        for attribute in attributes {
            let Some(name_node) = attribute.name() else {
                continue;
            };

            let name_text = self.ctx.text(name_node);

            let key = if name_node.kind() == Syntax::String {
                match self.ctx.lower_string(name_node) {
                    Some(unescaped) => unescaped,
                    None => continue,
                }
            } else {
                Cow::Borrowed(name_text)
            };

            let Some(annotations) = self.ctx.lower_annotations(attribute.annotations()) else {
                continue;
            };

            let optionality = if attribute.is_optional() {
                ast::Optionality::Optional
            } else {
                ast::Optionality::Required
            };

            let Some(definition) = attribute.definition() else {
                continue;
            };

            let Some(definition) = self.lower_type_expression(&definition) else {
                continue;
            };

            let declaration = ast::AttributeDeclaration::new(annotations, optionality, definition);
            result.push((key, declaration));
        }

        result
    }
}
