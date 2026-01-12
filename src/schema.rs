#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use alloc::string::String;
use core::error::Error;
use core::fmt;

use syntree::{FlavorDefault, Tree};

pub mod ast;
use ast::{AstNode as _, Declaration, Namespace, Schema as SchemaAst};

mod lexer;
pub use lexer::{SchemaLexer, SchemaToken};

mod parser;
pub use parser::SchemaParser;

mod syntax;
pub use syntax::SchemaSyntax;

type SchemaTree = Tree<SchemaSyntax, FlavorDefault>;

#[derive(Debug)]
pub struct SchemaErrors;

impl fmt::Display for SchemaErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl Error for SchemaErrors {}

#[derive(Debug)]
pub struct Schema {
    source: String,
    tree: SchemaTree,
}

impl Schema {
    pub fn parse(source: &str) -> Result<Self, SchemaErrors> {
        let parser = SchemaParser::new(source);
        let tree = parser.parse().map_err(|_err| SchemaErrors)?;
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
    pub const fn tree(&self) -> &SchemaTree {
        &self.tree
    }

    #[must_use]
    pub fn root(&self) -> Option<SchemaAst<'_>> {
        self.tree.first().and_then(SchemaAst::cast)
    }

    pub fn namespaces(&self) -> impl Iterator<Item = Namespace<'_>> {
        self.root().into_iter().flat_map(|root| root.namespaces())
    }

    pub fn declarations(&self) -> impl Iterator<Item = Declaration<'_>> {
        self.root().into_iter().flat_map(|root| root.declarations())
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

    #[cfg(feature = "serde")]
    pub fn to_serde_json_value(&self) -> Result<serde_json::Value, SchemaErrors> {
        todo!()
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.tree.walk() {
            if node.value().is_token() {
                f.write_str(&self.source[node.range()])?;
            }
        }

        Ok(())
    }
}
