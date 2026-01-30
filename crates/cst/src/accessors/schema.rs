use syntree::Node;

use super::CstNode;
use crate::{SchemaNode, SchemaSyntax};

macro_rules! cst_node {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            node: SchemaNode<'a>,
        }

        impl<'a> CstNode<'a> for $name<'a> {
            type Syntax = SchemaSyntax;

            fn can_cast(kind: SchemaSyntax) -> bool {
                kind == $kind
            }

            fn cast(node: SchemaNode<'a>) -> Option<Self> {
                Self::can_cast(node.value()).then_some(Self { node })
            }

            fn syntax(&self) -> SchemaNode<'a> {
                self.node
            }
        }
    };
}

cst_node!(Schema, SchemaSyntax::Schema);
impl<'a> Schema<'a> {
    pub fn namespaces(&self) -> impl Iterator<Item = NamespaceDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(NamespaceDecl::cast)
    }

    pub fn entities(&self) -> impl Iterator<Item = EntityDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(EntityDecl::cast)
    }

    pub fn actions(&self) -> impl Iterator<Item = ActionDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(ActionDecl::cast)
    }

    pub fn types(&self) -> impl Iterator<Item = TypeDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(TypeDecl::cast)
    }
}

cst_node!(NamespaceDecl, SchemaSyntax::NamespaceDeclaration);
impl<'a> NamespaceDecl<'a> {
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    pub fn entities(&self) -> impl Iterator<Item = EntityDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(EntityDecl::cast)
    }

    pub fn actions(&self) -> impl Iterator<Item = ActionDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(ActionDecl::cast)
    }

    pub fn types(&self) -> impl Iterator<Item = TypeDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(TypeDecl::cast)
    }

    pub fn namespaces(&self) -> impl Iterator<Item = NamespaceDecl<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(NamespaceDecl::cast)
    }
}

cst_node!(EntityDecl, SchemaSyntax::EntityDeclaration);
impl<'a> EntityDecl<'a> {
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.node.children().filter_map(Name::cast)
    }

    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    #[must_use]
    pub fn is_enum(&self) -> bool {
        self.node
            .children()
            .any(|child| child.value() == SchemaSyntax::Enum)
    }

    #[must_use]
    pub fn parents(&self) -> Option<EntityParents<'a>> {
        self.node.children().find_map(EntityParents::cast)
    }

    #[must_use]
    pub fn attributes(&self) -> Option<EntityAttributes<'a>> {
        self.node.children().find_map(EntityAttributes::cast)
    }

    #[must_use]
    pub fn tags(&self) -> Option<EntityTags<'a>> {
        self.node.children().find_map(EntityTags::cast)
    }

    #[must_use]
    pub fn enum_type(&self) -> Option<EnumType<'a>> {
        self.node.children().find_map(EnumType::cast)
    }
}

cst_node!(ActionDecl, SchemaSyntax::ActionDeclaration);
impl<'a> ActionDecl<'a> {
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.node.children().filter_map(Name::cast)
    }

    pub fn action_names<'s>(&self, source: &'s str) -> impl Iterator<Item = &'s str> + use<'a, 's> {
        self.node.children().filter_map(|child| {
            if child.value() == SchemaSyntax::String {
                let text = &source[child.range()];
                text.get(1..text.len().saturating_sub(1))
            } else if let Some(name) = Name::cast(child) {
                name.basename(source)
            } else {
                None
            }
        })
    }

    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    #[must_use]
    pub fn applies_to(&self) -> Option<AppliesToClause<'a>> {
        self.node.children().find_map(AppliesToClause::cast)
    }

    #[must_use]
    pub fn parents(&self) -> Option<ActionParents<'a>> {
        self.node.children().find_map(ActionParents::cast)
    }

    #[must_use]
    pub fn attributes(&self) -> Option<ActionAttributes<'a>> {
        self.node.children().find_map(ActionAttributes::cast)
    }
}

cst_node!(TypeDecl, SchemaSyntax::TypeDeclaration);
impl<'a> TypeDecl<'a> {
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    #[must_use]
    pub fn type_expr(&self) -> Option<TypeExpr<'a>> {
        let mut skipped_name = false;
        self.node
            .children()
            .filter(move |node| {
                if node.value() == SchemaSyntax::Name && !skipped_name {
                    skipped_name = true;
                    false
                } else {
                    true
                }
            })
            .find_map(TypeExpr::cast)
    }
}

