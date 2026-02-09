use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression, SumOperator};

#[derive(Clone, Copy, Debug)]
pub struct SumExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for SumExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::SumExpression => Some(Self { node }),
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
        self.node.children().find_map(|child| match child.kind() {
            Syntax::Add => Some(SumOperator::Add),
            Syntax::Subtract => Some(SumOperator::Subtract),
            _ => None,
        })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| matches!(child.kind(), Syntax::Add | Syntax::Subtract))
    }
}
