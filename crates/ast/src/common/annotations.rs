use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use crate::common::Identifier;
use crate::{Error, FxBuildHasher, IndexMap};

/// The value of an annotation.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnnotationValue<'a> {
    Empty,
    String(Cow<'a, str>),
}

/// A collection of annotations keyed by identifier.
#[derive(Clone, Debug)]
pub struct Annotations<'a>(IndexMap<Identifier<'a>, AnnotationValue<'a>>);

impl<'a> Annotations<'a> {
    /// Creates annotations from key-value pairs.
    ///
    /// # Errors
    ///
    /// Returns an error if any key appears more than once.
    pub fn new(entries: Vec<(Identifier<'a>, AnnotationValue<'a>)>) -> Result<Self, Error> {
        let mut map = IndexMap::with_capacity_and_hasher(entries.len(), FxBuildHasher);
        for (key, value) in entries {
            if map.contains_key(&key) {
                return Err(Error::DuplicateKey {
                    key: String::from(key.as_str()),
                });
            }

            map.insert(key, value);
        }

        Ok(Self(map))
    }

    /// Creates an empty annotations collection.
    #[must_use]
    pub fn empty() -> Self {
        Self(IndexMap::default())
    }

    /// Returns the annotation value for the given key.
    #[must_use]
    pub fn get(&self, key: Identifier<'a>) -> Option<&AnnotationValue<'a>> {
        self.0.get(&key)
    }

    /// Returns an iterator over annotation key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (Identifier<'a>, &AnnotationValue<'a>)> {
        self.0.iter().map(|(key, value)| (*key, value))
    }
}
