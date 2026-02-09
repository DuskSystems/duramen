use core::fmt;
use core::ops::Range;

use crate::Error;

/// A non-empty identifier string.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Identifier<'a>(&'a str);

impl<'a> Identifier<'a> {
    /// Creates an identifier from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is empty.
    pub const fn new(value: &'a str, span: Range<usize>) -> Result<Self, Error> {
        if value.is_empty() {
            return Err(Error::Empty { span });
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
