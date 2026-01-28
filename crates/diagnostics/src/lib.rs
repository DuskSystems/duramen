//! # `duramen-diagnostics`
//!
//! Provides reporting for Cedar.

#![expect(dead_code, reason = "TODO")]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

/// A diagnostic message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    severity: Severity,
    code: Option<&'static str>,
    message: String,
    label: Option<(Range<usize>, String)>,
    context: Vec<(Range<usize>, String)>,
    notes: Vec<String>,
}

impl Diagnostic {
    /// Returns the severity level.
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the error code.
    #[must_use]
    pub const fn code(&self) -> Option<&'static str> {
        self.code
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    fn error<M: Into<String>>(message: M) -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            label: None,
            context: Vec::new(),
            notes: Vec::new(),
        }
    }

    fn warning<M: Into<String>>(message: M) -> Self {
        Self {
            severity: Severity::Warning,
            code: None,
            message: message.into(),
            label: None,
            context: Vec::new(),
            notes: Vec::new(),
        }
    }

    const fn with_code(mut self, code: &'static str) -> Self {
        self.code = Some(code);
        self
    }

    fn with_label<M: Into<String>>(mut self, range: Range<usize>, message: M) -> Self {
        self.label = Some((range, message.into()));
        self
    }

    fn with_context<M: Into<String>>(mut self, range: Range<usize>, message: M) -> Self {
        self.context.push((range, message.into()));
        self
    }

    fn with_note<N: Into<String>>(mut self, note: N) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Renders the diagnostic.
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
            snippet =
                snippet.annotation(AnnotationKind::Primary.span(range.clone()).label(message));
        }

        for (range, message) in &self.context {
            snippet =
                snippet.annotation(AnnotationKind::Context.span(range.clone()).label(message));
        }

        let mut title = level.primary_title(&self.message);
        if let Some(code) = self.code {
            title = title.id(code);
        }

        let mut groups = vec![title.element(snippet)];
        for note in &self.notes {
            groups.push(Group::with_title(Level::NOTE.secondary_title(note)));
        }

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        renderer.render(&groups)
    }
}
