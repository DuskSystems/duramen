use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::{Expression, SumOperator};

#[derive(Clone, Copy, Debug)]
pub struct SumExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for SumExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::SumExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> SumExpression<'a> {
    /// Returns the left expression.
    #[must_use]
    pub fn left(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the right expression.
    #[must_use]
    pub fn right(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }

    /// Returns the sum operator.
    #[must_use]
    pub fn operator(&self) -> Option<SumOperator> {
        self.node
            .children()
            .find_map(|child| match child.kind().token()? {
                Token::Add => Some(SumOperator::Add),
                Token::Subtract => Some(SumOperator::Subtract),
                _ => None,
            })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| matches!(child.kind().token(), Some(Token::Add | Token::Subtract)))
    }
}
