use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct IfExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for IfExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::IfExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> IfExpression<'a> {
    /// Returns the test expression.
    #[must_use]
    pub fn test(&self) -> Option<Expression<'a>> {
        self.node.after(Token::IfKeyword).find_map(Expression::cast)
    }

    /// Returns the consequent expression.
    #[must_use]
    pub fn consequent(&self) -> Option<Expression<'a>> {
        self.node
            .after(Token::ThenKeyword)
            .find_map(Expression::cast)
    }

    /// Returns the alternate expression.
    #[must_use]
    pub fn alternate(&self) -> Option<Expression<'a>> {
        self.node
            .after(Token::ElseKeyword)
            .find_map(Expression::cast)
    }

    /// Returns the `if` keyword token.
    #[must_use]
    pub fn if_token(&self) -> Option<Node<'a>> {
        self.node.child(Token::IfKeyword)
    }

    /// Returns the `then` keyword token.
    #[must_use]
    pub fn then_token(&self) -> Option<Node<'a>> {
        self.node.child(Token::ThenKeyword)
    }

    /// Returns the `else` keyword token.
    #[must_use]
    pub fn else_token(&self) -> Option<Node<'a>> {
        self.node.child(Token::ElseKeyword)
    }
}
