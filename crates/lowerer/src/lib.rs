#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod common;

mod error;

mod policy;
pub use policy::PolicyLowerer;

mod schema;
pub use schema::SchemaLowerer;
