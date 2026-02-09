use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Name};

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

impl<'a> Types<'a> {
    /// Returns an iterator over name children.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> {
        self.node.children().filter_map(Name::cast)
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
