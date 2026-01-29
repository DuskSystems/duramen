#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Effect {
    Permit,
    Forbid,
}

impl Effect {
    #[must_use]
    pub const fn is_permit(self) -> bool {
        matches!(self, Self::Permit)
    }

    #[must_use]
    pub const fn is_forbid(self) -> bool {
        matches!(self, Self::Forbid)
    }
}
