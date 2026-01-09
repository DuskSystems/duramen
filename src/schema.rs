#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use core::error::Error;
use core::fmt;

mod lexer;
pub use lexer::{SchemaLexer, SchemaToken};

mod syntax;
pub use syntax::SchemaSyntax;

#[derive(Debug)]
pub struct SchemaErrors;

impl fmt::Display for SchemaErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema error")
    }
}

impl Error for SchemaErrors {}

#[derive(Debug)]
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
    pub fn to_serde_json(&self) -> Result<alloc::string::String, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<alloc::string::String, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json_value(&self) -> Result<serde_json::Value, SchemaErrors> {
        todo!()
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
