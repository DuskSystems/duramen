use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::common::Annotation;
use crate::schema::TypeExpression;

#[derive(Clone, Copy, Debug)]
pub struct AttributeDeclaration<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for AttributeDeclaration<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::AttributeDeclaration => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> AttributeDeclaration<'a> {
    /// Returns an iterator over annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the attribute name token (string or identifier).
    #[must_use]
    pub fn name(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| matches!(child.kind(), Syntax::String) || child.kind().is_identifier())
    }

    /// Returns whether the attribute is optional (has a `?` token).
    #[must_use]
    pub fn is_optional(&self) -> bool {
        self.question_mark().is_some()
    }

    /// Returns the `?` token.
    #[must_use]
    pub fn question_mark(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::QuestionMark)
    }

    /// Returns the attribute type definition.
    #[must_use]
    pub fn definition(&self) -> Option<TypeExpression<'a>> {
        self.node.children().find_map(TypeExpression::cast)
    }

    /// Returns the colon token.
    #[must_use]
    pub fn colon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Colon)
    }
}
