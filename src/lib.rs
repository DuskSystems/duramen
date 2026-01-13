#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod cursor;
pub mod diagnostics;
pub mod policy;
pub mod schema;
