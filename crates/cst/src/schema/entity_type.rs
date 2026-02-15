use duramen_syntax::{Group, Node, Token};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct EntityType<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EntityType<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::EntityType => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> EntityType<'a> {
    /// Returns the `entity` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Token::EntityKeyword)
    }
}
