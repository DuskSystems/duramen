use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::{Expression, UnaryOperator};

#[derive(Clone, Copy, Debug)]
pub struct UnaryExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for UnaryExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::UnaryExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> UnaryExpression<'a> {
    /// Returns the unary operator from the first operator token.
    #[must_use]
    pub fn operator(&self) -> Option<UnaryOperator> {
        self.node
            .children()
            .find_map(|child| match child.kind().token()? {
                Token::Not => Some(UnaryOperator::Not),
                Token::Subtract => Some(UnaryOperator::Negate),
                _ => None,
            })
    }

    /// Returns an iterator over the operator tokens.
    pub fn operator_tokens(&self) -> impl Iterator<Item = Node<'a>> {
        self.node
            .children()
            .filter(|child| matches!(child.kind().token(), Some(Token::Not | Token::Subtract)))
    }

    /// Returns the operand expression.
    #[must_use]
    pub fn operand(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}
