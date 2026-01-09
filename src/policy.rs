#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use core::error::Error;
use core::fmt;

mod lexer;
pub use lexer::{PolicyLexer, PolicyToken};

mod syntax;
pub use syntax::PolicyTokenKind;

#[derive(Debug)]
pub struct PolicyErrors;

impl fmt::Display for PolicyErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl Error for PolicyErrors {}

#[derive(Debug)]
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
    pub fn to_serde_json(&self) -> Result<alloc::string::String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<alloc::string::String, PolicyErrors> {
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn to_serde_json(&self) -> Result<alloc::string::String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<alloc::string::String, PolicyErrors> {
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
    pub fn to_serde_json(&self) -> Result<alloc::string::String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<alloc::string::String, PolicyErrors> {
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
