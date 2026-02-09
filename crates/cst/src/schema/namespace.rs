use duramen_syntax::{Node, Syntax};

use crate::{ActionDeclaration, Annotation, CstNode, EntityDeclaration, Name, TypeDeclaration};

#[derive(Clone, Copy, Debug)]
pub struct Namespace<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Namespace<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::NamespaceDeclaration => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Namespace<'a> {
    /// Returns an iterator over annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the namespace name.
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns an iterator over entity declaration children.
    pub fn entity_declarations(&self) -> impl Iterator<Item = EntityDeclaration<'a>> {
        self.node.children().filter_map(EntityDeclaration::cast)
    }

    /// Returns an iterator over action declaration children.
    pub fn action_declarations(&self) -> impl Iterator<Item = ActionDeclaration<'a>> {
        self.node.children().filter_map(ActionDeclaration::cast)
    }

    /// Returns an iterator over type declaration children.
    pub fn type_declarations(&self) -> impl Iterator<Item = TypeDeclaration<'a>> {
        self.node.children().filter_map(TypeDeclaration::cast)
    }

    /// Returns an iterator over nested namespace children.
    pub fn namespaces(&self) -> impl Iterator<Item = Namespace<'a>> {
        self.node.children().filter_map(Namespace::cast)
    }

    /// Returns the `namespace` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::NamespaceKeyword)
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
