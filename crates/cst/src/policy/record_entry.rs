use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct RecordEntry<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for RecordEntry<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::RecordEntry => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> RecordEntry<'a> {
    /// Returns the key token (string or identifier).
    #[must_use]
    pub fn key(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| child.kind() == Token::String || child.kind().is_identifier())
    }

    /// Returns the value expression.
    #[must_use]
    pub fn value(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the colon token.
    #[must_use]
    pub fn colon(&self) -> Option<Node<'a>> {
        self.node.child(Token::Colon)
    }
}
