//! # `duramen-est`
//!
//! Defines external syntax tree types for Cedar serialization.
//!
//! ## Design
//!
//! - Converts to/from AST infallibly.
//! - Normalizes operators into a canonical form.
//! - Supports JSON and Protobuf formats.

#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod convert;
pub mod json;
