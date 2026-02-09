use alloc::borrow::Cow;

use crate::policy::EntityReference;

/// A literal value.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Literal<'a> {
    Bool(bool),
    Integer(i64),
    String(Cow<'a, str>),
    Entity(EntityReference<'a>),
}
