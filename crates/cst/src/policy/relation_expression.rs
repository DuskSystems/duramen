use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression, RelationalOperator};

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

    /// Returns the relational operator.
    #[must_use]
    pub fn operator(&self) -> Option<RelationalOperator> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::LessThan => Some(RelationalOperator::Less),
            Syntax::LessThanEquals => Some(RelationalOperator::LessEqual),
            Syntax::GreaterThan => Some(RelationalOperator::Greater),
            Syntax::GreaterThanEquals => Some(RelationalOperator::GreaterEqual),
            Syntax::Equal => Some(RelationalOperator::Equal),
            Syntax::NotEqual => Some(RelationalOperator::NotEqual),
            Syntax::InKeyword => Some(RelationalOperator::In),
            _ => None,
        })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| {
            matches!(
                child.kind(),
                Syntax::LessThan
                    | Syntax::LessThanEquals
                    | Syntax::GreaterThan
                    | Syntax::GreaterThanEquals
                    | Syntax::Equal
                    | Syntax::NotEqual
                    | Syntax::InKeyword
                    | Syntax::Assign
            )
        })
    }
}
