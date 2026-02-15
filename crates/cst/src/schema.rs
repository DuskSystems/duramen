use duramen_syntax::{Group, Node};

use crate::CstNode;
use crate::common::Annotation;

mod action_attributes;
pub use action_attributes::ActionAttributes;

mod action_declaration;
pub use action_declaration::ActionDeclaration;

mod action_parents;
pub use action_parents::ActionParents;

mod applies_to;
pub use applies_to::AppliesTo;

mod attribute_declaration;
pub use attribute_declaration::AttributeDeclaration;

mod context_type;
pub use context_type::ContextType;

mod entity_attributes;
pub use entity_attributes::EntityAttributes;

mod entity_declaration;
pub use entity_declaration::EntityDeclaration;

mod entity_parents;
pub use entity_parents::EntityParents;

mod entity_tags;
pub use entity_tags::EntityTags;

mod entity_type;
pub use entity_type::EntityType;

mod enum_type;
pub use enum_type::EnumType;

mod namespace;
pub use namespace::Namespace;

mod principal_types;
pub use principal_types::PrincipalTypes;

mod record_type;
pub use record_type::RecordType;

mod resource_types;
pub use resource_types::ResourceTypes;

mod set_type;
pub use set_type::SetType;

mod type_declaration;
pub use type_declaration::TypeDeclaration;

mod type_expression;
pub use type_expression::TypeExpression;

mod types;
pub use types::Types;

#[derive(Clone, Copy, Debug)]
pub struct Schema<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Schema<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind().group()? {
            Group::Schema => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Schema<'a> {
    /// Returns an iterator over namespace children.
    pub fn namespaces(&self) -> impl Iterator<Item = Namespace<'a>> {
        self.node.children().filter_map(Namespace::cast)
    }

    /// Returns an iterator over top-level entity declaration children.
    pub fn entity_declarations(&self) -> impl Iterator<Item = EntityDeclaration<'a>> {
        self.node.children().filter_map(EntityDeclaration::cast)
    }

    /// Returns an iterator over top-level action declaration children.
    pub fn action_declarations(&self) -> impl Iterator<Item = ActionDeclaration<'a>> {
        self.node.children().filter_map(ActionDeclaration::cast)
    }

    /// Returns an iterator over top-level type declaration children.
    pub fn type_declarations(&self) -> impl Iterator<Item = TypeDeclaration<'a>> {
        self.node.children().filter_map(TypeDeclaration::cast)
    }

    /// Returns an iterator over top-level annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }
}
