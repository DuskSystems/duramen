use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::policy::LiteralKind;

#[derive(Clone, Copy, Debug)]
pub struct Literal<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Literal<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Literal => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Literal<'a> {
    /// Returns the literal kind.
    #[must_use]
    pub fn kind(&self) -> Option<LiteralKind> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::TrueKeyword | Syntax::FalseKeyword => Some(LiteralKind::Bool),
            Syntax::Integer => Some(LiteralKind::Integer),
            Syntax::String => Some(LiteralKind::String),
            _ => None,
        })
    }

    /// Returns the literal value token.
    #[must_use]
    pub fn token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| child.kind().is_literal())
    }
}
