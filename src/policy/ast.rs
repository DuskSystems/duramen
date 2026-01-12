use syntree::{FlavorDefault, Node, Span};

use super::PolicySyntax;

mod enums;
pub use enums::*;

mod expressions;
pub use expressions::*;

mod nodes;
pub use nodes::*;

mod tokens;
pub use tokens::*;

pub type PolicyNode<'a> = Node<'a, PolicySyntax, FlavorDefault>;

pub trait AstNode<'a>: Sized {
    fn can_cast(kind: PolicySyntax) -> bool;
    fn cast(node: PolicyNode<'a>) -> Option<Self>;
    fn syntax(&self) -> &PolicyNode<'a>;

    #[inline]
    fn span(&self) -> Span<u32> {
        *self.syntax().span()
    }
}

pub trait AstToken<'a>: Sized {
    fn can_cast(kind: PolicySyntax) -> bool;
    fn cast(node: PolicyNode<'a>) -> Option<Self>;
    fn syntax(&self) -> &PolicyNode<'a>;

    #[inline]
    fn span(&self) -> Span<u32> {
        *self.syntax().span()
    }

    #[inline]
    fn text<'s>(&self, source: &'s str) -> &'s str {
        &source[self.syntax().span().range()]
    }
}
