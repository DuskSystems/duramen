#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Effect {
    Permit,
    Forbid,
}
