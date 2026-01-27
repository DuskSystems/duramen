#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod cursor;

mod lexer;
pub use lexer::Lexer;

mod lookup;

mod token;
pub use token::{Token, TokenKind};
