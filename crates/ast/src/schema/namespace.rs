use alloc::string::String;
use alloc::vec::Vec;

use crate::common::{Annotations, Identifier, Name};
use crate::error::Error;
use crate::schema::Declaration;
use crate::{FxBuildHasher, IndexSet};

/// A namespace containing declarations.
#[derive(Clone, Debug)]
pub struct Namespace<'a> {
    annotations: Annotations<'a>,
    name: Option<Name<'a>>,
    declarations: Vec<Declaration<'a>>,
}

impl<'a> Namespace<'a> {
    /// Creates a new namespace.
    ///
    /// # Errors
    ///
    /// Returns an error if `declarations` is invalid.
    pub fn new(
        annotations: Annotations<'a>,
        name: Option<Name<'a>>,
        declarations: Vec<Declaration<'a>>,
    ) -> Result<Self, Error> {
        let mut entity_names =
            IndexSet::<Identifier<'a>>::with_capacity_and_hasher(declarations.len(), FxBuildHasher);
        let mut action_names =
            IndexSet::<&str>::with_capacity_and_hasher(declarations.len(), FxBuildHasher);
        let mut type_names =
            IndexSet::<Identifier<'a>>::with_capacity_and_hasher(declarations.len(), FxBuildHasher);

        for declaration in &declarations {
            match declaration {
                Declaration::Entity(entity) => {
                    for ident in entity.names() {
                        if !entity_names.insert(*ident) {
                            return Err(Error::DuplicateKey {
                                key: String::from(ident.as_str()),
                            });
                        }
                    }
                }
                Declaration::Action(action) => {
                    for name in action.names() {
                        if !action_names.insert(name) {
                            return Err(Error::DuplicateKey {
                                key: String::from(name),
                            });
                        }
                    }
                }
                Declaration::Type(type_decl) => {
                    let ident = type_decl.name();

                    if !type_names.insert(ident) {
                        return Err(Error::DuplicateKey {
                            key: String::from(ident.as_str()),
                        });
                    }
                }
            }
        }

        Ok(Self {
            annotations,
            name,
            declarations,
        })
    }

    /// Returns the namespace annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns the namespace name, if any.
    #[must_use]
    pub const fn name(&self) -> Option<&Name<'a>> {
        self.name.as_ref()
    }

    /// Returns the namespace declarations.
    #[must_use]
    pub fn declarations(&self) -> &[Declaration<'a>] {
        &self.declarations
    }
}
