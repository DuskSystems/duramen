#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Var {
    Principal,
    Action,
    Resource,
    Context,
}
