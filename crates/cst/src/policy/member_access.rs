use duramen_syntax::{Node, Syntax};

use crate::{CstNode, FieldAccess, IndexAccess, MethodCall};

#[derive(Clone, Copy, Debug)]
pub enum MemberAccess<'a> {
    Field(FieldAccess<'a>),
    Call(MethodCall<'a>),
    Index(IndexAccess<'a>),
}

impl<'a> CstNode<'a> for MemberAccess<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Field => FieldAccess::cast(node).map(Self::Field),
            Syntax::Call => MethodCall::cast(node).map(Self::Call),
            Syntax::Index => IndexAccess::cast(node).map(Self::Index),
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
