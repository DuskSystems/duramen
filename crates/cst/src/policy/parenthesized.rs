use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct Parenthesized<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Parenthesized<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Parenthesized => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Parenthesized<'a> {
    /// Returns the inner expression.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the opening parenthesis token.
    #[must_use]
    pub fn open_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Token::OpenParenthesis)
    }

    /// Returns the closing parenthesis token.
    #[must_use]
    pub fn close_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Token::CloseParenthesis)
    }
}
