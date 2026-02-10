#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
pub use error::EscapeError;

mod escaper;
pub use escaper::Escaper;
