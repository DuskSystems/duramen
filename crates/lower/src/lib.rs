#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::borrow::Cow;
use alloc::vec::Vec;

use duramen_diagnostic::{Diagnostic, Diagnostics};
use duramen_escape::Escaper;
use duramen_syntax::Node;
use {duramen_ast as ast, duramen_cst as cst};

mod error;

mod policy;
pub use policy::PolicyLowerer;

mod schema;
pub use schema::SchemaLowerer;

/// Shared context for lowering CST to AST.
struct LowerContext<'a, 'src> {
    source: &'src str,
    diagnostics: &'a mut Diagnostics,
}

impl<'a, 'src> LowerContext<'a, 'src> {
    /// Creates a new lower context.
    const fn new(source: &'src str, diagnostics: &'a mut Diagnostics) -> Self {
        Self {
            source,
            diagnostics,
        }
    }

    /// Emits a diagnostic error.
    fn diagnostic(&mut self, diagnostic: impl Into<Diagnostic>) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns the source text for a node.
    fn text(&self, node: Node<'_>) -> &'src str {
        let range = node.range();
        &self.source[range]
    }

    /// Lowers a name.
    fn lower_name(&mut self, name: &cst::Name<'_>) -> Option<ast::Name<'src>> {
        let segments: Vec<_> = name.segments().collect();
        if segments.is_empty() {
            return None;
        }

        let last = segments.len() - 1;
        let basename_text = self.text(segments[last]);
        let basename = self.make_identifier(basename_text)?;

        let mut path = Vec::with_capacity(last);
        for &segment in &segments[..last] {
            let text = self.text(segment);
            if let Some(identifier) = self.make_identifier(text) {
                path.push(identifier);
            }
        }

        Some(ast::Name::new(path, basename))
    }

    /// Lowers a name to an identifier (unqualified only).
    fn lower_identifier(&mut self, name: &cst::Name<'src>) -> Option<ast::Identifier<'src>> {
        let basename_text = name.basename(self.source)?;
        self.make_identifier(basename_text)
    }

    /// Creates an AST identifier, emitting a diagnostic on failure.
    fn make_identifier(&mut self, text: &'src str) -> Option<ast::Identifier<'src>> {
        match ast::Identifier::new(text) {
            Ok(identifier) => Some(identifier),
            Err(error) => {
                self.diagnostic(error);
                None
            }
        }
    }

    /// Lowers annotations.
    fn lower_annotations<'cst>(
        &mut self,
        annotations: impl Iterator<Item = cst::Annotation<'cst>>,
    ) -> Option<ast::Annotations<'src>> {
        let mut entries = Vec::new();

        for annotation in annotations {
            let Some(name_node) = annotation.name() else {
                continue;
            };

            let name_text = self.text(name_node);

            let Some(identifier) = self.make_identifier(name_text) else {
                continue;
            };

            let value = if let Some(value_node) = annotation.value() {
                let raw = self.text(value_node);
                let offset = value_node.range().start;

                match Escaper::new(raw).unescape_str() {
                    Ok(unescaped) => ast::AnnotationValue::String(unescaped),
                    Err(errors) => {
                        for error in errors {
                            self.diagnostic(error.offset(offset));
                        }
                        continue;
                    }
                }
            } else {
                ast::AnnotationValue::Empty
            };

            entries.push((identifier, value));
        }

        match ast::Annotations::new(entries) {
            Ok(annotations) => Some(annotations),
            Err(error) => {
                self.diagnostic(error);
                None
            }
        }
    }

    /// Extracts a string literal's content (unescaped) from a CST string token.
    fn lower_string(&mut self, node: Node<'_>) -> Option<Cow<'src, str>> {
        let raw = self.text(node);
        let offset = node.range().start;

        match Escaper::new(raw).unescape_str() {
            Ok(unescaped) => Some(unescaped),
            Err(errors) => {
                for error in errors {
                    self.diagnostic(error.offset(offset));
                }
                None
            }
        }
    }
}

/// Known extension function names.
const EXTENSION_FUNCTIONS: &[&str] = &[
    "ip",
    "decimal",
    "datetime",
    "duration",
    "date",
    "time",
    "offset",
    "toDate",
    "toTime",
    "toDuration",
    "toMilliseconds",
    "toSeconds",
    "toMinutes",
    "toHours",
    "toDays",
    "isIpv4",
    "isIpv6",
    "isLoopback",
    "isMulticast",
    "isInRange",
    "lessThan",
    "lessThanOrEqual",
    "greaterThan",
    "greaterThanOrEqual",
];
