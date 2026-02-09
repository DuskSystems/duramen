use core::fmt;

/// A Cedar variable.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Variable {
    Principal,
    Action,
    Resource,
    Context,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Principal => f.write_str("principal"),
            Self::Action => f.write_str("action"),
            Self::Resource => f.write_str("resource"),
            Self::Context => f.write_str("context"),
        }
    }
}
