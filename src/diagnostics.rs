use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub range: Range<usize>,
    pub message: String,
    pub primary: bool,
}

impl Label {
    #[must_use]
    pub fn primary<M: Into<String>>(range: Range<usize>, message: M) -> Self {
        Self {
            range,
            message: message.into(),
            primary: true,
        }
    }

    #[must_use]
    pub fn secondary<M: Into<String>>(range: Range<usize>, message: M) -> Self {
        Self {
            range,
            message: message.into(),
            primary: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub range: Range<usize>,
    pub replacement: String,
    pub message: String,
}

impl Suggestion {
    #[must_use]
    pub fn new<R: Into<String>, M: Into<String>>(
        range: Range<usize>,
        replacement: R,
        message: M,
    ) -> Self {
        Self {
            range,
            replacement: replacement.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<&'static str>,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub suggestions: Vec<Suggestion>,
}

impl Diagnostic {
    #[must_use]
    pub fn error<M: Into<String>>(message: M) -> Self {
        Self {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    #[must_use]
    pub fn warning<M: Into<String>>(message: M) -> Self {
        Self {
            severity: Severity::Warning,
            code: None,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    #[must_use]
    pub const fn with_code(mut self, code: &'static str) -> Self {
        self.code = Some(code);
        self
    }

    #[must_use]
    pub fn with_label<M: Into<String>>(mut self, range: Range<usize>, message: M) -> Self {
        self.labels.push(Label::primary(range, message));
        self
    }

    #[must_use]
    pub fn with_secondary_label<M: Into<String>>(
        mut self,
        range: Range<usize>,
        message: M,
    ) -> Self {
        self.labels.push(Label::secondary(range, message));
        self
    }

    #[must_use]
    pub fn with_note<N: Into<String>>(mut self, note: N) -> Self {
        self.notes.push(note.into());
        self
    }

    #[must_use]
    pub fn with_suggestion<R: Into<String>, M: Into<String>>(
        mut self,
        range: Range<usize>,
        replacement: R,
        message: M,
    ) -> Self {
        self.suggestions
            .push(Suggestion::new(range, replacement, message));
        self
    }

    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, Severity::Error)
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
        for label in &self.labels {
            let kind = if label.primary {
                AnnotationKind::Primary
            } else {
                AnnotationKind::Context
            };

            snippet = snippet.annotation(kind.span(label.range.clone()).label(&label.message));
        }

        let mut title = level.primary_title(&self.message);
        if let Some(code) = self.code {
            title = title.id(code);
        }

        let mut report = vec![title.element(snippet)];
        for note in &self.notes {
            report.push(Group::with_title(Level::NOTE.secondary_title(note)));
        }

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        renderer.render(&report)
    }
}
