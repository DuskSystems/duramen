use super::types::Type;
use crate::common::Id;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AttributeDecl {
    name: Id,
    required: bool,
    attr_type: Type,
}

impl AttributeDecl {
    #[must_use]
    pub const fn new(name: Id, required: bool, attr_type: Type) -> Self {
        Self {
            name,
            required,
            attr_type,
        }
    }

    #[must_use]
    pub const fn required(name: Id, attr_type: Type) -> Self {
        Self {
            name,
            required: true,
            attr_type,
        }
    }

    #[must_use]
    pub const fn optional(name: Id, attr_type: Type) -> Self {
        Self {
            name,
            required: false,
            attr_type,
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
    pub fn into_parts(self) -> (Id, bool, Type) {
        (self.name, self.required, self.attr_type)
    }
}
