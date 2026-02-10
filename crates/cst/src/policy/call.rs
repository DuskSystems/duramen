use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::policy::Arguments;

#[derive(Clone, Copy, Debug)]
pub struct Call<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Call<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Call => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Call<'a> {
    /// Returns the `.` dot token.
    #[must_use]
    pub fn dot(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Dot)
    }

    /// Returns the method name token.
    #[must_use]
    pub fn name(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| child.kind().is_identifier())
    }

    /// Returns the argument list.
    #[must_use]
    pub fn arguments(&self) -> Option<Arguments<'a>> {
        self.node.children().find_map(Arguments::cast)
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
