//! # `duramen-est`
//!
//! Defines external syntax tree types for Cedar serialization.
//!
//! ## Design
//!
//! - Converts to/from AST.
//! - Normalizes operators into a canonical form.
//! - Supports JSON and Protobuf formats.

#![no_std]

#[cfg(feature = "std")]
extern crate std;
