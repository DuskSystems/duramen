use duramen_syntax::{Node, Syntax};

use crate::{CstNode, EntityReference, Name};

#[derive(Clone, Copy, Debug)]
pub struct ActionParents<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ActionParents<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::ActionParents => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> ActionParents<'a> {
    /// Returns an iterator over name children.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> {
        self.node.children().filter_map(Name::cast)
    }

    /// Returns an iterator over entity reference children.
    pub fn entity_references(&self) -> impl Iterator<Item = EntityReference<'a>> {
        self.node.children().filter_map(EntityReference::cast)
    }

    /// Returns the `in` keyword token.
    #[must_use]
    pub fn in_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::InKeyword)
    }

    /// Returns the opening bracket token.
    #[must_use]
    pub fn open_bracket(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::OpenBracket)
    }

    /// Returns the closing bracket token.
    #[must_use]
    pub fn close_bracket(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::CloseBracket)
    }
}
