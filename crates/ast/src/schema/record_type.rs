use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::ops::Range;

use crate::schema::AttributeDeclaration;
use crate::{Error, FxBuildHasher, IndexMap};

/// An ordered record of attribute declarations.
#[derive(Clone, Debug)]
pub struct RecordType<'a> {
    attributes: IndexMap<Cow<'a, str>, AttributeDeclaration<'a>>,
}

impl<'a> RecordType<'a> {
    /// Creates a record type from attribute declarations.
    ///
    /// # Errors
    ///
    /// Returns an error if any attribute name appears more than once.
    pub fn new(
        attributes: Vec<(Cow<'a, str>, AttributeDeclaration<'a>)>,
        span: Range<usize>,
    ) -> Result<Self, Error> {
        let mut map = IndexMap::with_capacity_and_hasher(attributes.len(), FxBuildHasher);

        for (key, value) in attributes {
            if map.contains_key(&*key) {
                return Err(Error::DuplicateKey {
                    key: key.into_owned(),
                    span,
                });
            }

            map.insert(key, value);
        }

        Ok(Self { attributes: map })
    }

    /// Creates an empty record type.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            attributes: IndexMap::default(),
        }
    }

    /// Returns the attribute declaration for the given name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&AttributeDeclaration<'a>> {
        self.attributes.get(name)
    }

    /// Returns an iterator over attribute name-declaration pairs.
    pub fn attributes(&self) -> impl Iterator<Item = (&str, &AttributeDeclaration<'a>)> {
        self.attributes.iter().map(|(key, value)| (&**key, value))
    }
}
