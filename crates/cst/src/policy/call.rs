use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Call<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Call<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Call => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}
