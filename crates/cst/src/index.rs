#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeIndex(u32);

impl NodeIndex {
    pub const NONE: Self = Self(0);

    #[inline(always)]
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index as u32 + 1)
    }

    #[inline(always)]
    #[must_use]
    pub const fn get(self) -> Option<usize> {
        if self.is_none() {
            None
        } else {
            Some((self.0 - 1) as usize)
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_none(self) -> bool {
        self.0 == Self::NONE.0
    }
}
