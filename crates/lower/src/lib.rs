//! # `duramen-lower`
//!
//! Transforms concrete syntax trees into abstract syntax trees.
//!
//! ## Design
//!
//! - Produces owned AST nodes.
//! - Validates structure.
//! - Collects diagnostics for semantic errors/warnings.

#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod policy;
pub use policy::PolicyLowerer;

mod schema;
pub use schema::SchemaLowerer;

pub mod unescape;
