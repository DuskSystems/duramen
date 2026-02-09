use crate::schema::{EnumChoices, StandardEntity};

/// The kind of entity declaration.
#[derive(Clone, Debug)]
pub enum EntityKind<'a> {
    Standard(StandardEntity<'a>),
    Enum(EnumChoices<'a>),
}
