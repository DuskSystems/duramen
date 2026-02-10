use alloc::string::String;

use crate::RESERVED_TYPE_NAMES;
use crate::common::{Annotations, Identifier};
use crate::error::Error;
use crate::schema::TypeExpression;

/// A declaration of a named type.
#[derive(Clone, Debug)]
pub struct TypeDeclaration<'a> {
    annotations: Annotations<'a>,
    name: Identifier<'a>,
    definition: TypeExpression<'a>,
}

impl<'a> TypeDeclaration<'a> {
    /// Creates a type declaration.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is invalid.
    pub fn new(
        annotations: Annotations<'a>,
        name: Identifier<'a>,
        definition: TypeExpression<'a>,
    ) -> Result<Self, Error> {
        if RESERVED_TYPE_NAMES.contains(&name.as_str()) {
            return Err(Error::ReservedTypeName {
                name: String::from(name.as_str()),
            });
        }

        Ok(Self {
            annotations,
            name,
            definition,
        })
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
}
