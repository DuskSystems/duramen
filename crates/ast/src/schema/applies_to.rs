use alloc::string::String;
use alloc::vec::Vec;

use crate::common::Name;
use crate::schema::ContextType;
use crate::{Error, FxBuildHasher, IndexSet};

/// Specifies which principal and resource types an action applies to.
#[derive(Clone, Debug)]
pub struct AppliesTo<'a> {
    principals: IndexSet<Name<'a>>,
    resources: IndexSet<Name<'a>>,
    context: Option<ContextType<'a>>,
}

impl<'a> AppliesTo<'a> {
    /// Creates an applies-to clause.
    ///
    /// # Errors
    ///
    /// Returns an error if `principals` or `resources` contain duplicates.
    pub fn new(
        principals: Vec<Name<'a>>,
        resources: Vec<Name<'a>>,
        context: Option<ContextType<'a>>,
    ) -> Result<Self, Error> {
        let mut principal_set = IndexSet::with_capacity_and_hasher(principals.len(), FxBuildHasher);

        for name in principals {
            let (index, inserted) = principal_set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(principal_set[index].basename().as_str()),
                });
            }
        }

        let mut resource_set = IndexSet::with_capacity_and_hasher(resources.len(), FxBuildHasher);

        for name in resources {
            let (index, inserted) = resource_set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(resource_set[index].basename().as_str()),
                });
            }
        }

        Ok(Self {
            principals: principal_set,
            resources: resource_set,
            context,
        })
    }

    /// Returns the principal type constraints.
    pub fn principals(&self) -> impl Iterator<Item = &Name<'a>> {
        self.principals.iter()
    }

    /// Returns the resource type constraints.
    pub fn resources(&self) -> impl Iterator<Item = &Name<'a>> {
        self.resources.iter()
    }

    /// Returns the context type, if any.
    #[must_use]
    pub const fn context(&self) -> Option<&ContextType<'a>> {
        self.context.as_ref()
    }
}
