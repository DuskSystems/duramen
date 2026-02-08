mod action_attributes;
pub use action_attributes::ActionAttributes;

mod action_declaration;
pub use action_declaration::ActionDeclaration;

mod action_parents;
pub use action_parents::ActionParents;

mod applies_to_clause;
pub use applies_to_clause::AppliesToClause;

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

mod namespace_declaration;
pub use namespace_declaration::NamespaceDeclaration;

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

mod r#type;
pub use r#type::TypeExpression;

mod type_list;
use duramen_syntax::{Node, Syntax};
pub use type_list::TypeList;

use crate::CstNode;

#[derive(Clone, Copy, Debug)]
pub struct Schema<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Schema<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Schema => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}