cst_node!(Annotation, SchemaSyntax::Annotation);
impl Annotation<'_> {
    #[must_use]
    pub fn name<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|node| node.value() == SchemaSyntax::Identifier || node.value().is_name_keyword())
            .map(|node| &source[node.range()])
    }

    #[must_use]
    pub fn value<'s>(&self, source: &'s str) -> Option<&'s str> {
        let child = self
            .node
            .children()
            .find(|child| child.value() == SchemaSyntax::String)?;

        let text = &source[child.range()];
        text.get(1..text.len().saturating_sub(1))
    }
}

cst_node!(EntityParents, SchemaSyntax::EntityParents);
impl<'a> EntityParents<'a> {
    #[must_use]
    pub fn types(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

cst_node!(EntityAttributes, SchemaSyntax::EntityAttributes);
impl<'a> EntityAttributes<'a> {
    pub fn attributes(&self) -> impl Iterator<Item = AttributeDecl<'a>> + use<'a> {
        self.node.children().filter_map(AttributeDecl::cast)
    }
}

cst_node!(EntityTags, SchemaSyntax::EntityTags);
impl<'a> EntityTags<'a> {
    #[must_use]
    pub fn type_expr(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

cst_node!(AppliesToClause, SchemaSyntax::AppliesToClause);
impl<'a> AppliesToClause<'a> {
    #[must_use]
    pub fn principal_types(&self) -> Option<PrincipalTypes<'a>> {
        self.node.children().find_map(PrincipalTypes::cast)
    }

    #[must_use]
    pub fn resource_types(&self) -> Option<ResourceTypes<'a>> {
        self.node.children().find_map(ResourceTypes::cast)
    }

    #[must_use]
    pub fn context_type(&self) -> Option<ContextType<'a>> {
        self.node.children().find_map(ContextType::cast)
    }
}

cst_node!(PrincipalTypes, SchemaSyntax::PrincipalTypes);
impl<'a> PrincipalTypes<'a> {
    #[must_use]
    pub fn types(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

cst_node!(ResourceTypes, SchemaSyntax::ResourceTypes);
impl<'a> ResourceTypes<'a> {
    #[must_use]
    pub fn types(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

cst_node!(ContextType, SchemaSyntax::ContextType);
impl<'a> ContextType<'a> {
    #[must_use]
    pub fn type_expr(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

cst_node!(ActionParents, SchemaSyntax::ActionParents);
impl<'a> ActionParents<'a> {
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.node.children().filter_map(Name::cast)
    }

    pub fn qualified_names<'s>(
        &self,
        source: &'s str,
    ) -> impl Iterator<Item = (Name<'a>, Option<&'s str>)> + use<'a, 's> {
        let mut children = self.node.children().peekable();
        core::iter::from_fn(move || {
            loop {
                let child = children.next()?;
                let Some(name) = Name::cast(child) else {
                    continue;
                };

                let eid = match children.peek().map(Node::value) {
                    Some(SchemaSyntax::Colon2) => {
                        children.next();
                        if let Some(string) = children.peek()
                            && string.value() == SchemaSyntax::String
                        {
                            let text = &source[string.range()];
                            children.next();
                            text.get(1..text.len().saturating_sub(1))
                        } else {
                            None
                        }
                    }
                    Some(SchemaSyntax::String) => {
                        let string = children.next()?;
                        let text = &source[string.range()];
                        text.get(1..text.len().saturating_sub(1))
                    }
                    _ => None,
                };

                return Some((name, eid));
            }
        })
    }
}

cst_node!(ActionAttributes, SchemaSyntax::ActionAttributes);
impl<'a> ActionAttributes<'a> {
    pub fn attributes(&self) -> impl Iterator<Item = AttributeDecl<'a>> + use<'a> {
        self.node.children().filter_map(AttributeDecl::cast)
    }
}

cst_node!(AttributeDecl, SchemaSyntax::AttributeDeclaration);
impl<'a> AttributeDecl<'a> {
    #[must_use]
    pub fn name<'s>(&self, source: &'s str) -> Option<&'s str> {
        let child = self.node.children().find(|child| {
            let kind = child.value();
            kind == SchemaSyntax::String || kind == SchemaSyntax::Identifier || kind.is_keyword()
        })?;

        let text = &source[child.range()];

        if child.value() == SchemaSyntax::String {
            text.get(1..text.len().saturating_sub(1))
        } else {
            Some(text)
        }
    }

    #[must_use]
    pub fn is_optional(&self) -> bool {
        self.node
            .children()
            .any(|child| child.value() == SchemaSyntax::Question)
    }

    #[must_use]
    pub fn type_expr(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }

    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }
}

cst_node!(Name, SchemaSyntax::Name);
impl<'a> Name<'a> {
    pub fn segments<'s>(&self, source: &'s str) -> impl Iterator<Item = &'s str> + use<'a, 's> {
        self.node
            .children()
            .filter(|node| {
                node.value() == SchemaSyntax::Identifier || node.value().is_name_keyword()
            })
            .map(|node| &source[node.range()])
    }

