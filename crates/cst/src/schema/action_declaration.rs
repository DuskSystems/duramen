use duramen_syntax::{Node, Syntax};

use crate::{ActionAttributes, ActionParents, Annotation, AppliesTo, CstNode, Name};

#[derive(Clone, Copy, Debug)]
pub struct ActionDeclaration<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for ActionDeclaration<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::ActionDeclaration => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> ActionDeclaration<'a> {
    /// Returns an iterator over annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns an iterator over action name children.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> {
        self.node.children().filter_map(Name::cast)
    }

    /// Returns the action parents clause.
    #[must_use]
    pub fn parents(&self) -> Option<ActionParents<'a>> {
        self.node.children().find_map(ActionParents::cast)
    }

    /// Returns the applies-to clause.
    #[must_use]
    pub fn applies_to(&self) -> Option<AppliesTo<'a>> {
        self.node.children().find_map(AppliesTo::cast)
    }

    /// Returns the action attributes block.
    #[must_use]
    pub fn attributes(&self) -> Option<ActionAttributes<'a>> {
        self.node.children().find_map(ActionAttributes::cast)
    }

    /// Returns the `action` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::ActionKeyword)
    }

    /// Returns the semicolon token.
    #[must_use]
    pub fn semicolon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Semicolon)
    }
}
