use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Group, Level, Patch, Renderer, Snippet};

use crate::suggestion::Suggestion;

/// The kind of diagnostic.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

impl From<DiagnosticKind> for Level<'static> {
    fn from(value: DiagnosticKind) -> Self {
        match value {
            DiagnosticKind::Error => Level::ERROR,
            DiagnosticKind::Warning => Level::WARNING,
        }
    }
}

/// A diagnostic message with source location and optional suggestions.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    kind: DiagnosticKind,
    message: String,
    label: Option<(Range<usize>, String)>,
    context: Vec<(Range<usize>, String)>,
    notes: Vec<String>,
    help: Vec<String>,
    suggestions: Vec<Suggestion>,
}

impl Diagnostic {
    /// Creates an error diagnostic.
    #[must_use]
    pub fn error<M: Into<String>>(message: M) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            message: message.into(),
            label: None,
            context: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning<M: Into<String>>(message: M) -> Self {
        Self {
            kind: DiagnosticKind::Warning,
            message: message.into(),
            label: None,
            context: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Returns the diagnostic kind.
    #[must_use]
    pub const fn kind(&self) -> DiagnosticKind {
        self.kind
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the primary span if present.
    #[must_use]
    pub fn span(&self) -> Option<&Range<usize>> {
        self.label.as_ref().map(|(range, _)| range)
    }

    /// Returns the primary label if present.
    #[must_use]
    pub fn label(&self) -> Option<(&Range<usize>, &str)> {
        self.label
            .as_ref()
            .map(|(range, message)| (range, message.as_str()))
    }

    /// Returns the context labels.
    #[must_use]
    pub fn context(&self) -> &[(Range<usize>, String)] {
        &self.context
    }

    /// Returns the notes.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns the help messages.
    #[must_use]
    pub fn help(&self) -> &[String] {
        &self.help
    }

    /// Returns the suggestions.
    #[must_use]
    pub fn suggestions(&self) -> &[Suggestion] {
        &self.suggestions
    }

    /// Sets the primary label for the diagnostic.
    ///
    /// Note: Calling this multiple times will replace the previous label.
    #[must_use]
    pub fn with_label<M: Into<String>>(mut self, range: Range<usize>, message: M) -> Self {
        self.label = Some((range, message.into()));
        self
    }

    /// Adds a contextual label to the diagnostic.
    #[must_use]
    pub fn with_context<M: Into<String>>(mut self, range: Range<usize>, message: M) -> Self {
        self.context.push((range, message.into()));
        self
    }

    /// Adds a note to the diagnostic.
    #[must_use]
    pub fn with_note<N: Into<String>>(mut self, note: N) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Adds a help message to the diagnostic.
    #[must_use]
    pub fn with_help<H: Into<String>>(mut self, help: H) -> Self {
        self.help.push(help.into());
        self
    }

    /// Adds a suggestion to the diagnostic.
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Renders the diagnostic to a string using annotate-snippets.
    #[must_use]
    pub fn render(&self, path: &str, source: &str) -> String {
        let mut snippet = Snippet::source(source).path(path);

        if let Some((range, message)) = &self.label {
            snippet =
                snippet.annotation(AnnotationKind::Primary.span(range.clone()).label(message));
        }

        for (range, message) in &self.context {
            snippet =
                snippet.annotation(AnnotationKind::Context.span(range.clone()).label(message));
        }

        let level: Level<'_> = self.kind.into();
        let title = level.primary_title(&self.message);

        let mut groups = vec![title.element(snippet)];
        for note in &self.notes {
            groups.push(Group::with_title(Level::NOTE.secondary_title(note)));
        }

        for help in &self.help {
            groups.push(Group::with_title(Level::HELP.secondary_title(help)));
        }

        for suggestion in &self.suggestions {
            groups.push(
                Group::with_title(Level::HELP.secondary_title(suggestion.message())).element(
                    Snippet::source(source).patch(Patch::new(
                        suggestion.span().clone(),
                        suggestion.replacement(),
                    )),
                ),
            );
        }

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        renderer.render(&groups)
    }
}
