use crate::common::Annotations;
use crate::schema::{Optionality, TypeExpression};

/// An attribute declaration within a record or entity.
#[derive(Clone, Debug)]
pub struct AttributeDeclaration<'a> {
    annotations: Annotations<'a>,
    optionality: Optionality,
    definition: TypeExpression<'a>,
}

impl<'a> AttributeDeclaration<'a> {
    /// Creates an attribute declaration.
    #[must_use]
    pub const fn new(
        annotations: Annotations<'a>,
        optionality: Optionality,
        definition: TypeExpression<'a>,
    ) -> Self {
        Self {
            annotations,
            optionality,
            definition,
        }
    }

    /// Returns the attribute annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns whether the attribute is required or optional.
    #[must_use]
    pub const fn optionality(&self) -> Optionality {
        self.optionality
    }

    /// Returns the attribute type definition.
    #[must_use]
    pub const fn definition(&self) -> &TypeExpression<'a> {
        &self.definition
    }
}
