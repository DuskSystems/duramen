use duramen_syntax::{Group, Node};

use crate::CstNode;
use crate::policy::{Call, Field, Index};

#[derive(Clone, Copy, Debug)]
pub enum MemberAccess<'a> {
    Field(Field<'a>),
    Call(Call<'a>),
    Index(Index<'a>),
}

impl<'a> CstNode<'a> for MemberAccess<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Field => Field::cast(node).map(Self::Field),
            Group::Call => Call::cast(node).map(Self::Call),
            Group::Index => Index::cast(node).map(Self::Index),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        match self {
            Self::Field(node) => node.syntax(),
            Self::Call(node) => node.syntax(),
            Self::Index(node) => node.syntax(),
        }
    }
}
