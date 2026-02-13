use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct AndExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for AndExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::AndExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> AndExpression<'a> {
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

    /// Returns the `&&` operator token, or `&` if used as a fallback.
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node
            .child(Syntax::And)
            .or_else(|| self.node.child(Syntax::Ampersand))
    }
}
