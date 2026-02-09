use core::ops::Range;

use crate::common::{Annotations, Identifier};
use crate::schema::TypeExpression;

/// A declaration of a named type.
#[derive(Clone, Debug)]
pub struct TypeDeclaration<'a> {
    annotations: Annotations<'a>,
    name: Identifier<'a>,
    definition: TypeExpression<'a>,
    span: Range<usize>,
}

impl<'a> TypeDeclaration<'a> {
    /// Creates a type declaration.
    #[must_use]
    pub const fn new(
        annotations: Annotations<'a>,
        name: Identifier<'a>,
        definition: TypeExpression<'a>,
        span: Range<usize>,
    ) -> Self {
        Self {
            annotations,
            name,
            definition,
            span,
        }
    }

    /// Returns the type annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns the type name.
    #[must_use]
    pub const fn name(&self) -> Identifier<'a> {
        self.name
    }

    /// Returns the type definition.
    #[must_use]
    pub const fn definition(&self) -> &TypeExpression<'a> {
        &self.definition
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> &Range<usize> {
        &self.span
    }
}
