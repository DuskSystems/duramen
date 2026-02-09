use alloc::boxed::Box;

use crate::common::Name;
use crate::schema::{EntityTypeSet, EnumType, RecordType};

/// A type expression in a schema.
#[derive(Clone, Debug)]
pub enum TypeExpression<'a> {
    Reference(Name<'a>),
    Set(Box<Self>),
    Record(RecordType<'a>),
    Entity(EntityTypeSet<'a>),
    Enum(EnumType<'a>),
}
