use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::{Expression, RelationOperator};

#[derive(Clone, Copy, Debug)]
pub struct RelationExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for RelationExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::RelationExpression => Some(Self { node }),
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
        self.node
            .children()
            .find_map(|child| match child.kind().token()? {
                Token::Less => Some(RelationOperator::Less),
                Token::LessEqual => Some(RelationOperator::LessEqual),
                Token::Greater => Some(RelationOperator::Greater),
                Token::GreaterEqual => Some(RelationOperator::GreaterEqual),
                Token::Equal => Some(RelationOperator::Equal),
                Token::NotEqual => Some(RelationOperator::NotEqual),
                Token::InKeyword => Some(RelationOperator::In),
                _ => None,
            })
    }

    /// Returns the operator token.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| {
            matches!(
                child.kind().token(),
                Some(
                    Token::Less
                        | Token::LessEqual
                        | Token::Greater
                        | Token::GreaterEqual
                        | Token::Equal
                        | Token::NotEqual
                        | Token::InKeyword
                        | Token::Assign
                )
            )
        })
    }
}
