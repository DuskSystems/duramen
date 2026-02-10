use duramen_syntax::{Node, Syntax};

use crate::CstNode;
use crate::schema::{ContextType, PrincipalTypes, ResourceTypes};

#[derive(Clone, Copy, Debug)]
pub struct AppliesTo<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for AppliesTo<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::AppliesToClause => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> AppliesTo<'a> {
    /// Returns the principal types clause.
    #[must_use]
    pub fn principals(&self) -> Option<PrincipalTypes<'a>> {
        self.node.children().find_map(PrincipalTypes::cast)
    }

    /// Returns the resource types clause.
    #[must_use]
    pub fn resources(&self) -> Option<ResourceTypes<'a>> {
        self.node.children().find_map(ResourceTypes::cast)
    }

    /// Returns the context type.
    #[must_use]
    pub fn context(&self) -> Option<ContextType<'a>> {
        self.node.children().find_map(ContextType::cast)
    }

    /// Returns the `appliesTo` keyword token.
    #[must_use]
    pub fn keyword(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::AppliesToKeyword)
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
