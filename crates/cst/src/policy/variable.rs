#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Variable {
    Principal,
    Action,
    Resource,
    Context,
}
