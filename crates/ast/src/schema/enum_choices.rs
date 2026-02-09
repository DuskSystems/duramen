use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::{Error, FxBuildHasher, IndexSet, IndexSet1};

/// A set of enum variant names.
#[derive(Clone, Debug)]
pub struct EnumChoices<'a> {
    choices: IndexSet1<Cow<'a, str>>,
}

impl<'a> EnumChoices<'a> {
    /// Creates enum choices from variant names.
    ///
    /// # Errors
    ///
    /// Returns an error if `choices` is empty or contains duplicates.
    pub fn new(choices: Vec<Cow<'a, str>>, span: Range<usize>) -> Result<Self, Error> {
        let mut set = IndexSet::with_capacity_and_hasher(choices.len(), FxBuildHasher);

        for choice in choices {
            let (index, inserted) = set.insert_full(choice);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(&*set[index]),
                    span,
                });
            }
        }

        let set = IndexSet1::try_from(set).map_err(|_empty| Error::Empty { span })?;

        Ok(Self { choices: set })
    }

    /// Returns an iterator over variant names.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.choices.iter1().into_iter().map(AsRef::as_ref)
    }
}
