use alloc::string::String;
use core::fmt;

use crate::error::Error;

/// A non-empty identifier string.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Identifier<'a>(&'a str);

impl<'a> Identifier<'a> {
    /// Creates an identifier from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is invalid.
    pub fn new(value: &'a str) -> Result<Self, Error> {
        if value.is_empty() {
            return Err(Error::Empty);
        }

        if value.starts_with("__cedar") {
            return Err(Error::ReservedPrefix {
                name: String::from(value),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier string.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

impl fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
