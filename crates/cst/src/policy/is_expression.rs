use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::common::Name;
use crate::policy::Expression;

#[derive(Clone, Copy, Debug)]
pub struct IsExpression<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for IsExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::IsExpression => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> IsExpression<'a> {
    /// Returns the expression being tested.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    /// Returns the entity kind name.
    #[must_use]
    pub fn kind(&self) -> Option<Name<'a>> {
        self.node.after(Syntax::IsKeyword).find_map(Name::cast)
    }

    /// Returns the `in` target expression.
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node
            .after(Syntax::InKeyword)
            .find_map(Expression::cast)
    }

    /// Returns the `is` keyword token.
    #[must_use]
    pub fn is_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::IsKeyword)
    }

    /// Returns the `in` keyword token.
    #[must_use]
    pub fn in_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::InKeyword)
    }
}
