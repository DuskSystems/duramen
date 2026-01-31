//! # `duramen-cst`
//!
//! Defines concrete syntax tree types for Cedar.
//!
//! ## Design
//!
//! - Preserves all source including whitespace and comments.
//! - References the source text.
//! - May contain structural errors or missing fields.

#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::ops::Range;

mod builder;
pub use builder::Builder;

pub mod policy;
pub mod schema;

mod tree;
pub use tree::{Node, Tree};

/// CST accessors, inspired by:
/// <https://rust-lang.github.io/rust-analyzer/syntax/ast/trait.AstNode.html>.
pub trait CstNode<'a>: Sized + 'a {
    type Syntax: Copy;

    fn can_cast(kind: Self::Syntax) -> bool;
    fn cast(node: Node<'a, Self::Syntax>) -> Option<Self>;
    fn syntax(&self) -> Node<'a, Self::Syntax>;

    fn range(&self) -> Range<usize> {
        self.syntax().range()
    }

    fn text<'s>(&self, source: &'s str) -> &'s str {
        &source[self.range()]
    }
}
