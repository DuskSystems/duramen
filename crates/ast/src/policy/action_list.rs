use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::policy::EntityReference;
use crate::{Error, FxBuildHasher, IndexSet, IndexSet1};

/// A set of entity references in an action constraint.
#[derive(Clone, Debug)]
pub struct ActionList<'a> {
    actions: IndexSet1<EntityReference<'a>>,
}

impl<'a> ActionList<'a> {
    /// Creates an action list from entity references.
    ///
    /// # Errors
    ///
    /// Returns an error if `actions` is empty or contains duplicates.
    pub fn new(actions: Vec<EntityReference<'a>>, span: Range<usize>) -> Result<Self, Error> {
        let mut set = IndexSet::with_capacity_and_hasher(actions.len(), FxBuildHasher);

        for action in actions {
            let (index, inserted) = set.insert_full(action);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(set[index].id()),
                    span,
                });
            }
        }

        let set = IndexSet1::try_from(set).map_err(|_empty| Error::Empty { span })?;

        Ok(Self { actions: set })
    }

    /// Returns an iterator over the entity references.
    pub fn iter(&self) -> impl Iterator<Item = &EntityReference<'a>> {
        self.actions.iter1().into_iter()
    }
}
