//! # `duramen-ast`
//!
//! Defines typed abstract syntax tree types for Cedar.
//!
//! ## Design
//!
//! - All required fields guaranteed present. Correct by construction.
//! - Owns all data. No references to source.

#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod common;
pub mod policy;
pub mod schema;
