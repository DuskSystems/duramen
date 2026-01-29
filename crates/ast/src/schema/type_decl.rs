use super::types::Type;
use crate::common::{Annotations, Id};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TypeDecl {
    name: Id,
    type_def: Type,
    annotations: Annotations,
}

impl TypeDecl {
    #[must_use]
    pub const fn new(name: Id, type_def: Type, annotations: Annotations) -> Self {
        Self {
            name,
            type_def,
            annotations,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &Id {
        &self.name
    }

    #[must_use]
    pub const fn type_def(&self) -> &Type {
        &self.type_def
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    #[must_use]
    pub fn into_parts(self) -> (Id, Type, Annotations) {
        (self.name, self.type_def, self.annotations)
    }
}
