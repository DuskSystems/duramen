use core::fmt;

pub const MIN: i64 = i64::MIN;
pub const MAX: i64 = i64::MAX;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Integer(i64);

impl Integer {
    pub const MAX: Self = Self(MAX);
    pub const MIN: Self = Self(MIN);
    pub const ONE: Self = Self(1);
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(result) => Some(Self(result)),
            None => None,
        }
    }

    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(result) => Some(Self(result)),
            None => None,
        }
    }

    #[must_use]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.0.checked_mul(rhs.0) {
            Some(result) => Some(Self(result)),
            None => None,
        }
    }

    #[must_use]
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(result) => Some(Self(result)),
            None => None,
        }
    }

    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }

    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<Integer> for i64 {
    fn from(value: Integer) -> Self {
        value.0
    }
}
