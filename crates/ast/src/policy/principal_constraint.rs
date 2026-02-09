use crate::policy::ScopeConstraint;

/// A constraint on the principal scope.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct PrincipalConstraint<'a>(ScopeConstraint<'a>);

impl<'a> PrincipalConstraint<'a> {
    /// Creates a new principal constraint.
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
