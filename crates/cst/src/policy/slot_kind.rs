#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum SlotKind {
    Principal,
    Resource,
    Other,
}
