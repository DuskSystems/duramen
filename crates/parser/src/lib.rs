//! # `duramen-parser`
//!
//! Parses Cedar source code into a concrete syntax tree.
//!
//! ## Design
//!
//! - Recursive descent with Pratt parsing for expressions.
//! - Continues parsing at synchronization points on error.

#![no_std]

#[cfg(feature = "std")]
extern crate std;
