use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Name<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Name<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Name => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Name<'a> {
    /// Returns the identifier segment tokens.
    pub fn segments(&self) -> impl Iterator<Item = Node<'a>> {
        self.node
            .children()
            .filter(|child| child.kind().is_identifier())
    }

    /// Returns the path separator (`::`) tokens.
    pub fn separators(&self) -> impl Iterator<Item = Node<'a>> {
        self.node
            .children()
            .filter(|child| child.kind() == Syntax::PathSeparator)
    }

    /// Returns the text of the last identifier segment.
    #[must_use]
    pub fn basename(&self, source: &'a str) -> Option<&'a str> {
        self.segments().last().map(|node| &source[node.range()])
    }

    /// Returns whether the name has more than one segment.
    #[must_use]
    pub fn is_qualified(&self) -> bool {
        let mut segments = self.segments();
        segments.next().is_some() && segments.next().is_some()
    }
}
