use alloc::vec::Vec;

use crate::policy::PatternElement;

/// A glob-like pattern used in `like` expressions.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Pattern<'a> {
    elements: Vec<PatternElement<'a>>,
}

impl<'a> Pattern<'a> {
    /// Creates a new pattern from elements.
    #[must_use]
    pub const fn new(elements: Vec<PatternElement<'a>>) -> Self {
        Self { elements }
    }

    /// Returns the pattern elements.
    #[must_use]
    pub fn elements(&self) -> &[PatternElement<'a>] {
        &self.elements
    }
}
