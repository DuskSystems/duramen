use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct EnumType<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EnumType<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::EnumType => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> EnumType<'a> {
    /// Returns an iterator over variant string tokens.
    pub fn variants(&self) -> impl Iterator<Item = Node<'a>> {
        self.node
            .children()
            .filter(|child| child.kind() == Syntax::String)
    }

    /// Returns the `enum` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::EnumKeyword)
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
