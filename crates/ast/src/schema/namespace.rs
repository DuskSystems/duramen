use alloc::vec::Vec;

use super::action_decl::ActionDecl;
use super::entity_decl::EntityDecl;
use super::type_decl::TypeDecl;
use crate::common::{Annotations, Name};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Namespace {
    name: Option<Name>,
    entities: Vec<EntityDecl>,
    actions: Vec<ActionDecl>,
    types: Vec<TypeDecl>,
    annotations: Annotations,
}

impl Namespace {
    #[must_use]
    pub const fn new(
        name: Option<Name>,
        entities: Vec<EntityDecl>,
        actions: Vec<ActionDecl>,
        types: Vec<TypeDecl>,
        annotations: Annotations,
    ) -> Self {
        Self {
            name,
            entities,
            actions,
            types,
            annotations,
        }
    }

    #[must_use]
    pub const fn default_namespace(
        entities: Vec<EntityDecl>,
        actions: Vec<ActionDecl>,
        types: Vec<TypeDecl>,
    ) -> Self {
        Self {
            name: None,
            entities,
            actions,
            types,
            annotations: Annotations::new(),
        }
    }

    #[must_use]
    pub const fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.name.is_none()
    }

    #[must_use]
    pub fn entities(&self) -> &[EntityDecl] {
        &self.entities
    }

    #[must_use]
    pub fn actions(&self) -> &[ActionDecl] {
        &self.actions
    }

    #[must_use]
    pub fn types(&self) -> &[TypeDecl] {
        &self.types
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Schema {
    namespaces: Vec<Namespace>,
}

impl Schema {
    #[must_use]
    pub const fn new(namespaces: Vec<Namespace>) -> Self {
        Self { namespaces }
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self {
            namespaces: Vec::new(),
        }
    }

    #[must_use]
    pub fn namespaces(&self) -> &[Namespace] {
        &self.namespaces
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.namespaces.is_empty()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.namespaces.len()
    }

    #[must_use]
    pub fn into_namespaces(self) -> Vec<Namespace> {
        self.namespaces
    }
}
