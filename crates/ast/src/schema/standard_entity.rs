use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::common::Name;
use crate::schema::{AttributeDeclaration, TypeExpression};
use crate::{Error, FxBuildHasher, IndexMap, IndexSet};

/// A standard entity with parents, attributes, and optional tags.
#[derive(Clone, Debug)]
pub struct StandardEntity<'a> {
    parents: IndexSet<Name<'a>>,
    attributes: IndexMap<Cow<'a, str>, AttributeDeclaration<'a>>,
    tags: Option<TypeExpression<'a>>,
}

impl<'a> StandardEntity<'a> {
    /// Creates a standard entity definition.
    ///
    /// # Errors
    ///
    /// Returns an error if `parents` or `attributes` contain duplicates.
    pub fn new(
        parents: Vec<Name<'a>>,
        attributes: Vec<(Cow<'a, str>, AttributeDeclaration<'a>)>,
        tags: Option<TypeExpression<'a>>,
        span: Range<usize>,
    ) -> Result<Self, Error> {
        let mut parent_set = IndexSet::with_capacity_and_hasher(parents.len(), FxBuildHasher);

        for parent in parents {
            let (index, inserted) = parent_set.insert_full(parent);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(parent_set[index].basename().as_str()),
                    span,
                });
            }
        }

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

        Ok(Self {
            parents: parent_set,
            attributes: map,
            tags,
        })
    }

    /// Returns the parent entity types.
    pub fn parents(&self) -> impl Iterator<Item = &Name<'a>> {
        self.parents.iter()
    }

    /// Returns the attribute declaration for the given name.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&AttributeDeclaration<'a>> {
        self.attributes.get(name)
    }

    /// Returns an iterator over attribute name-declaration pairs.
    pub fn attributes(&self) -> impl Iterator<Item = (&str, &AttributeDeclaration<'a>)> {
        self.attributes.iter().map(|(key, value)| (&**key, value))
    }

    /// Returns the tag type expression, if any.
    #[must_use]
    pub const fn tags(&self) -> Option<&TypeExpression<'a>> {
        self.tags.as_ref()
    }
}
