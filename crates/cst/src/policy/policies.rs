use duramen_syntax::{Group, Node};

use crate::CstNode;
use crate::policy::Policy;

#[derive(Clone, Copy, Debug)]
pub struct Policies<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Policies<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Policies => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Policies<'a> {
    /// Returns an iterator over the policy children.
    pub fn policies(&self) -> impl Iterator<Item = Policy<'a>> {
        self.node.children().filter_map(Policy::cast)
    }
}
