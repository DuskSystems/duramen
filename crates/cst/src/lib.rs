#![no_std]

#[cfg(feature = "std")]
extern crate std;

use syntree::{Builder, FlavorDefault, Tree};

pub type PolicyTree = Tree<PolicySyntax, FlavorDefault>;
pub type PolicyBuilder = Builder<PolicySyntax>;

pub type SchemaTree = Tree<SchemaSyntax, FlavorDefault>;
pub type SchemaBuilder = Builder<SchemaSyntax>;

mod syntax;
pub use syntax::policy::PolicySyntax;
pub use syntax::schema::SchemaSyntax;
