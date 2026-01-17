#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod cst;
pub(crate) mod cursor;
pub mod diagnostics;
#[cfg(any(feature = "serde", feature = "facet", feature = "prost"))]
pub(crate) mod escape;
pub mod policy;
pub mod schema;
