#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod common;

mod error;
pub use error::ParseError;

mod policy;
pub use policy::PolicyParser;

mod schema;
pub use schema::SchemaParser;
