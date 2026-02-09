use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::common::Name;
use crate::{Error, FxBuildHasher, IndexSet, IndexSet1};

/// A set of entity type names.
#[derive(Clone, Debug)]
pub struct EntityTypeSet<'a> {
    types: IndexSet1<Name<'a>>,
}

impl<'a> EntityTypeSet<'a> {
    /// Creates an entity type set from type names.
    ///
    /// # Errors
    ///
    /// Returns an error if `types` is empty or contains duplicates.
    pub fn new(types: Vec<Name<'a>>, span: Range<usize>) -> Result<Self, Error> {
        let mut set = IndexSet::with_capacity_and_hasher(types.len(), FxBuildHasher);

        for name in types {
            let (index, inserted) = set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(set[index].basename().as_str()),
                    span,
                });
            }
        }

        let set = IndexSet1::try_from(set).map_err(|_empty| Error::Empty { span })?;

        Ok(Self { types: set })
    }

    /// Returns whether the set contains the given name.
    #[must_use]
    pub fn contains(&self, name: &Name<'a>) -> bool {
        self.types.contains(name)
    }

    /// Returns an iterator over the type names.
    pub fn iter(&self) -> impl Iterator<Item = &Name<'a>> {
        self.types.iter1().into_iter()
    }
}
