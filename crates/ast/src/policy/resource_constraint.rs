use crate::policy::ScopeConstraint;

/// A constraint on the resource scope.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct ResourceConstraint<'a>(ScopeConstraint<'a>);

impl<'a> ResourceConstraint<'a> {
    /// Creates a new resource constraint.
    #[must_use]
    pub const fn new(constraint: ScopeConstraint<'a>) -> Self {
        Self(constraint)
    }

    /// Returns the scope constraint.
    #[must_use]
    pub const fn constraint(&self) -> &ScopeConstraint<'a> {
        &self.0
    }
}
