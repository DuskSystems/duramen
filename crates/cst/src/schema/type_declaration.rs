use duramen_syntax::{Node, Syntax};

use crate::{Annotation, CstNode, Name, TypeExpression};

#[derive(Clone, Copy, Debug)]
pub struct TypeDeclaration<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for TypeDeclaration<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::TypeDeclaration => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> TypeDeclaration<'a> {
    /// Returns an iterator over annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the type name.
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns the type definition.
    #[must_use]
    pub fn definition(&self) -> Option<TypeExpression<'a>> {
        self.node.children().find_map(TypeExpression::cast)
    }

    /// Returns the `type` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::TypeKeyword)
    }

    /// Returns the `=` token.
    #[must_use]
    pub fn assign(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Assign)
    }

    /// Returns the semicolon token.
    #[must_use]
    pub fn semicolon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Semicolon)
    }
}
