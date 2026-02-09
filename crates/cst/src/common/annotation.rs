use duramen_syntax::{Node, Syntax};

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Annotation<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Annotation<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Annotation => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Annotation<'a> {
    /// Returns the `@` token.
    #[must_use]
    pub fn at_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::At)
    }

    /// Returns the annotation name token (identifier).
    #[must_use]
    pub fn name(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| child.kind().is_identifier())
    }

    /// Returns the annotation value string token.
    #[must_use]
    pub fn value(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::String)
    }

    /// Returns the opening parenthesis token.
    #[must_use]
    pub fn open_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::OpenParenthesis)
    }

    /// Returns the closing parenthesis token.
    #[must_use]
    pub fn close_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::CloseParenthesis)
    }
}
