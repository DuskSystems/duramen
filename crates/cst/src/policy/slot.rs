use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Slot<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Slot<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Slot => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Slot<'a> {
    /// Returns the `?` token.
    #[must_use]
    pub fn question_mark(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::QuestionMark)
    }

    /// Returns the identifier token after `?`.
    #[must_use]
    pub fn name(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| child.kind().is_identifier())
    }
}
