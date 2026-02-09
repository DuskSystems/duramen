use duramen_syntax::{Node, Syntax};

use crate::{CstNode, TypeExpression};

#[derive(Clone, Copy, Debug)]
pub struct EntityTags<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EntityTags<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::EntityTags => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> EntityTags<'a> {
    /// Returns the type definition.
    #[must_use]
    pub fn definition(&self) -> Option<TypeExpression<'a>> {
        self.node.children().find_map(TypeExpression::cast)
    }

    /// Returns the `tags` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::TagsKeyword)
    }
}
