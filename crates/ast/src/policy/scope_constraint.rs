use crate::common::Name;
use crate::policy::EntityOrSlot;

/// A constraint on a principal or resource scope.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum ScopeConstraint<'a> {
    Any,
    Equal(EntityOrSlot<'a>),
    In(EntityOrSlot<'a>),
    Is(Name<'a>),
    IsIn(Name<'a>, EntityOrSlot<'a>),
}
