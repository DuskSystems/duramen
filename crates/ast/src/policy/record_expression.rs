use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::policy::Expression;
use crate::{Error, FxBuildHasher, IndexMap};

/// An ordered record of key-value expression pairs.
#[derive(Clone, Debug)]
pub struct RecordExpression<'a> {
    entries: IndexMap<Cow<'a, str>, Expression<'a>>,
}

impl<'a> RecordExpression<'a> {
    /// Creates a record expression from key-value pairs.
    ///
    /// # Errors
    ///
    /// Returns an error if any key appears more than once.
    pub fn new(entries: Vec<(Cow<'a, str>, Expression<'a>)>) -> Result<Self, Error> {
        let mut map = IndexMap::with_capacity_and_hasher(entries.len(), FxBuildHasher);

        for (key, value) in entries {
            if map.contains_key(&*key) {
                return Err(Error::DuplicateKey {
                    key: key.into_owned(),
                });
            }

            map.insert(key, value);
        }

        Ok(Self { entries: map })
    }

    /// Creates an empty record expression.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            entries: IndexMap::default(),
        }
    }

    /// Returns the expression for the given key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Expression<'a>> {
        self.entries.get(key)
    }

    /// Returns an iterator over key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Expression<'a>)> {
        self.entries.iter().map(|(key, value)| (&**key, value))
    }
}
