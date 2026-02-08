use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Arguments<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Arguments<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Arguments => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}
