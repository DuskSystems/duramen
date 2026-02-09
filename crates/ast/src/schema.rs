use alloc::vec::Vec;

mod action_declaration;
pub use action_declaration::ActionDeclaration;

mod action_reference;
pub use action_reference::ActionReference;

mod applies_to;
pub use applies_to::AppliesTo;

mod attribute_declaration;
pub use attribute_declaration::AttributeDeclaration;

mod context_type;
pub use context_type::ContextType;

mod declaration;
pub use declaration::Declaration;

mod entity_declaration;
pub use entity_declaration::EntityDeclaration;

mod entity_kind;
pub use entity_kind::EntityKind;

mod entity_type_set;
pub use entity_type_set::EntityTypeSet;

mod enum_choices;
pub use enum_choices::EnumChoices;

mod namespace;
pub use namespace::Namespace;

mod optionality;
pub use optionality::Optionality;

mod record_type;
pub use record_type::RecordType;

mod standard_entity;
pub use standard_entity::StandardEntity;

mod type_declaration;
pub use type_declaration::TypeDeclaration;

mod type_expression;
pub use type_expression::TypeExpression;

/// A Cedar schema.
#[derive(Clone, Debug)]
pub struct Schema<'a> {
    namespaces: Vec<Namespace<'a>>,
}

impl<'a> Schema<'a> {
    /// Creates a new schema.
    #[must_use]
    pub const fn new(namespaces: Vec<Namespace<'a>>) -> Self {
        Self { namespaces }
    }

    /// Returns the namespaces.
    #[must_use]
    pub fn namespaces(&self) -> &[Namespace<'a>] {
        &self.namespaces
    }
}
