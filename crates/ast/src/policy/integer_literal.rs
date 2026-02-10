use alloc::string::String;

use crate::error::Error;

/// An integer literal value.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct IntegerLiteral(i64);

impl IntegerLiteral {
    /// Parses an integer literal from text.
    ///
    /// # Errors
    ///
    /// Returns an error if `text` is invalid.
    pub fn new(text: &str) -> Result<Self, Error> {
        text.parse::<i64>()
            .map(Self)
            .map_err(|_err| Error::IntegerOverflow {
                text: String::from(text),
            })
    }

    /// Returns the integer value.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }
}
