use duramen_syntax::{Group, Node};

use crate::CstNode;
use crate::common::Name;
use crate::schema::{EntityType, EnumType, RecordType, SetType};

#[derive(Clone, Copy, Debug)]
pub enum TypeExpression<'a> {
    Set(SetType<'a>),
    Record(RecordType<'a>),
    Entity(EntityType<'a>),
    Enum(EnumType<'a>),
    Reference(Name<'a>),
}

impl<'a> CstNode<'a> for TypeExpression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group() {
            Some(Group::TypeExpression) => node.children().find_map(Self::cast),
            Some(Group::SetType) => SetType::cast(node).map(Self::Set),
            Some(Group::RecordType) => RecordType::cast(node).map(Self::Record),
            Some(Group::EntityType) => EntityType::cast(node).map(Self::Entity),
            Some(Group::EnumType) => EnumType::cast(node).map(Self::Enum),
            Some(Group::Name) => Name::cast(node).map(Self::Reference),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        match self {
            Self::Set(node) => node.syntax(),
            Self::Record(node) => node.syntax(),
            Self::Entity(node) => node.syntax(),
            Self::Enum(node) => node.syntax(),
            Self::Reference(node) => node.syntax(),
        }
    }
}
