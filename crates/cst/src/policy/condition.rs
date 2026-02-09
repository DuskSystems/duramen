use duramen_syntax::{Node, Syntax};

use crate::{ConditionKind, CstNode, Expression};

#[derive(Clone, Copy, Debug)]
pub struct Condition<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Condition<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Condition => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Condition<'a> {
    /// Returns the condition kind (`when` or `unless`).
    #[must_use]
    pub fn kind(&self) -> Option<ConditionKind> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::WhenKeyword => Some(ConditionKind::When),
            Syntax::UnlessKeyword => Some(ConditionKind::Unless),
            _ => None,
        })
    }

    /// Returns the condition keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| matches!(child.kind(), Syntax::WhenKeyword | Syntax::UnlessKeyword))
    }

    /// Returns the body expression.
    #[must_use]
    pub fn body(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the opening brace token.
    #[must_use]
    pub fn open_brace(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::OpenBrace)
    }

    /// Returns the closing brace token.
    #[must_use]
    pub fn close_brace(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::CloseBrace)
    }
}
