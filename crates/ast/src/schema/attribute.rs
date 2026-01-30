use super::types::Type;
use crate::common::{Annotations, Id};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AttributeDecl {
    name: Id,
    required: bool,
    attr_type: Type,
    annotations: Annotations,
}

impl AttributeDecl {
    #[must_use]
    pub const fn new(name: Id, required: bool, attr_type: Type, annotations: Annotations) -> Self {
        Self {
            name,
            required,
            attr_type,
            annotations,
        }
    }

    #[must_use]
    pub const fn required(name: Id, attr_type: Type) -> Self {
        Self {
            name,
            required: true,
            attr_type,
            annotations: Annotations::new(),
        }
    }

    #[must_use]
    pub const fn optional(name: Id, attr_type: Type) -> Self {
        Self {
            name,
            required: false,
            attr_type,
            annotations: Annotations::new(),
        }
    }

    #[must_use]
    pub const fn name(&self) -> &Id {
        &self.name
    }

    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    #[must_use]
    pub const fn is_optional(&self) -> bool {
        !self.required
    }

    #[must_use]
    pub const fn attr_type(&self) -> &Type {
        &self.attr_type
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    #[must_use]
    pub fn into_parts(self) -> (Id, bool, Type, Annotations) {
        (self.name, self.required, self.attr_type, self.annotations)
    }
}
