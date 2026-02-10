use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct LikeExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for LikeExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::LikeExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> LikeExpression<'a> {
    /// Returns the expression being tested.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the pattern expression.
    #[must_use]
    pub fn pattern(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }

    /// Returns the `like` keyword token.
    #[must_use]
    pub fn like_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::LikeKeyword)
    }
}
