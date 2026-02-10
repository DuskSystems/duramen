use alloc::borrow::Cow;

/// A string literal value.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct StringLiteral<'a>(Cow<'a, str>);

impl<'a> StringLiteral<'a> {
    /// Creates a new string literal.
    #[must_use]
    pub const fn new(value: Cow<'a, str>) -> Self {
        Self(value)
    }

    /// Returns the string value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}
