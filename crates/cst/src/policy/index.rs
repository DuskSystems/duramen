use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression};

#[derive(Clone, Copy, Debug)]
pub struct Index<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Index<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Index => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Index<'a> {
    /// Returns the index expression.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the opening bracket token.
    #[must_use]
    pub fn open_bracket(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::OpenBracket)
    }

    /// Returns the closing bracket token.
    #[must_use]
    pub fn close_bracket(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::CloseBracket)
    }
}
