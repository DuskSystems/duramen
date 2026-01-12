#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use alloc::string::String;
use core::error::Error;
use core::fmt;

use syntree::{FlavorDefault, Tree};

pub mod ast;

mod lexer;
pub use lexer::{PolicyLexer, PolicyToken};

mod parser;
pub use parser::PolicyParser;

mod syntax;
pub use syntax::PolicySyntax;

type PolicyTree = Tree<PolicySyntax, FlavorDefault>;

#[derive(Debug)]
pub struct PolicyErrors;

impl fmt::Display for PolicyErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl Error for PolicyErrors {}

#[derive(Debug)]
pub struct PolicySet {
    source: String,
    tree: PolicyTree,
}

impl PolicySet {
    pub fn parse(source: &str) -> Result<Self, PolicyErrors> {
        let parser = PolicyParser::new(source);
        let tree = parser.parse().map_err(|_err| PolicyErrors)?;
        Ok(Self {
            source: String::from(source),
            tree,
        })
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub const fn tree(&self) -> &PolicyTree {
        &self.tree
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

    #[cfg(feature = "serde")]
    pub fn to_serde_json_value(&self) -> Result<serde_json::Value, PolicyErrors> {
        todo!()
    }
}

impl fmt::Display for PolicySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.tree.walk() {
            if node.value().is_token() {
                f.write_str(&self.source[node.range()])?;
            }
        }

        Ok(())
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
