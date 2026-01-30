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
    Greater,
    GreaterEq,
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
