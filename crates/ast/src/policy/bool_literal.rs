/// A boolean literal value.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct BoolLiteral(bool);

impl BoolLiteral {
    /// Creates a new boolean literal.
    #[must_use]
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the boolean value.
    #[must_use]
    pub const fn value(self) -> bool {
        self.0
    }
}
