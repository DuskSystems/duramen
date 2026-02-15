use duramen_syntax::{Group, Node, Token};

use crate::CstNode;
use crate::policy::VariableDefinition;

#[derive(Clone, Copy, Debug)]
pub struct Scope<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Scope<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Scope => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Scope<'a> {
    /// Returns an iterator over the variable definition children.
    pub fn variable_definitions(&self) -> impl Iterator<Item = VariableDefinition<'a>> + use<'a> {
        self.node.children().filter_map(VariableDefinition::cast)
    }

    /// Returns the opening parenthesis token.
    #[must_use]
    pub fn open_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Token::OpenParenthesis)
    }

    /// Returns the closing parenthesis token.
    #[must_use]
    pub fn close_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Token::CloseParenthesis)
    }
}
