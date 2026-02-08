use crate::{CstNode, EntityType, EnumType, Name, Node, RecordType, SetType, Syntax};

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
        match node.kind() {
            Syntax::TypeExpression => node.children().find_map(Self::cast),
            Syntax::SetType => SetType::cast(node).map(Self::Set),
            Syntax::RecordType => RecordType::cast(node).map(Self::Record),
            Syntax::EntityType => EntityType::cast(node).map(Self::Entity),
            Syntax::EnumType => EnumType::cast(node).map(Self::Enum),
            Syntax::Name => Name::cast(node).map(Self::Reference),
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
