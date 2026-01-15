#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

use alloc::vec::Vec;
use core::error::Error;
use core::fmt;

use syntree::{FlavorDefault, Tree};

use crate::diagnostics::Diagnostic;

pub mod ast;
use ast::{AstNode as _, Declaration, Namespace, Schema as SchemaAst};

#[cfg(any(feature = "serde", feature = "facet"))]
pub mod est;

mod lexer;
pub use lexer::{SchemaLexer, SchemaToken};

mod parser;
use parser::SchemaParser;

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
pub struct Schema<'a> {
    source: &'a str,
    tree: SchemaTree,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Schema<'a> {
    pub(crate) const fn new(
        source: &'a str,
        tree: SchemaTree,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        Self {
            source,
            tree,
            diagnostics,
        }
    }

    #[must_use]
    pub fn parse(source: &'a str) -> Self {
        SchemaParser::new(source).parse()
    }

    #[must_use]
    pub const fn source(&self) -> &'a str {
        self.source
    }

    #[must_use]
    pub const fn tree(&self) -> &SchemaTree {
        &self.tree
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    #[must_use]
    pub fn root(&self) -> Option<SchemaAst<'_>> {
        self.tree.first().and_then(SchemaAst::cast)
    }

    pub fn namespaces(&self) -> impl Iterator<Item = Namespace<'_>> + use<'_> {
        self.root().into_iter().flat_map(|root| root.namespaces())
    }

    pub fn declarations(&self) -> impl Iterator<Item = Declaration<'_>> + use<'_> {
        self.root().into_iter().flat_map(|root| root.declarations())
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<alloc::string::String, SchemaErrors> {
        let json = self.to_schema_fragment_json();
        serde_json::to_string(&json).map_err(|_err| SchemaErrors)
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, SchemaErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<alloc::string::String, SchemaErrors> {
        let json = self.to_schema_fragment_json();
        facet_json::to_string(&json).map_err(|_err| SchemaErrors)
    }

    #[cfg(feature = "serde")]
    #[must_use]
    pub fn to_serde_json_value(&self) -> serde_json::Value {
        let json = self.to_schema_fragment_json();
        serde_json::to_value(&json).unwrap_or(serde_json::Value::Null)
    }

    #[cfg(any(feature = "serde", feature = "facet"))]
    fn to_schema_fragment_json(&self) -> est::json::SchemaFragmentJson {
        let fragment = est::convert_schema(self);
        est::json::convert_to_json(&fragment)
    }
}

impl fmt::Display for Schema<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.tree.walk() {
            if node.value().is_token() {
                f.write_str(&self.source[node.range()])?;
            }
        }

        Ok(())
    }
}
