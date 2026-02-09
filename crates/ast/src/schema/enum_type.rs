use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use crate::{Error, FxBuildHasher, IndexSet, IndexSet1};

/// An enum type with a set of variant names.
#[derive(Clone, Debug)]
pub struct EnumType<'a> {
    variants: IndexSet1<Cow<'a, str>>,
}

impl<'a> EnumType<'a> {
    /// Creates an enum type from variant names.
    ///
    /// # Errors
    ///
    /// Returns an error if `variants` is empty or contains duplicates.
    pub fn new(variants: Vec<Cow<'a, str>>) -> Result<Self, Error> {
        let mut set = IndexSet::with_capacity_and_hasher(variants.len(), FxBuildHasher);

        for variant in variants {
            let (index, inserted) = set.insert_full(variant);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(&*set[index]),
                });
            }
        }

        let set = IndexSet1::try_from(set).map_err(|_empty| Error::Empty)?;

        Ok(Self { variants: set })
    }

    /// Returns an iterator over variant names.
    pub fn variants(&self) -> impl Iterator<Item = &str> {
        self.variants.iter1().into_iter().map(AsRef::as_ref)
    }
}
