use alloc::string::String;
use alloc::vec::Vec;

use crate::common::{Annotations, Identifier};
use crate::error::Error;
use crate::schema::EntityKind;
use crate::{FxBuildHasher, IndexSet, IndexSet1, RESERVED_TYPE_NAMES};

/// A declaration of one or more entity types.
#[derive(Clone, Debug)]
pub struct EntityDeclaration<'a> {
    annotations: Annotations<'a>,
    names: IndexSet1<Identifier<'a>>,
    kind: EntityKind<'a>,
}

impl<'a> EntityDeclaration<'a> {
    /// Creates an entity declaration.
    ///
    /// # Errors
    ///
    /// Returns an error if `names` is invalid.
    pub fn new(
        annotations: Annotations<'a>,
        names: Vec<Identifier<'a>>,
        kind: EntityKind<'a>,
    ) -> Result<Self, Error> {
        let mut set = IndexSet::with_capacity_and_hasher(names.len(), FxBuildHasher);

        for name in names {
            if RESERVED_TYPE_NAMES.contains(&name.as_str()) {
                return Err(Error::ReservedTypeName {
                    name: String::from(name.as_str()),
                });
            }

            let (index, inserted) = set.insert_full(name);

            if !inserted {
                return Err(Error::DuplicateKey {
                    key: String::from(set[index].as_str()),
                });
            }
        }

        let names = IndexSet1::try_from(set).map_err(|_empty| Error::Empty)?;

        Ok(Self {
            annotations,
            names,
            kind,
        })
    }

    /// Returns the entity annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns the entity type names.
    pub fn names(&self) -> impl Iterator<Item = &Identifier<'a>> {
        self.names.iter1().into_iter()
    }

    /// Returns the entity kind.
    #[must_use]
    pub const fn kind(&self) -> &EntityKind<'a> {
        &self.kind
    }
}
