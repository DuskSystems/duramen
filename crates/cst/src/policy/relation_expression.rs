use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression, RelationOperator};

#[derive(Clone, Copy, Debug)]
pub struct RelationExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for RelationExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::RelationExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> RelationExpression<'a> {
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

    /// Returns the relation operator.
    #[must_use]
    pub fn operator(&self) -> Option<RelationOperator> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::Less => Some(RelationOperator::Less),
            Syntax::LessEqual => Some(RelationOperator::LessEqual),
            Syntax::Greater => Some(RelationOperator::Greater),
            Syntax::GreaterEqual => Some(RelationOperator::GreaterEqual),
            Syntax::Equal => Some(RelationOperator::Equal),
            Syntax::NotEqual => Some(RelationOperator::NotEqual),
            Syntax::InKeyword => Some(RelationOperator::In),
            _ => None,
        })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| {
            matches!(
                child.kind(),
                Syntax::Less
                    | Syntax::LessEqual
                    | Syntax::Greater
                    | Syntax::GreaterEqual
                    | Syntax::Equal
                    | Syntax::NotEqual
                    | Syntax::InKeyword
                    | Syntax::Assign
            )
        })
    }
}
