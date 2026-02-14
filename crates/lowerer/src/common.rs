use alloc::borrow::Cow;
use alloc::vec::Vec;

use duramen_cst::CstNode as _;
use duramen_diagnostic::Diagnostics;
use duramen_escape::Escaper;
use duramen_syntax::{Node, Syntax};
use {duramen_ast as ast, duramen_cst as cst};

use crate::error::LowerError;

/// Shared context for lowering CST to AST.
pub struct LowerContext {
    pub diagnostics: Diagnostics,
}

impl LowerContext {
    /// Creates a new lower context from parse diagnostics.
    pub const fn new(diagnostics: Diagnostics) -> Self {
        Self { diagnostics }
    }

    /// Lowers a name.
    pub fn lower_name<'src>(&mut self, name: &cst::Name<'src>) -> Option<ast::Name<'src>> {
        let segments: Vec<_> = name.segments().collect();
        if segments.is_empty() {
            return None;
        }

        let last = segments.len() - 1;
        let basename = match ast::Identifier::new(segments[last].text()) {
            Ok(identifier) => identifier,
            Err(error) => {
                self.diagnostics.push(error);
                return None;
            }
        };

        let mut path = Vec::with_capacity(last);
        for &segment in &segments[..last] {
            match ast::Identifier::new(segment.text()) {
                Ok(identifier) => path.push(identifier),
                Err(error) => self.diagnostics.push(error),
            }
        }

        Some(ast::Name::new(path, basename))
    }

    /// Lowers a name to an identifier (unqualified only).
    pub fn lower_identifier<'src>(
        &mut self,
        name: &cst::Name<'src>,
    ) -> Option<ast::Identifier<'src>> {
        let text = name.basename()?;
        match ast::Identifier::new(text) {
            Ok(identifier) => Some(identifier),
            Err(error) => {
                self.diagnostics.push(error);
                None
            }
        }
    }

    /// Lowers annotations.
    pub fn lower_annotations<'src, I: Iterator<Item = cst::Annotation<'src>>>(
        &mut self,
        annotations: I,
    ) -> Option<ast::Annotations<'src>> {
        let mut entries = Vec::new();

        for annotation in annotations {
            let node = annotation.syntax();
            if node.child(Syntax::OpenParenthesis).is_some()
                && node.child(Syntax::CloseParenthesis).is_none()
            {
                let span = if let Some(child) = node
                    .after(Syntax::OpenParenthesis)
                    .find(|child| !child.kind().is_trivial())
                {
                    child.first().range()
                } else {
                    let end = node.range().end;
                    end..end
                };

                self.diagnostics.push(LowerError::ExpectedToken {
                    span,
                    expected: "`)`",
                });
            }

            let Some(name_node) = annotation.name() else {
                continue;
            };

            let identifier = match ast::Identifier::new(name_node.text()) {
                Ok(identifier) => identifier,
                Err(error) => {
                    self.diagnostics.push(error);
                    continue;
                }
            };

            let value = if let Some(value_node) = annotation.value() {
                let raw = value_node.text();
                let offset = value_node.range().start;

                match Escaper::new(raw).unescape_str() {
                    Ok(unescaped) => ast::AnnotationValue::String(unescaped),
                    Err(errors) => {
                        for error in errors {
                            self.diagnostics.push(error.offset(offset));
                        }
                        continue;
                    }
                }
            } else if node.child(Syntax::OpenParenthesis).is_some()
                && node.child(Syntax::CloseParenthesis).is_some()
            {
                let span = if let Some(child) = node
                    .after(Syntax::OpenParenthesis)
                    .find(|child| !child.kind().is_trivial())
                {
                    child.first().range()
                } else {
                    let end = node.range().end;
                    end..end
                };

                self.diagnostics.push(LowerError::ExpectedToken {
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
                self.diagnostics.push(error);
                None
            }
        }
    }

    /// Extracts a string literal's content (unescaped) from a CST string token.
    pub fn lower_string<'src>(&mut self, node: Node<'src>) -> Option<Cow<'src, str>> {
        let raw = node.text();
        let offset = node.range().start;

        match Escaper::new(raw).unescape_str() {
            Ok(unescaped) => Some(unescaped),
            Err(errors) => {
                for error in errors {
                    self.diagnostics.push(error.offset(offset));
                }

                None
            }
        }
    }
}
