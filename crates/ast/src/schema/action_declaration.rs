use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use crate::common::Annotations;
use crate::schema::{ActionReference, AppliesTo, AttributeDeclaration};
use crate::{Error, FxBuildHasher, IndexMap, IndexSet, IndexSet1};

/// A declaration of one or more actions.
#[derive(Clone, Debug)]
pub struct ActionDeclaration<'a> {
    annotations: Annotations<'a>,
    names: IndexSet1<Cow<'a, str>>,
    parents: IndexSet<ActionReference<'a>>,
    applies_to: Option<AppliesTo<'a>>,
    attributes: IndexMap<Cow<'a, str>, AttributeDeclaration<'a>>,
}

impl<'a> ActionDeclaration<'a> {
    /// Creates an action declaration.
    ///
    /// # Errors
    ///
    /// Returns an error if `names` is empty, or `names`, `parents`, or
    /// `attributes` contain duplicates.
    pub fn new(
        annotations: Annotations<'a>,
        names: Vec<Cow<'a, str>>,
        parents: Vec<ActionReference<'a>>,
        applies_to: Option<AppliesTo<'a>>,
        attributes: Vec<(Cow<'a, str>, AttributeDeclaration<'a>)>,
    ) -> Result<Self, Error> {
        let mut name_set = IndexSet::with_capacity_and_hasher(names.len(), FxBuildHasher);

        for name in names {
            let (index, inserted) = name_set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(&*name_set[index]),
                });
            }
        }

        let names = IndexSet1::try_from(name_set).map_err(|_empty| Error::Empty)?;

        let mut parent_set = IndexSet::with_capacity_and_hasher(parents.len(), FxBuildHasher);

        for parent in parents {
            let (index, inserted) = parent_set.insert_full(parent);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(parent_set[index].id()),
                });
            }
        }

        let mut map = IndexMap::with_capacity_and_hasher(attributes.len(), FxBuildHasher);

        for (key, value) in attributes {
            if map.contains_key(&*key) {
                return Err(Error::DuplicateKey {
                    key: key.into_owned(),
                });
            }

            map.insert(key, value);
        }

        Ok(Self {
            annotations,
            names,
            parents: parent_set,
            applies_to,
            attributes: map,
        })
    }

    /// Returns the action annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns the action names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.names.iter1().into_iter().map(AsRef::as_ref)
    }

    /// Returns the parent action references.
    pub fn parents(&self) -> impl Iterator<Item = &ActionReference<'a>> {
        self.parents.iter()
    }

    /// Returns the applies-to clause, if any.
    #[must_use]
    pub const fn applies_to(&self) -> Option<&AppliesTo<'a>> {
        self.applies_to.as_ref()
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
}
