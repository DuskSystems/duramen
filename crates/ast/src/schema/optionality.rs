/// Whether an attribute is required or optional.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Optionality {
    Required,
    Optional,
}
