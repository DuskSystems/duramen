use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression, MultiplicationOperator};

#[derive(Clone, Copy, Debug)]
pub struct ProductExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ProductExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::ProductExpression => Some(Self { node }),
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

    /// Returns the multiplication operator.
    #[must_use]
    pub fn operator(&self) -> Option<MultiplicationOperator> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::Wildcard => Some(MultiplicationOperator::Multiply),
            Syntax::Divide => Some(MultiplicationOperator::Divide),
            Syntax::Modulo => Some(MultiplicationOperator::Modulo),
            _ => None,
        })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| {
            matches!(
                child.kind(),
                Syntax::Wildcard | Syntax::Divide | Syntax::Modulo
            )
        })
    }
}
