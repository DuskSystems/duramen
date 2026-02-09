use alloc::vec::Vec;

use crate::policy::PatternElement;

/// A glob-like pattern used in `like` expressions.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Pattern {
    elements: Vec<PatternElement>,
}

impl Pattern {
    /// Creates a new pattern from elements.
    #[must_use]
    pub const fn new(elements: Vec<PatternElement>) -> Self {
        Self { elements }
    }

    /// Returns the pattern elements.
    #[must_use]
    pub fn elements(&self) -> &[PatternElement] {
        &self.elements
    }
}
