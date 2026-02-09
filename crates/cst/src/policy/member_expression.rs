use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression, MemberAccess};

#[derive(Clone, Copy, Debug)]
pub struct MemberExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for MemberExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::MemberExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> MemberExpression<'a> {
    /// Returns the base expression.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns an iterator over the member access children.
    pub fn accesses(&self) -> impl Iterator<Item = MemberAccess<'a>> {
        self.node.children().filter_map(MemberAccess::cast)
    }
}
