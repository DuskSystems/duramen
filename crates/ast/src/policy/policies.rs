use alloc::vec::Vec;

use crate::policy::Policy;

/// A collection of policies.
#[derive(Clone, Debug)]
pub struct Policies<'a> {
    policies: Vec<Policy<'a>>,
}

impl<'a> Policies<'a> {
    /// Creates a new policy collection.
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
