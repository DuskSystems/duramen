#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum UnaryOp {
    Not,
    Neg,
    IsEmpty,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum BinaryOp {
    Eq,
    Less,
    LessEq,
    Add,
    Sub,
    Mul,
    In,
    Contains,
    ContainsAll,
    ContainsAny,
    GetTag,
    HasTag,
}
