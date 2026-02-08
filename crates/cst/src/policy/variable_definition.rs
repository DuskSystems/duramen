use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct VariableDefinition<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for VariableDefinition<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::VariableDefinition => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}
