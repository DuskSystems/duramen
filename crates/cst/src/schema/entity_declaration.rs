use duramen_syntax::{Node, Syntax};

use crate::{Annotation, CstNode, EntityAttributes, EntityParents, EntityTags, EnumType, Name};

#[derive(Clone, Copy, Debug)]
pub struct EntityDeclaration<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for EntityDeclaration<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::EntityDeclaration => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> EntityDeclaration<'a> {
    /// Returns an iterator over annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns an iterator over entity name children.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> {
        self.node.children().filter_map(Name::cast)
    }

    /// Returns the entity parents clause.
    #[must_use]
    pub fn parents(&self) -> Option<EntityParents<'a>> {
        self.node.children().find_map(EntityParents::cast)
    }

    /// Returns the entity attributes block.
    #[must_use]
    pub fn attributes(&self) -> Option<EntityAttributes<'a>> {
        self.node.children().find_map(EntityAttributes::cast)
    }

    /// Returns the entity tags clause.
    #[must_use]
    pub fn tags(&self) -> Option<EntityTags<'a>> {
        self.node.children().find_map(EntityTags::cast)
    }

    /// Returns the enum definition.
    #[must_use]
    pub fn enumeration(&self) -> Option<EnumType<'a>> {
        self.node.children().find_map(EnumType::cast)
    }

    /// Returns the `entity` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::EntityKeyword)
    }

    /// Returns the semicolon token.
    #[must_use]
    pub fn semicolon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Semicolon)
    }
}
