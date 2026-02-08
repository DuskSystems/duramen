use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Types<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Types<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Types => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}
