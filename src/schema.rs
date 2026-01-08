#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use alloc::string::String;

use syntree::{FlavorDefault, Node, Tree};

mod lexer;
pub use lexer::{SchemaLexer, SchemaToken};

mod syntax;
pub use syntax::SchemaKind;

pub type SchemaTree = Tree<SchemaKind, FlavorDefault>;
pub type SchemaNode<'a> = Node<'a, SchemaKind, FlavorDefault>;

pub struct SchemaErrors;

pub struct Schema;

impl Schema {
    pub fn parse(_source: &str) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, SchemaErrors> {
        todo!()
    }
}
