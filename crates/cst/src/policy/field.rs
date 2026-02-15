use duramen_syntax::{Group, Node, Token};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Field<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Field<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Field => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Field<'a> {
    /// Returns the `.` dot token.
    #[must_use]
    pub fn dot(&self) -> Option<Node<'a>> {
        self.node.child(Token::Dot)
    }

    /// Returns the field name token.
    #[must_use]
    pub fn name(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| child.kind().is_identifier())
    }
}
