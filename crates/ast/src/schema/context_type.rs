use crate::common::Name;
use crate::schema::RecordType;

/// A context type in an applies-to clause.
///
/// Cedar restricts context types to either a reference to a named type
/// (which must resolve to a record) or an inline record definition.
#[derive(Clone, Debug)]
pub enum ContextType<'a> {
    /// A reference to a named type (e.g. `MyContextType`).
    Reference(Name<'a>),
    /// An inline record definition (e.g. `{ key: String }`).
    Record(RecordType<'a>),
}
