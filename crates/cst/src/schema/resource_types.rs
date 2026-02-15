use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::common::Name;
use crate::schema::Types;

#[derive(Clone, Copy, Debug)]
pub struct ResourceTypes<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ResourceTypes<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::ResourceTypes => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> ResourceTypes<'a> {
    /// Returns the bracketed types list.
    #[must_use]
    pub fn types(&self) -> Option<Types<'a>> {
        self.node.children().find_map(Types::cast)
    }

    /// Returns the single unbracketed type name.
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns the `resource` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Token::ResourceKeyword)
    }

    /// Returns the colon token.
    #[must_use]
    pub fn colon(&self) -> Option<Node<'a>> {
        self.node.child(Token::Colon)
    }
}
