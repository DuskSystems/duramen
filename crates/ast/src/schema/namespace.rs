use alloc::vec::Vec;
use core::ops::Range;

use crate::common::{Annotations, Name};
use crate::schema::Declaration;

/// A namespace containing declarations.
#[derive(Clone, Debug)]
pub struct Namespace<'a> {
    annotations: Annotations<'a>,
    name: Option<Name<'a>>,
    declarations: Vec<Declaration<'a>>,
    span: Range<usize>,
}

impl<'a> Namespace<'a> {
    /// Creates a new namespace.
    #[must_use]
    pub const fn new(
        annotations: Annotations<'a>,
        name: Option<Name<'a>>,
        declarations: Vec<Declaration<'a>>,
        span: Range<usize>,
    ) -> Self {
        Self {
            annotations,
            name,
            declarations,
            span,
        }
    }

    /// Returns the namespace annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns the namespace name, if any.
    #[must_use]
    pub const fn name(&self) -> Option<&Name<'a>> {
        self.name.as_ref()
    }

    /// Returns the namespace declarations.
    #[must_use]
    pub fn declarations(&self) -> &[Declaration<'a>] {
        &self.declarations
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        &self.span
    }
}
