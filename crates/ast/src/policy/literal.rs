use crate::policy::{BoolLiteral, EntityReference, IntegerLiteral, StringLiteral};

/// A literal value.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Literal<'a> {
    Bool(BoolLiteral),
    Integer(IntegerLiteral),
    String(StringLiteral<'a>),
    Entity(EntityReference<'a>),
}
