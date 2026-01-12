use super::{AstToken, SchemaNode};
use crate::schema::SchemaSyntax;

#[derive(Debug, Clone, Copy)]
pub struct StringToken<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstToken<'a> for StringToken<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::String
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntegerToken<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstToken<'a> for IntegerToken<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::Integer
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdentifierToken<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstToken<'a> for IdentifierToken<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::Identifier
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}
