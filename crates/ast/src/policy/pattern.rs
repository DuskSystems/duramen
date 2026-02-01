use alloc::vec::Vec;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pattern(Vec<PatternElem>);

impl Pattern {
    #[must_use]
    pub const fn new(elements: Vec<PatternElem>) -> Self {
        Self(elements)
    }

    #[must_use]
    pub fn elements(&self) -> &[PatternElem] {
        &self.0
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PatternElem> {
        self.0.iter()
    }
}

impl From<Vec<PatternElem>> for Pattern {
    fn from(value: Vec<PatternElem>) -> Self {
        Self::new(value)
    }
}

impl FromIterator<PatternElem> for Pattern {
    fn from_iter<T: IntoIterator<Item = PatternElem>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum PatternElem {
    Char(char),
    Wildcard,
}

impl PatternElem {
    #[must_use]
    pub const fn is_wildcard(self) -> bool {
        matches!(self, Self::Wildcard)
    }

    #[must_use]
    pub const fn as_char(self) -> Option<char> {
        match self {
            Self::Char(char) => Some(char),
            Self::Wildcard => None,
        }
    }
}
