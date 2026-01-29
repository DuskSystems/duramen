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
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use syntree::Error;
use syntree::{Builder, Flavor, FlavorDefault, Node, Tree};

use crate::syntax::policy::PolicySyntax;
use crate::syntax::schema::SchemaSyntax;

pub type PolicyTree = Tree<PolicySyntax, FlavorDefault>;
pub type PolicyBuilder = Builder<PolicySyntax>;
pub type PolicyNode<'a> = Node<'a, PolicySyntax, FlavorDefault>;

pub type SchemaTree = Tree<SchemaSyntax, FlavorDefault>;
pub type SchemaBuilder = Builder<SchemaSyntax>;
pub type SchemaNode<'a> = Node<'a, SchemaSyntax, FlavorDefault>;

pub type Checkpoint = syntree::Checkpoint<<FlavorDefault as Flavor>::Pointer>;

pub mod accessors;
pub mod syntax;
