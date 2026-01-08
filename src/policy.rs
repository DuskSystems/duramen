#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use alloc::string::String;

use syntree::{FlavorDefault, Node, Tree};

mod lexer;
pub use lexer::{PolicyLexer, PolicyToken};

mod syntax;
pub use syntax::PolicyKind;

pub type PolicyTree = Tree<PolicyKind, FlavorDefault>;
pub type PolicyNode<'a> = Node<'a, PolicyKind, FlavorDefault>;

pub struct PolicyErrors;

pub struct PolicySet;

impl PolicySet {
    pub fn parse(_source: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, PolicyErrors> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Policy;

impl Policy {
    pub fn parse(_source: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, PolicyErrors> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Template;

impl Template {
    pub fn parse(_source: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, PolicyErrors> {
        todo!()
    }
}
