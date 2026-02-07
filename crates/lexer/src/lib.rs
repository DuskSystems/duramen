#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod cursor;

mod error;
pub use error::LexerError;

mod lexer;
pub use lexer::Lexer;

mod token;
pub use token::{Token, TokenKind};
