use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::RecordEntry;

#[derive(Clone, Copy, Debug)]
pub struct Record<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Record<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Record => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Record<'a> {
    /// Returns an iterator over the record entries.
    pub fn entries(&self) -> impl Iterator<Item = RecordEntry<'a>> {
        self.node.children().filter_map(RecordEntry::cast)
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
