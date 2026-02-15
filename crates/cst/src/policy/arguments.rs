use duramen_syntax::{Group, Node};

use crate::CstNode;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct Arguments<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Arguments<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Arguments => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Arguments<'a> {
    /// Returns an iterator over the argument expressions.
    pub fn expressions(&self) -> impl Iterator<Item = Expression<'a>> {
        self.node.children().filter_map(Expression::cast)
    }
}
