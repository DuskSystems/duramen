#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SlotId {
    Principal,
    Resource,
}

impl SlotId {
    #[must_use]
    pub const fn is_principal(self) -> bool {
        matches!(self, Self::Principal)
    }

    #[must_use]
    pub const fn is_resource(self) -> bool {
        matches!(self, Self::Resource)
    }
}
