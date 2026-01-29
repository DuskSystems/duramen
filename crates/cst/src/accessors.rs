//! CST accessors, inspired by:
//! <https://rust-lang.github.io/rust-analyzer/syntax/ast/trait.AstNode.html>.

use core::ops::Range;

use syntree::{FlavorDefault, Node};

pub mod policy;
pub mod schema;

pub trait CstNode<'a>: Sized + 'a {
    type Syntax: Copy;

    fn can_cast(kind: Self::Syntax) -> bool;
    fn cast(node: Node<'a, Self::Syntax, FlavorDefault>) -> Option<Self>;
    fn syntax(&self) -> Node<'a, Self::Syntax, FlavorDefault>;

    fn range(&self) -> Range<usize> {
        self.syntax().range()
    }

    fn text<'s>(&self, source: &'s str) -> &'s str {
        &source[self.range()]
    }
}
