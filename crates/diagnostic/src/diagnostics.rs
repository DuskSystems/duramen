use alloc::vec::{IntoIter, Vec};
use core::slice::Iter;

use crate::Diagnostic;

/// Collection of diagnostics.
#[derive(Debug, Default)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Creates a new empty diagnostics collection.
    #[must_use]
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds a diagnostic to the collection.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of diagnostics.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns an iterator over the diagnostics.
    pub fn iter(&self) -> Iter<'_, Diagnostic> {
        self.items.iter()
    }

    /// Extends this collection with another.
    pub fn extend<I: IntoIterator<Item = Diagnostic>>(&mut self, iter: I) {
        self.items.extend(iter);
    }

    /// Consumes the collection and returns the diagnostics as a vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.items
    }
}

impl IntoIterator for Diagnostics {
    type IntoIter = IntoIter<Diagnostic>;
    type Item = Diagnostic;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type IntoIter = Iter<'a, Diagnostic>;
    type Item = &'a Diagnostic;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
