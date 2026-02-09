use alloc::vec::Vec;

use crate::policy::Policy;

/// A collection of policies.
#[derive(Clone, Debug)]
pub struct PolicySet<'a> {
    policies: Vec<Policy<'a>>,
}

impl<'a> PolicySet<'a> {
    /// Creates a new policy set.
    #[must_use]
    pub const fn new(policies: Vec<Policy<'a>>) -> Self {
        Self { policies }
    }

    /// Returns the policies.
    #[must_use]
    pub fn policies(&self) -> &[Policy<'a>] {
        &self.policies
    }
}