    #[must_use]
    pub fn is_qualified(&self) -> bool {
        self.node
            .children()
            .filter(|node| {
                node.value() == SchemaSyntax::Identifier || node.value().is_name_keyword()
            })
            .nth(1)
            .is_some()
    }

    #[must_use]
    pub fn basename<'s>(&self, source: &'s str) -> Option<&'s str> {
        if let Some(string_node) = self
            .node
            .children()
            .find(|node| node.value() == SchemaSyntax::String)
        {
            let text = &source[string_node.range()];
            return text.get(1..text.len().saturating_sub(1));
        }

        self.node
            .children()
            .filter(|node| {
                node.value() == SchemaSyntax::Identifier || node.value().is_name_keyword()
            })
            .last()
            .map(|node| &source[node.range()])
    }

    pub fn namespace<'s>(&self, source: &'s str) -> impl Iterator<Item = &'s str> + use<'a, 's> {
        let segments: alloc::vec::Vec<_> = self
            .node
            .children()
            .filter(|node| {
                node.value() == SchemaSyntax::Identifier || node.value().is_name_keyword()
            })
            .collect();

        let count = segments.len().saturating_sub(1);
        segments
            .into_iter()
            .take(count)
            .map(|node| &source[node.range()])
    }
}

cst_node!(TypeList, SchemaSyntax::TypeList);
impl<'a> TypeList<'a> {
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.node.children().filter_map(Name::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeExpr<'a> {
    Set(SetType<'a>),
    Record(RecordType<'a>),
    Entity(EntityType<'a>),
    Enum(EnumType<'a>),
    Reference(Name<'a>),
}

impl<'a> CstNode<'a> for TypeExpr<'a> {
    type Syntax = SchemaSyntax;

    fn can_cast(kind: SchemaSyntax) -> bool {
        matches!(
            kind,
            SchemaSyntax::TypeExpr
                | SchemaSyntax::SetType
                | SchemaSyntax::RecordType
                | SchemaSyntax::EntityType
                | SchemaSyntax::EnumType
                | SchemaSyntax::Name
        )
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        match node.value() {
            SchemaSyntax::TypeExpr => node.children().skip_tokens().find_map(Self::cast),
            SchemaSyntax::SetType => SetType::cast(node).map(Self::Set),
            SchemaSyntax::RecordType => RecordType::cast(node).map(Self::Record),
            SchemaSyntax::EntityType => EntityType::cast(node).map(Self::Entity),
            SchemaSyntax::EnumType => EnumType::cast(node).map(Self::Enum),
            SchemaSyntax::Name => Name::cast(node).map(Self::Reference),
            _ => None,
        }
    }

    fn syntax(&self) -> SchemaNode<'a> {
        match self {
            Self::Set(t) => t.syntax(),
            Self::Record(t) => t.syntax(),
            Self::Entity(t) => t.syntax(),
            Self::Enum(t) => t.syntax(),
            Self::Reference(t) => t.syntax(),
        }
    }
}

cst_node!(SetType, SchemaSyntax::SetType);
impl<'a> SetType<'a> {
    #[must_use]
    pub fn element_type(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

cst_node!(RecordType, SchemaSyntax::RecordType);
impl<'a> RecordType<'a> {
    pub fn attributes(&self) -> impl Iterator<Item = AttributeDecl<'a>> + use<'a> {
        self.node.children().filter_map(AttributeDecl::cast)
    }
}

cst_node!(EntityType, SchemaSyntax::EntityType);
impl<'a> EntityType<'a> {
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }
}

cst_node!(EnumType, SchemaSyntax::EnumType);
impl<'a> EnumType<'a> {
    pub fn variants<'s>(&self, source: &'s str) -> impl Iterator<Item = &'s str> + use<'a, 's> {
        self.node
            .children()
            .filter(|node| node.value() == SchemaSyntax::EnumVariant)
            .filter_map(|node| {
                let text = &source[node.range()];
                text.get(1..text.len().saturating_sub(1))
            })
    }
}
