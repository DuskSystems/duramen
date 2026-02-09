use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::common::Name;
use crate::schema::ContextType;
use crate::{Error, FxBuildHasher, IndexSet};

/// Specifies which principal and resource types an action applies to.
#[derive(Clone, Debug)]
pub struct AppliesTo<'a> {
    principal_types: IndexSet<Name<'a>>,
    resource_types: IndexSet<Name<'a>>,
    context: Option<ContextType<'a>>,
}

impl<'a> AppliesTo<'a> {
    /// Creates an applies-to clause.
    ///
    /// # Errors
    ///
    /// Returns an error if `principal_types` or `resource_types` contain
    /// duplicates.
    pub fn new(
        principal_types: Vec<Name<'a>>,
        resource_types: Vec<Name<'a>>,
        context: Option<ContextType<'a>>,
        span: Range<usize>,
    ) -> Result<Self, Error> {
        let mut principal_set =
            IndexSet::with_capacity_and_hasher(principal_types.len(), FxBuildHasher);

        for name in principal_types {
            let (index, inserted) = principal_set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(principal_set[index].basename().as_str()),
                    span,
                });
            }
        }

        let mut resource_set =
            IndexSet::with_capacity_and_hasher(resource_types.len(), FxBuildHasher);

        for name in resource_types {
            let (index, inserted) = resource_set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(resource_set[index].basename().as_str()),
                    span,
                });
            }
        }

        Ok(Self {
            principal_types: principal_set,
            resource_types: resource_set,
            context,
        })
    }

    /// Returns the principal type constraints.
    pub fn principal_types(&self) -> impl Iterator<Item = &Name<'a>> {
        self.principal_types.iter()
    }

    /// Returns the resource type constraints.
    pub fn resource_types(&self) -> impl Iterator<Item = &Name<'a>> {
        self.resource_types.iter()
    }

    /// Returns the context type, if any.
    #[must_use]
    pub const fn context(&self) -> Option<&ContextType<'a>> {
        self.context.as_ref()
    }
}
