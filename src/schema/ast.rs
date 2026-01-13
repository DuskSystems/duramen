use syntree::{FlavorDefault, Node, Span};

use super::SchemaSyntax;

mod nodes;
pub use nodes::*;

mod tokens;
pub use tokens::*;

mod types;
pub use types::*;

pub type SchemaNode<'a> = Node<'a, SchemaSyntax, FlavorDefault>;

macro_rules! ast_node {
    ($(#[$meta:meta])* $name:ident, $kind:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            node: SchemaNode<'a>,
        }

        impl<'a> AstNode<'a> for $name<'a> {
            fn can_cast(kind: SchemaSyntax) -> bool {
                kind == $kind
            }

            fn cast(node: SchemaNode<'a>) -> Option<Self> {
                Self::can_cast(node.value()).then_some(Self { node })
            }

            fn syntax(&self) -> &SchemaNode<'a> {
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
            node: SchemaNode<'a>,
        }

        impl<'a> AstToken<'a> for $name<'a> {
            fn can_cast(kind: SchemaSyntax) -> bool {
                kind == $kind
            }

            fn cast(node: SchemaNode<'a>) -> Option<Self> {
                Self::can_cast(node.value()).then_some(Self { node })
            }

            fn syntax(&self) -> &SchemaNode<'a> {
                &self.node
            }
        }
    };
}

pub(crate) use {ast_node, ast_token};

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
