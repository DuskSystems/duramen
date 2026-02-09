use core::fmt;

/// The effect of a policy.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Effect {
    Permit,
    Forbid,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Permit => f.write_str("permit"),
            Self::Forbid => f.write_str("forbid"),
        }
    }
}
