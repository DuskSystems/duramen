/// An element of a `like` pattern.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum PatternElement {
    Char(char),
    Wildcard,
}
