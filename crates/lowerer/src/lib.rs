#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use duramen_cst::CstNode as _;
use duramen_diagnostic::{Diagnostic, Diagnostics};
use duramen_escape::Escaper;
use duramen_syntax::{Node, Syntax};
use {duramen_ast as ast, duramen_cst as cst};

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

    /// Emits a context-aware diagnostic for the first error child of `node`.
    fn emit_error(&mut self, node: Node<'_>, context: &str) {
        let Some(error_node) = node.children().find(|child| child.kind().is_error()) else {
            return;
        };

        if let Some(diagnostic) = Self::recognize_error(error_node) {
            self.diagnostic(diagnostic);
            return;
        }

        let span = error_node
            .children()
            .filter(|child| !child.kind().is_trivial())
            .last()
            .map_or_else(
                || error_node.range(),
                |last| error_node.range().start..last.range().end,
            );

        let text = &self.source[span.clone()];
        self.diagnostic(
            Diagnostic::error(format!("unexpected token `{text}`"))
                .with_label(span, format!("not valid in {context}")),
        );
    }

    /// Recognizes well-known error tokens and returns a tailored diagnostic.
    fn recognize_error(node: Node<'_>) -> Option<Diagnostic> {
        let child = if node.kind().is_token() {
            node
        } else {
            node.children().find(|child| !child.kind().is_trivial())?
        };
        let kind = child.kind();
        let range = child.range();
        let outer_range = node.range();

        let diagnostic = match kind {
            Syntax::StringSingleQuoted => {
                Diagnostic::error("strings must use double quotes, not single quotes")
                    .with_label(range, "single-quoted string")
                    .with_help("replace single quotes `'` with double quotes `\"`")
            }

            Syntax::CommentBlock => {
                let text = child.text();
                let mut diagnostic = Diagnostic::error("block comments are not supported")
                    .with_label(range.start..range.start + 2, "comment opened here")
                    .with_help("use `//` for line comments");

                if text.ends_with("*/") {
                    diagnostic =
                        diagnostic.with_context(range.end - 2..range.end, "comment closed here");
                }

                diagnostic
            }

            Syntax::StringUnterminated => Diagnostic::error("unterminated string literal")
                .with_label(range.start..range.start + 1, "string is never closed")
                .with_help("add a closing `\"` to terminate the string"),

            Syntax::Ampersand => Diagnostic::error("unexpected `&`")
                .with_label(range, "not a valid operator")
                .with_help("use `&&` for logical and"),

            Syntax::Pipe => Diagnostic::error("unexpected `|`")
                .with_label(range, "not a valid operator")
                .with_help("use `||` for logical or"),

            Syntax::ContextKeyword => Diagnostic::error("`context` is not a scope variable")
                .with_label(range, "not valid in scope")
                .with_note("`context` can only be used in conditions, not in the scope"),

            Syntax::At => Diagnostic::error("annotations are not valid in the scope")
                .with_label(outer_range, "not valid in scope")
                .with_help("move annotations before the effect"),

            _ => return None,
        };

        Some(diagnostic)
    }

    /// Lowers annotations.
    fn lower_annotations<'cst>(
        &mut self,
        annotations: impl Iterator<Item = cst::Annotation<'cst>>,
    ) -> Option<ast::Annotations<'src>> {
        let mut entries = Vec::new();

        for annotation in annotations {
            let node = annotation.syntax();
            if node.child(Syntax::OpenParenthesis).is_some()
                && node.child(Syntax::CloseParenthesis).is_none()
            {
                if let Some(diagnostic) = node
                    .descendants()
                    .filter(|child| child.kind().is_error())
                    .find_map(Self::recognize_error)
                {
                    self.diagnostic(diagnostic);
                    return None;
                }

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

                self.diagnostic(Diagnostic::error("expected `)`").with_label(span, "expected `)`"));

                return None;
            }

            if let Some(error_node) = node
                .after(Syntax::OpenParenthesis)
                .take_while(|child| child.kind() != Syntax::CloseParenthesis)
                .find(|child| child.kind().is_error())
            {
                if let Some(diagnostic) = Self::recognize_error(error_node) {
                    self.diagnostic(diagnostic);
                } else if let Some(value_node) = annotation.value() {
                    let span = value_node.range().start..error_node.range().end;
                    self.diagnostic(
                        Diagnostic::error("annotations accept only a single string value")
                            .with_label(span, "unexpected extra content")
                            .with_note("use `@name(\"value\")` with a single string"),
                    );
                } else {
                    self.diagnostic(
                        Diagnostic::error("expected a string literal")
                            .with_label(error_node.range(), "not a valid annotation value")
                            .with_note("use `@name(\"value\")` with a double-quoted string"),
                    );
                }

                return None;
            }

            // Check for error nodes in the annotation name (e.g. @bad-annotation
            // where `-` splits the identifier into multiple tokens).
            if let Some(error_node) = node
                .children()
                .take_while(|child| child.kind() != Syntax::OpenParenthesis)
                .find(|child| child.kind().is_error())
            {
                let name_start = annotation
                    .name()
                    .map_or_else(|| error_node.range().start, |name| name.range().start);

                let name_end = error_node
                    .children()
                    .filter(|child| !child.kind().is_trivial())
                    .last()
                    .map_or_else(|| error_node.range().end, |last| last.range().end);

                let span = name_start..name_end;
                let text = &self.source[span.clone()];

                self.diagnostic(
                    Diagnostic::error(format!("invalid annotation name `{text}`"))
                        .with_label(span, "must be a valid identifier")
                        .with_help("annotation names can only contain letters, digits, and `_`"),
                );

                return None;
            }

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
            } else if let Some(open) = node.child(Syntax::OpenParenthesis)
                && let Some(close) = node.child(Syntax::CloseParenthesis)
            {
                self.diagnostic(
                    Diagnostic::error("empty annotation value")
                        .with_label(
                            open.range().start..close.range().end,
                            "value cannot be empty",
                        )
                        .with_help("provide a string value: `@name(\"value\")`"),
                );

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
