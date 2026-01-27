#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod cursor;

mod lexer;
pub use lexer::Lexer;

mod token;
pub use token::{Token, TokenKind};
