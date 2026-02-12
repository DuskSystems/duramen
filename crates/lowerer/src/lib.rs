#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::borrow::Cow;
use alloc::vec::Vec;

use duramen_cst::CstNode as _;
use duramen_diagnostic::{Diagnostic, Diagnostics};
use duramen_escape::Escaper;
use duramen_syntax::{Node, Syntax};
use {duramen_ast as ast, duramen_cst as cst};

use crate::error::LowerError;

mod error;

mod policy;
pub use policy::PolicyLowerer;

mod schema;
pub use schema::SchemaLowerer;

/// Shared context for lowering CST to AST.
struct LowerContext<'a> {
    diagnostics: &'a mut Diagnostics,
}

impl<'a> LowerContext<'a> {
    /// Creates a new lower context.
    const fn new(diagnostics: &'a mut Diagnostics) -> Self {
        Self { diagnostics }
    }

    /// Emits a diagnostic error.
    fn diagnostic(&mut self, diagnostic: impl Into<Diagnostic>) {
        self.diagnostics.push(diagnostic);
    }

    /// Lowers a name.
    fn lower_name<'src>(&mut self, name: &cst::Name<'src>) -> Option<ast::Name<'src>> {
        let segments: Vec<_> = name.segments().collect();
        if segments.is_empty() {
            return None;
        }

        let last = segments.len() - 1;
        let basename = self.make_identifier(segments[last].text())?;

        let mut path = Vec::with_capacity(last);
        for &segment in &segments[..last] {
            if let Some(identifier) = self.make_identifier(segment.text()) {
                path.push(identifier);
            }
        }

        Some(ast::Name::new(path, basename))
    }

    /// Lowers a name to an identifier (unqualified only).
    fn lower_identifier<'src>(&mut self, name: &cst::Name<'src>) -> Option<ast::Identifier<'src>> {
        let basename_text = name.basename()?;
        self.make_identifier(basename_text)
    }

    /// Creates an AST identifier, emitting a diagnostic on failure.
    fn make_identifier<'src>(&mut self, text: &'src str) -> Option<ast::Identifier<'src>> {
        match ast::Identifier::new(text) {
            Ok(identifier) => Some(identifier),
            Err(error) => {
                self.diagnostic(error);
                None
            }
        }
    }

    /// Lowers annotations.
    fn lower_annotations<'src>(
        &mut self,
        annotations: impl Iterator<Item = cst::Annotation<'src>>,
    ) -> Option<ast::Annotations<'src>> {
        let mut entries = Vec::new();

        for annotation in annotations {
            let node = annotation.syntax();
            if node.child(Syntax::OpenParenthesis).is_some()
                && node.child(Syntax::CloseParenthesis).is_none()
            {
                let span = node
                    .after(Syntax::OpenParenthesis)
                    .find(|child| !child.kind().is_trivial())
                    .map_or_else(
                        || {
                            let end = node.range().end;
                            end..end
                        },
                        |child| child.first().range(),
                    );

                self.diagnostic(LowerError::ExpectedToken {
                    span,
                    expected: "`)`",
                });
            }

            let Some(name_node) = annotation.name() else {
                continue;
            };

            let Some(identifier) = self.make_identifier(name_node.text()) else {
                continue;
            };

            let value = if let Some(value_node) = annotation.value() {
                let raw = value_node.text();
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
            } else if node.child(Syntax::OpenParenthesis).is_some()
                && node.child(Syntax::CloseParenthesis).is_some()
            {
                let span = node
                    .after(Syntax::OpenParenthesis)
                    .find(|child| !child.kind().is_trivial())
                    .map_or_else(
                        || {
                            let end = node.range().end;
                            end..end
                        },
                        |child| child.first().range(),
                    );

                self.diagnostic(LowerError::ExpectedToken {
                    span,
                    expected: "a string literal",
                });

                continue;
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
    fn lower_string<'src>(&mut self, node: Node<'src>) -> Option<Cow<'src, str>> {
        let raw = node.text();
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
