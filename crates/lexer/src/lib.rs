#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "TODO"
)]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod cursor;

mod lexer;
pub use lexer::Lexer;

mod token;
pub use token::{Token, TokenKind};
