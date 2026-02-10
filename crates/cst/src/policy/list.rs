use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::policy::Arguments;

#[derive(Clone, Copy, Debug)]
pub struct List<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for List<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::List => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> List<'a> {
    /// Returns the argument list.
    #[must_use]
    pub fn arguments(&self) -> Option<Arguments<'a>> {
        self.node.children().find_map(Arguments::cast)
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
