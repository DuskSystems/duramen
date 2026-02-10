use alloc::borrow::Cow;

/// An element of a `like` pattern.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PatternElement<'a> {
    Literal(Cow<'a, str>),
    Wildcard,
}
