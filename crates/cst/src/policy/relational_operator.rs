#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum RelationalOperator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    In,
}
