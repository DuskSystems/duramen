use alloc::string::String;
use core::ops::Range;

/// The kind of suggestion.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum SuggestionKind {
    Fix,
    Hint,
}

/// Suggested fix for a diagnostic.
#[derive(Clone, Debug)]
pub struct Suggestion {
    kind: SuggestionKind,
    span: Range<usize>,
    replacement: String,
    message: String,
}

impl Suggestion {
    /// Creates a fix suggestion.
    #[must_use]
    pub fn fix<R: Into<String>>(span: Range<usize>, replacement: R) -> Self {
        Self {
            kind: SuggestionKind::Fix,
            span,
            replacement: replacement.into(),
            message: String::new(),
        }
    }

    /// Creates a hint suggestion.
    #[must_use]
    pub fn hint<R: Into<String>>(span: Range<usize>, replacement: R) -> Self {
        Self {
            kind: SuggestionKind::Hint,
            span,
            replacement: replacement.into(),
            message: String::new(),
        }
    }

    /// Returns the suggestion kind.
    #[must_use]
    pub const fn kind(&self) -> SuggestionKind {
        self.kind
    }

    /// Returns the span this suggestion applies to.
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        &self.span
    }

    /// Returns the replacement text.
    #[must_use]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    /// Returns the suggestion message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Sets the suggestion message.
    #[must_use]
    pub fn with_message<M: Into<String>>(mut self, message: M) -> Self {
        self.message = message.into();
        self
    }
}
