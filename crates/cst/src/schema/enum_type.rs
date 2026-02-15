use duramen_syntax::{Group, Node, Token};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct EnumType<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EnumType<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::EnumType => Some(Self { node }),
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
            .filter(|child| child.kind() == Token::String)
    }

    /// Returns the `enum` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Token::EnumKeyword)
    }

    /// Returns the opening bracket token.
    #[must_use]
    pub fn open_bracket(&self) -> Option<Node<'a>> {
        self.node.child(Token::OpenBracket)
    }

    /// Returns the closing bracket token.
    #[must_use]
    pub fn close_bracket(&self) -> Option<Node<'a>> {
        self.node.child(Token::CloseBracket)
    }
}
