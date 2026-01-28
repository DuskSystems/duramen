//! # `duramen-cst`
//!
//! Defines concrete syntax tree types for Cedar.
//!
//! ## Design
//!
//! - Preserves all source including whitespace and comments.
//! - References the source text.
//! - May contain structural errors or missing fields.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use syntree::Error;
use syntree::{Builder, Flavor, FlavorDefault, Tree};

pub type PolicyTree = Tree<PolicySyntax, FlavorDefault>;
pub type PolicyBuilder = Builder<PolicySyntax>;

pub type SchemaTree = Tree<SchemaSyntax, FlavorDefault>;
pub type SchemaBuilder = Builder<SchemaSyntax>;

pub type Checkpoint = syntree::Checkpoint<<FlavorDefault as Flavor>::Pointer>;

mod syntax;
pub use syntax::policy::PolicySyntax;
pub use syntax::schema::SchemaSyntax;
