use syntree::{FlavorDefault, Node, Span};

use super::SchemaSyntax;

mod nodes;
pub use nodes::*;

mod tokens;
pub use tokens::*;

mod types;
pub use types::*;

pub type SchemaNode<'a> = Node<'a, SchemaSyntax, FlavorDefault>;

pub trait AstNode<'a>: Sized {
    fn can_cast(kind: SchemaSyntax) -> bool;
    fn cast(node: SchemaNode<'a>) -> Option<Self>;
    fn syntax(&self) -> &SchemaNode<'a>;

    #[inline]
    fn span(&self) -> Span<u32> {
        *self.syntax().span()
    }
}

pub trait AstToken<'a>: Sized {
    fn can_cast(kind: SchemaSyntax) -> bool;
    fn cast(node: SchemaNode<'a>) -> Option<Self>;
    fn syntax(&self) -> &SchemaNode<'a>;

    #[inline]
    fn span(&self) -> Span<u32> {
        *self.syntax().span()
    }

    #[inline]
    fn text<'s>(&self, source: &'s str) -> &'s str {
        &source[self.syntax().span().range()]
    }
}
