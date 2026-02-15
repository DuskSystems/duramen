use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::schema::AttributeDeclaration;

#[derive(Clone, Copy, Debug)]
pub struct ActionAttributes<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ActionAttributes<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::ActionAttributes => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> ActionAttributes<'a> {
    /// Returns an iterator over attribute declaration children.
    pub fn attributes(&self) -> impl Iterator<Item = AttributeDeclaration<'a>> {
        self.node.children().filter_map(AttributeDeclaration::cast)
    }

    /// Returns the `attributes` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Token::AttributesKeyword)
    }

    /// Returns the opening brace token.
    #[must_use]
    pub fn open_brace(&self) -> Option<Node<'a>> {
        self.node.child(Token::OpenBrace)
    }

    /// Returns the closing brace token.
    #[must_use]
    pub fn close_brace(&self) -> Option<Node<'a>> {
        self.node.child(Token::CloseBrace)
    }
}
