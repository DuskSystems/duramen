#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum LiteralKind {
    Bool,
    Integer,
    String,
}
