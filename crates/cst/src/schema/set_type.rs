use duramen_syntax::{Node, Syntax};

use crate::{CstNode, TypeExpression};

#[derive(Clone, Copy, Debug)]
pub struct SetType<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for SetType<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::SetType => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> SetType<'a> {
    /// Returns the element type expression.
    #[must_use]
    pub fn element(&self) -> Option<TypeExpression<'a>> {
        self.node.children().find_map(TypeExpression::cast)
    }

    /// Returns the `Set` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::SetKeyword)
    }

    /// Returns the opening angle bracket token.
    #[must_use]
    pub fn open_angle(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::LessThan)
    }

    /// Returns the closing angle bracket token.
    #[must_use]
    pub fn close_angle(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::GreaterThan)
    }
}
