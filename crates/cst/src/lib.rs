#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

use core::ops::Range;

use duramen_syntax::{Node, Syntax};

/// CST accessors, inspired by:
/// <https://rust-lang.github.io/rust-analyzer/syntax/ast/trait.AstNode.html>.
pub trait CstNode<'a>: Sized + 'a {
    fn can_cast(kind: Syntax) -> bool;
    fn cast(node: Node<'a>) -> Option<Self>;
    fn syntax(&self) -> Node<'a>;

    fn range(&self) -> Range<usize> {
        self.syntax().range()
    }

    fn text<'src>(&self, source: &'src str) -> &'src str {
        &source[self.range()]
    }
}
