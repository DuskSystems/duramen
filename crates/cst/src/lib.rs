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
