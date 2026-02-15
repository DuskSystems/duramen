use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::common::Name;
use crate::policy::RecordEntry;

#[derive(Clone, Copy, Debug)]
pub struct EntityReference<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EntityReference<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::EntityReference => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> EntityReference<'a> {
    /// Returns the entity kind name.
    #[must_use]
    pub fn kind(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns the entity identifier string token.
    #[must_use]
    pub fn id(&self) -> Option<Node<'a>> {
        self.node.child(Token::String)
    }

    /// Returns an iterator over the record entries (for record-style references).
    pub fn entries(&self) -> impl Iterator<Item = RecordEntry<'a>> {
        self.node.children().filter_map(RecordEntry::cast)
    }

    /// Returns the `::` path separator token.
    #[must_use]
    pub fn path_separator(&self) -> Option<Node<'a>> {
        self.node.child(Token::PathSeparator)
    }
}
