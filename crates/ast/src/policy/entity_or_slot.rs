use crate::policy::EntityReference;

/// An entity reference or template slot.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EntityOrSlot<'a> {
    Entity(EntityReference<'a>),
    Slot,
}
