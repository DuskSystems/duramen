use duramen_syntax::{Node, Syntax};

use crate::{AdditionOperator, CstNode, Expression};

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

    /// Returns the addition operator.
    #[must_use]
    pub fn operator(&self) -> Option<AdditionOperator> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::Plus => Some(AdditionOperator::Add),
            Syntax::Minus => Some(AdditionOperator::Subtract),
            _ => None,
        })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| matches!(child.kind(), Syntax::Plus | Syntax::Minus))
    }
}
