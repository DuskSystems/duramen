use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::LiteralKind;

#[derive(Clone, Copy, Debug)]
pub struct Literal<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Literal<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Literal => Some(Self { node }),
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
        self.node
            .children()
            .find_map(|child| match child.kind().token()? {
                Token::TrueKeyword | Token::FalseKeyword => Some(LiteralKind::Bool),
                Token::Integer => Some(LiteralKind::Integer),
                Token::String => Some(LiteralKind::String),
                _ => None,
            })
    }

    /// Returns the literal value token.
    #[must_use]
    pub fn token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| child.kind().is_literal())
    }
}
