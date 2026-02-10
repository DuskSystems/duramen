use alloc::string::String;
use alloc::vec::Vec;

use crate::error::Error;
use crate::policy::EntityReference;
use crate::{FxBuildHasher, IndexSet, IndexSet1};

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
    /// Returns an error if `actions` is invalid.
    pub fn new(actions: Vec<EntityReference<'a>>) -> Result<Self, Error> {
        let mut set = IndexSet::with_capacity_and_hasher(actions.len(), FxBuildHasher);

        for action in actions {
            let (index, inserted) = set.insert_full(action);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(set[index].id()),
                });
            }
        }

        let set = IndexSet1::try_from(set).map_err(|_empty| Error::Empty)?;

        Ok(Self { actions: set })
    }

    /// Returns an iterator over the entity references.
    pub fn iter(&self) -> impl Iterator<Item = &EntityReference<'a>> {
        self.actions.iter1().into_iter()
    }
}
