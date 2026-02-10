use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::common::Name;
use crate::schema::Types;

#[derive(Clone, Copy, Debug)]
pub struct PrincipalTypes<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for PrincipalTypes<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::PrincipalTypes => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> PrincipalTypes<'a> {
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

    /// Returns the `principal` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::PrincipalKeyword)
    }

    /// Returns the colon token.
    #[must_use]
    pub fn colon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Colon)
    }
}
