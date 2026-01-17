use core::ops::Range;

use super::PolicySyntax;
use crate::cst::Node;

mod enums;
pub use enums::*;

mod expressions;
pub use expressions::*;

mod nodes;
pub use nodes::*;

mod tokens;
pub use tokens::*;

pub type PolicyNode<'a> = Node<'a, PolicySyntax>;

macro_rules! ast_node {
    ($(#[$meta:meta])* $name:ident, $kind:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            node: PolicyNode<'a>,
        }

        impl<'a> AstNode<'a> for $name<'a> {
            fn can_cast(kind: PolicySyntax) -> bool {
                kind == $kind
            }

            fn cast(node: PolicyNode<'a>) -> Option<Self> {
                Self::can_cast(node.value()).then_some(Self { node })
            }

            fn syntax(&self) -> &PolicyNode<'a> {
                &self.node
            }
        }
    };
}

macro_rules! ast_token {
    ($(#[$meta:meta])* $name:ident, $kind:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            node: PolicyNode<'a>,
        }

        impl<'a> AstToken<'a> for $name<'a> {
            fn can_cast(kind: PolicySyntax) -> bool {
                kind == $kind
            }

            fn cast(node: PolicyNode<'a>) -> Option<Self> {
                Self::can_cast(node.value()).then_some(Self { node })
            }

            fn syntax(&self) -> &PolicyNode<'a> {
                &self.node
            }
        }
    };
}

pub(crate) use {ast_node, ast_token};

pub trait AstNode<'a>: Sized {
    fn can_cast(kind: PolicySyntax) -> bool;
    fn cast(node: PolicyNode<'a>) -> Option<Self>;
    fn syntax(&self) -> &PolicyNode<'a>;

    #[inline]
    fn range(&self) -> Range<usize> {
        self.syntax().range()
    }
}

pub trait AstToken<'a>: Sized {
    fn can_cast(kind: PolicySyntax) -> bool;
    fn cast(node: PolicyNode<'a>) -> Option<Self>;
    fn syntax(&self) -> &PolicyNode<'a>;

    #[inline]
    fn range(&self) -> Range<usize> {
        self.syntax().range()
    }

    #[inline]
    fn text<'s>(&self, source: &'s str) -> &'s str {
        &source[self.syntax().range()]
    }
}
