#![no_std]

#[cfg(any(feature = "serde", feature = "facet"))]
extern crate alloc;

pub mod cursor;
pub mod policy;
pub mod schema;
