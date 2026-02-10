use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::schema::TypeExpression;

#[derive(Clone, Copy, Debug)]
pub struct ContextType<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ContextType<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::ContextType => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> ContextType<'a> {
    /// Returns the type definition.
    #[must_use]
    pub fn definition(&self) -> Option<TypeExpression<'a>> {
        self.node.children().find_map(TypeExpression::cast)
    }

    /// Returns the `context` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::ContextKeyword)
    }

    /// Returns the colon token.
    #[must_use]
    pub fn colon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Colon)
    }
}
