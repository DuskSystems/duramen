//! # `duramen-diagnostics`
//!
//! Provides reporting for Cedar.

#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::ops::Range;

use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

#[derive(Debug, Default)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
    has_error: bool,
}

impl Diagnostics {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: Vec::new(),
            has_error: false,
        }
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        if !self.has_error && diagnostic.severity() == Severity::Error {
            self.has_error = true;
        }

        self.items.push(diagnostic);
    }

    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.has_error
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter()
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.items
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    severity: Severity,
    message: String,
    label: Option<(Range<u32>, String)>,
    context: Vec<(Range<u32>, String)>,
    notes: Vec<String>,
}

impl Diagnostic {
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn error<M: Into<String>>(message: M) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            label: None,
            context: Vec::new(),
            notes: Vec::new(),
        }
    }

    #[must_use]
    pub fn warning<M: Into<String>>(message: M) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            label: None,
            context: Vec::new(),
            notes: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_label<M: Into<String>>(mut self, range: Range<u32>, message: M) -> Self {
        self.label = Some((range, message.into()));
        self
    }

    #[must_use]
    pub fn with_context<M: Into<String>>(mut self, range: Range<u32>, message: M) -> Self {
        self.context.push((range, message.into()));
        self
    }

    #[must_use]
    pub fn with_note<N: Into<String>>(mut self, note: N) -> Self {
        self.notes.push(note.into());
        self
    }

    #[must_use]
    pub fn render(&self, path: &str, source: &str) -> String {
        let level = match self.severity {
            Severity::Error => Level::ERROR,
            Severity::Warning => Level::WARNING,
            Severity::Note => Level::NOTE,
            Severity::Help => Level::HELP,
        };

        let mut snippet = Snippet::source(source).path(path);

        if let Some((range, message)) = &self.label {
            let span = range.start as usize..range.end as usize;
            snippet = snippet.annotation(AnnotationKind::Primary.span(span).label(message));
        }

        for (range, message) in &self.context {
            let span = range.start as usize..range.end as usize;
            snippet = snippet.annotation(AnnotationKind::Context.span(span).label(message));
        }

        let title = level.primary_title(&self.message);

        let mut groups = vec![title.element(snippet)];
        for note in &self.notes {
            groups.push(Group::with_title(Level::NOTE.secondary_title(note)));
        }

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        renderer.render(&groups)
    }

    #[must_use]
    pub fn missing_effect(span: Range<u32>) -> Self {
        Self::error("missing effect").with_label(span, "expected `permit` or `forbid`")
    }

    #[must_use]
    pub fn missing_scope_variable(var: &str, span: Range<u32>) -> Self {
        Self::error(format!("missing `{var}` clause"))
            .with_label(span, format!("expected `{var}` clause"))
    }

    #[must_use]
    pub fn invalid_scope_operator(got: &str, expected: &str, span: Range<u32>) -> Self {
        Self::error(format!("invalid operator `{got}`"))
            .with_label(span, format!("expected {expected}"))
    }

    #[must_use]
    pub fn expected_entity(span: Range<u32>) -> Self {
        Self::error("expected entity reference")
            .with_label(span, "expected entity like `Type::\"id\"`")
    }

    #[must_use]
    pub fn expected_entity_or_slot(span: Range<u32>) -> Self {
        Self::error("expected entity reference or slot").with_label(
            span,
            "expected entity like `Type::\"id\"` or slot like `?principal`",
        )
    }

    #[must_use]
    pub fn invalid_slot_id(id: &str, span: Range<u32>) -> Self {
        Self::error(format!("invalid slot `?{id}`"))
            .with_label(span, "expected `?principal` or `?resource`")
    }

    #[must_use]
    pub fn invalid_integer(value: &str, span: Range<u32>) -> Self {
        Self::error(format!("invalid integer `{value}`")).with_label(span, "not a valid integer")
    }

    #[must_use]
    pub fn integer_overflow(value: &str, span: Range<u32>) -> Self {
        Self::error(format!("integer overflow: `{value}`"))
            .with_label(span, "value out of range for i64")
    }

    #[must_use]
    pub fn invalid_string_escape(reason: &str, span: Range<u32>) -> Self {
        Self::error(format!("invalid escape sequence: {reason}")).with_label(span, "invalid escape")
    }

    #[must_use]
    pub fn invalid_pattern(reason: &str, span: Range<u32>) -> Self {
        Self::error(format!("invalid pattern: {reason}")).with_label(span, "invalid pattern")
    }

    #[must_use]
    pub fn unknown_method(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("unknown method `{name}`")).with_label(span, "unknown method")
    }

    #[must_use]
    pub fn unknown_function(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("unknown function `{name}`")).with_label(span, "unknown function")
    }

    #[must_use]
    pub fn wrong_arity(name: &str, expected: usize, got: usize, span: Range<u32>) -> Self {
        Self::error(format!(
            "`{name}` expects {expected} argument(s), got {got}"
        ))
        .with_label(span, format!("expected {expected} argument(s)"))
    }

    #[must_use]
    pub fn reserved_identifier(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("`{name}` is a reserved identifier"))
            .with_label(span, "reserved identifier")
    }

    #[must_use]
    pub fn invalid_identifier(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("`{name}` is not a valid identifier"))
            .with_label(span, "invalid identifier")
    }

    #[must_use]
    pub fn duplicate_annotation(name: &str, span: Range<u32>, first: Range<u32>) -> Self {
        Self::error(format!("duplicate annotation `@{name}`"))
            .with_label(span, "duplicate annotation")
            .with_context(first, "first defined here")
    }

    #[must_use]
    pub fn duplicate_declaration(
        kind: &str,
        name: &str,
        span: Range<u32>,
        first: Range<u32>,
    ) -> Self {
        Self::error(format!("duplicate {kind} `{name}`"))
            .with_label(span, format!("duplicate {kind}"))
            .with_context(first, "first defined here")
    }

    #[must_use]
    pub fn invalid_type_reference(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("unknown type `{name}`")).with_label(span, "unknown type")
    }

    #[must_use]
    pub fn cyclic_type_definition(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("cyclic type definition `{name}`"))
            .with_label(span, "type references itself")
    }

    #[must_use]
    pub fn empty_node(expected: &str, span: Range<u32>) -> Self {
        Self::error(format!("expected {expected}")).with_label(span, format!("expected {expected}"))
    }

    #[must_use]
    pub fn missing_child(parent: &str, child: &str, span: Range<u32>) -> Self {
        Self::error(format!("{parent} missing {child}"))
            .with_label(span, format!("missing {child}"))
    }

    #[must_use]
    pub fn too_many_operators(op: &str, span: Range<u32>) -> Self {
        Self::error(format!("too many `{op}` operators"))
            .with_label(span, "too many consecutive operators")
    }

    #[must_use]
    pub fn duplicate_key(key: &str, span: Range<u32>, first: Range<u32>) -> Self {
        Self::error(format!("duplicate key `{key}`"))
            .with_label(span, "duplicate key")
            .with_context(first, "first defined here")
    }

    #[must_use]
    pub fn method_as_function(name: &str, span: Range<u32>) -> Self {
        Self::error(format!("`{name}` is a method, not a function"))
            .with_label(span, "method called as function")
            .with_note(format!("use `expr.{name}(...)` instead"))
    }
}
