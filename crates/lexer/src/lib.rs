//! # `duramen-lexer`
//!
//! Tokenizes Cedar source code into a stream of tokens.
//!
//! ## Design
//!
//! - Works with string slices without copying.
//! - Cannot fail. Unrecognized bytes become [`TokenKind::Unknown`].
//! - Produces a flat stream without tree structure.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod cursor;

mod lexer;
pub use lexer::Lexer;

mod token;
pub use token::{Token, TokenKind};
