use duramen_syntax::{Node, Syntax};

use crate::{
    AndExpression, CstNode, EntityReference, HasExpression, IfExpression, IsExpression,
    LikeExpression, List, Literal, MemberExpression, Name, OrExpression, Parenthesized,
    ProductExpression, Record, RelationExpression, Slot, SumExpression, UnaryExpression,
};

#[derive(Clone, Copy, Debug)]
pub enum Expression<'a> {
    If(IfExpression<'a>),
    Or(OrExpression<'a>),
    And(AndExpression<'a>),
    Relation(RelationExpression<'a>),
    Sum(SumExpression<'a>),
    Product(ProductExpression<'a>),
    Has(HasExpression<'a>),
    Like(LikeExpression<'a>),
    Is(IsExpression<'a>),
    Unary(UnaryExpression<'a>),
    Member(MemberExpression<'a>),
    Literal(Literal<'a>),
    EntityReference(EntityReference<'a>),
    Slot(Slot<'a>),
    Parenthesized(Parenthesized<'a>),
    List(List<'a>),
    Record(Record<'a>),
    Name(Name<'a>),
}

impl<'a> CstNode<'a> for Expression<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::IfExpression => IfExpression::cast(node).map(Self::If),
            Syntax::OrExpression => OrExpression::cast(node).map(Self::Or),
            Syntax::AndExpression => AndExpression::cast(node).map(Self::And),
            Syntax::RelationExpression => RelationExpression::cast(node).map(Self::Relation),
            Syntax::SumExpression => SumExpression::cast(node).map(Self::Sum),
            Syntax::ProductExpression => ProductExpression::cast(node).map(Self::Product),
            Syntax::HasExpression => HasExpression::cast(node).map(Self::Has),
            Syntax::LikeExpression => LikeExpression::cast(node).map(Self::Like),
            Syntax::IsExpression => IsExpression::cast(node).map(Self::Is),
            Syntax::UnaryExpression => UnaryExpression::cast(node).map(Self::Unary),
            Syntax::MemberExpression => MemberExpression::cast(node).map(Self::Member),
            Syntax::Literal => Literal::cast(node).map(Self::Literal),
            Syntax::EntityReference => EntityReference::cast(node).map(Self::EntityReference),
            Syntax::Slot => Slot::cast(node).map(Self::Slot),
            Syntax::Parenthesized => Parenthesized::cast(node).map(Self::Parenthesized),
            Syntax::List => List::cast(node).map(Self::List),
            Syntax::Record => Record::cast(node).map(Self::Record),
            Syntax::Name => Name::cast(node).map(Self::Name),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        match self {
            Self::If(node) => node.syntax(),
            Self::Or(node) => node.syntax(),
            Self::And(node) => node.syntax(),
            Self::Relation(node) => node.syntax(),
            Self::Sum(node) => node.syntax(),
            Self::Product(node) => node.syntax(),
            Self::Has(node) => node.syntax(),
            Self::Like(node) => node.syntax(),
            Self::Is(node) => node.syntax(),
            Self::Unary(node) => node.syntax(),
            Self::Member(node) => node.syntax(),
            Self::Literal(node) => node.syntax(),
            Self::EntityReference(node) => node.syntax(),
            Self::Slot(node) => node.syntax(),
            Self::Parenthesized(node) => node.syntax(),
            Self::List(node) => node.syntax(),
            Self::Record(node) => node.syntax(),
            Self::Name(node) => node.syntax(),
        }
    }
}
