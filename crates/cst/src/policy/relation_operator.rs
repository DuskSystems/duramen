#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum RelationOperator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    In,
}
