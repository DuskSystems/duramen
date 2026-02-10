use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::common::Name;
use crate::schema::Types;

#[derive(Clone, Copy, Debug)]
pub struct EntityParents<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EntityParents<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::EntityParents => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> EntityParents<'a> {
    /// Returns the bracketed types list.
    #[must_use]
    pub fn types(&self) -> Option<Types<'a>> {
        self.node.children().find_map(Types::cast)
    }

    /// Returns the single unbracketed parent type name.
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns the `in` keyword token.
    #[must_use]
    pub fn in_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::InKeyword)
    }
}
