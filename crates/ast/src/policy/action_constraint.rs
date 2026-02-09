use crate::policy::{ActionList, EntityReference};

/// A constraint on the action scope.
#[derive(Clone, Debug)]
pub enum ActionConstraint<'a> {
    Any,
    Equal(EntityReference<'a>),
    In(ActionList<'a>),
}

impl<'a> ActionConstraint<'a> {
    /// Returns an iterator over all entity references in this constraint.
    pub fn entities(&self) -> impl Iterator<Item = &EntityReference<'a>> {
        let single = match self {
            Self::Equal(entity) => Some(entity),
            Self::Any | Self::In(_) => None,
        };

        let list = match self {
            Self::In(list) => Some(list),
            Self::Any | Self::Equal(_) => None,
        };

        single
            .into_iter()
            .chain(list.into_iter().flat_map(ActionList::iter))
    }
}
