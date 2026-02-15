use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct HasExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for HasExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::HasExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> HasExpression<'a> {
    /// Returns the expression being tested.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the attribute expression.
    #[must_use]
    pub fn attribute(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }

    /// Returns the `has` keyword token.
    #[must_use]
    pub fn has_token(&self) -> Option<Node<'a>> {
        self.node.child(Token::HasKeyword)
    }
}
