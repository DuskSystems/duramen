use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::{Expression, ProductOperator};

#[derive(Clone, Copy, Debug)]
pub struct ProductExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ProductExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::ProductExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> ProductExpression<'a> {
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

    /// Returns the product operator.
    #[must_use]
    pub fn operator(&self) -> Option<ProductOperator> {
        self.node
            .children()
            .find_map(|child| match child.kind().token()? {
                Token::Multiply => Some(ProductOperator::Multiply),
                Token::Divide => Some(ProductOperator::Divide),
                Token::Modulo => Some(ProductOperator::Modulo),
                _ => None,
            })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| {
            matches!(
                child.kind().token(),
                Some(Token::Multiply | Token::Divide | Token::Modulo)
            )
        })
    }
}
