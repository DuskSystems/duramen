#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NodeId(u32);

impl NodeId {
    pub const NONE: Self = Self(u32::MAX);

    #[inline(always)]
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}
