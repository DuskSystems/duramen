#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod cursor;

mod lexer;
pub use lexer::Lexer;

pub(crate) mod lookup;

mod token;
pub use token::{Token, TokenKind};
