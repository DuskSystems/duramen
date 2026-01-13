use core::fmt::{self, Write};

use super::{
    AstNode, AstToken as _, Declaration, IdentifierToken, IntegerToken, SchemaNode, StringToken,
    TypeExpr, ast_node,
};
use crate::schema::SchemaSyntax;

ast_node!(Schema, SchemaSyntax::Schema);

impl<'a> Schema<'a> {
    /// Returns an iterator over namespace declarations in this schema.
    pub fn namespaces(&self) -> impl Iterator<Item = Namespace<'a>> + use<'a> {
        self.node.children().filter_map(Namespace::cast)
    }

    /// Returns an iterator over top-level declarations (outside namespaces).
    pub fn declarations(&self) -> impl Iterator<Item = Declaration<'a>> + use<'a> {
        self.node.children().filter_map(Declaration::cast)
    }
}

ast_node!(Namespace, SchemaSyntax::Namespace);

impl<'a> Namespace<'a> {
    /// Returns an iterator over annotations on this namespace.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the qualified name of this namespace.
    ///
    /// ```cedarschema
    /// namespace Acme::Corp { ... }
    /// //        ^^^^^^^^^^ name
    /// ```
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns an iterator over declarations within this namespace.
    pub fn declarations(&self) -> impl Iterator<Item = Declaration<'a>> + use<'a> {
        self.node.children().filter_map(Declaration::cast)
    }
}

ast_node!(Annotation, SchemaSyntax::Annotation);

impl<'a> Annotation<'a> {
    /// Returns the annotation name (identifier or keyword after `@`).
    ///
    /// ```cedarschema
    /// @doc("description")
    /// // ^^^ name is "doc"
    /// ```
    #[must_use]
    pub fn name(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == SchemaSyntax::Identifier || node.value().is_keyword())
            .and_then(IdentifierToken::cast)
    }

    /// Returns the annotation value (string in parentheses).
    ///
    /// ```cedarschema
    /// @doc("User accounts")
    /// //   ^^^^^^^^^^^^^^^ value
    /// ```
    #[must_use]
    pub fn value(&self) -> Option<StringToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == SchemaSyntax::String)
            .and_then(StringToken::cast)
    }
}

ast_node!(Name, SchemaSyntax::Name);

impl<'a> Name<'a> {
    /// Returns an iterator over the identifier segments of this name.
    ///
    /// ```cedarschema
    /// entity User in [Acme::Corp::Group];
    /// //              ^^^^ ^^^^ ^^^^^ segments: ["Acme", "Corp", "Group"]
    /// ```
    pub fn segments(&self) -> impl Iterator<Item = IdentifierToken<'a>> + use<'a> {
        self.node
            .children()
            .filter(|node| node.value() == SchemaSyntax::Identifier)
            .filter_map(IdentifierToken::cast)
    }

    /// Returns `true` if this is a simple (unqualified) name with a single segment.
    #[must_use]
    pub fn is_simple(&self) -> bool {
        let mut segments = self.segments();
        segments.next().is_some() && segments.next().is_none()
    }

    /// Returns the first segment if this is a simple (unqualified) name.
    #[must_use]
    pub fn as_simple(&self) -> Option<IdentifierToken<'a>> {
        let mut segments = self.segments();
        let first = segments.next()?;
        if segments.next().is_some() {
            return None;
        }
        Some(first)
    }

    /// Writes the full qualified name to the given writer.
    pub fn write_to<W: Write>(&self, source: &str, writer: &mut W) -> fmt::Result {
        let mut first = true;
        for segment in self.segments() {
            if !first {
                writer.write_str("::")?;
            }

            first = false;
            writer.write_str(segment.text(source))?;
        }

        Ok(())
    }
}

ast_node!(EntityDeclaration, SchemaSyntax::EntityDeclaration);

impl<'a> EntityDeclaration<'a> {
    /// Returns an iterator over annotations on this entity declaration.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns an iterator over entity type names being declared.
    ///
    /// Multiple names can be declared together: `entity User, Admin, Guest;`
    pub fn names(&self) -> impl Iterator<Item = IdentifierToken<'a>> + use<'a> {
        self.node
            .children()
            .filter(|node| node.value() == SchemaSyntax::Identifier)
            .filter_map(IdentifierToken::cast)
    }

    /// Returns the parent types clause (`in [...]`).
    ///
    /// ```cedarschema
    /// entity User in [Group, Team];
    /// //          ^^^^^^^^^^^^^^^^ parents
    /// ```
    #[must_use]
    pub fn parents(&self) -> Option<EntityParents<'a>> {
        self.node.children().find_map(EntityParents::cast)
    }

    /// Returns the attributes block.
    ///
    /// ```cedarschema
    /// entity User {
    ///     name: String,
    ///     email?: String,
    /// };
    /// ```
    #[must_use]
    pub fn attributes(&self) -> Option<EntityAttributes<'a>> {
        self.node.children().find_map(EntityAttributes::cast)
    }

    /// Returns the tags type specification.
    ///
    /// ```cedarschema
    /// entity Document tags String;
    /// ```
    #[must_use]
    pub fn tags(&self) -> Option<EntityTags<'a>> {
        self.node.children().find_map(EntityTags::cast)
    }

    /// Returns the enum type definition if this is an enum entity.
    ///
    /// ```cedarschema
    /// entity Status = ["active", "inactive"];
    /// //            ^^^^^^^^^^^^^^^^^^^^^^^^ enum_type
    /// ```
    #[must_use]
    pub fn enum_type(&self) -> Option<EnumType<'a>> {
        self.node.children().find_map(EnumType::cast)
    }
}

ast_node!(EntityParents, SchemaSyntax::EntityParents);

impl<'a> EntityParents<'a> {
    /// Returns the list of parent entity types.
    #[must_use]
    pub fn type_list(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

ast_node!(EntityAttributes, SchemaSyntax::EntityAttributes);

impl<'a> EntityAttributes<'a> {
    /// Returns the record type defining the attributes.
    #[must_use]
    pub fn record_type(&self) -> Option<RecordType<'a>> {
        self.node.children().find_map(RecordType::cast)
    }
}

ast_node!(EntityTags, SchemaSyntax::EntityTags);

impl<'a> EntityTags<'a> {
    /// Returns the type of tag values.
    #[must_use]
    pub fn tag_type(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

ast_node!(EnumType, SchemaSyntax::EnumType);

impl<'a> EnumType<'a> {
    /// Returns an iterator over enum variant strings.
    ///
    /// ```cedarschema
    /// entity Status = ["active", "inactive"];
    /// //               ^^^^^^^^  ^^^^^^^^^^ variants
    /// ```
    pub fn variants(&self) -> impl Iterator<Item = StringToken<'a>> + use<'a> {
        self.node
            .children()
            .filter(|node| node.value() == SchemaSyntax::String)
            .filter_map(StringToken::cast)
    }
}

ast_node!(ActionDeclaration, SchemaSyntax::ActionDeclaration);

impl<'a> ActionDeclaration<'a> {
    /// Returns an iterator over annotations on this action declaration.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns an iterator over action names being declared.
    ///
    /// Multiple actions can be declared together: `action read, write, delete;`
    pub fn names(&self) -> impl Iterator<Item = IdentifierToken<'a>> + use<'a> {
        self.node
            .children()
            .filter(|node| node.value() == SchemaSyntax::Identifier)
            .filter_map(IdentifierToken::cast)
    }

    /// Returns the parent actions clause (`in [...]`).
    ///
    /// ```cedarschema
    /// action read in [access];
    /// //          ^^^^^^^^^^^ parents
    /// ```
    #[must_use]
    pub fn parents(&self) -> Option<ActionParents<'a>> {
        self.node.children().find_map(ActionParents::cast)
    }

    /// Returns the `appliesTo` clause specifying principal/resource/context types.
    #[must_use]
    pub fn applies_to(&self) -> Option<AppliesTo<'a>> {
        self.node.children().find_map(AppliesTo::cast)
    }

    /// Returns the action attributes block.
    #[must_use]
    pub fn attributes(&self) -> Option<ActionAttributes<'a>> {
        self.node.children().find_map(ActionAttributes::cast)
    }
}

ast_node!(ActionParents, SchemaSyntax::ActionParents);

impl<'a> ActionParents<'a> {
    /// Returns an iterator over parent action names.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.node.children().filter_map(Name::cast)
    }
}

ast_node!(AppliesTo, SchemaSyntax::AppliesTo);

impl<'a> AppliesTo<'a> {
    /// Returns the principal types specification.
    ///
    /// ```cedarschema
    /// appliesTo { principal: [User, Admin], ... }
    /// //          ^^^^^^^^^^^^^^^^^^^^^^^ principal_types
    /// ```
    #[must_use]
    pub fn principal_types(&self) -> Option<PrincipalTypes<'a>> {
        self.node.children().find_map(PrincipalTypes::cast)
    }

    /// Returns the resource types specification.
    #[must_use]
    pub fn resource_types(&self) -> Option<ResourceTypes<'a>> {
        self.node.children().find_map(ResourceTypes::cast)
    }

    /// Returns the context type specification.
    #[must_use]
    pub fn context_type(&self) -> Option<ContextType<'a>> {
        self.node.children().find_map(ContextType::cast)
    }
}

ast_node!(PrincipalTypes, SchemaSyntax::PrincipalTypes);

impl<'a> PrincipalTypes<'a> {
    /// Returns the list of principal entity types.
    #[must_use]
    pub fn type_list(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

ast_node!(ResourceTypes, SchemaSyntax::ResourceTypes);

impl<'a> ResourceTypes<'a> {
    /// Returns the list of resource entity types.
    #[must_use]
    pub fn type_list(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

ast_node!(ContextType, SchemaSyntax::ContextType);

impl<'a> ContextType<'a> {
    /// Returns the context type expression (typically a record type).
    #[must_use]
    pub fn type_expr(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

ast_node!(ActionAttributes, SchemaSyntax::ActionAttributes);

impl<'a> ActionAttributes<'a> {
    /// Returns an iterator over attribute entries.
    pub fn entries(&self) -> impl Iterator<Item = AttributeEntry<'a>> + use<'a> {
        self.node.children().filter_map(AttributeEntry::cast)
    }
}

ast_node!(AttributeEntry, SchemaSyntax::AttributeEntry);

impl<'a> AttributeEntry<'a> {
    /// Returns the attribute key (identifier).
    #[must_use]
    pub fn key(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == SchemaSyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }

    /// Returns the attribute value (literal).
    #[must_use]
    pub fn value(&self) -> Option<LiteralValue<'a>> {
        self.node.children().find_map(LiteralValue::cast)
    }
}

/// Literal value in action attributes.
///
/// Can be an integer, string, or boolean.
#[derive(Debug, Clone, Copy)]
pub enum LiteralValue<'a> {
    Integer(IntegerToken<'a>),
    String(StringToken<'a>),
    Bool(SchemaNode<'a>),
}

impl<'a> LiteralValue<'a> {
    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        match node.value() {
            SchemaSyntax::Integer => IntegerToken::cast(node).map(Self::Integer),
            SchemaSyntax::String => StringToken::cast(node).map(Self::String),
            SchemaSyntax::TrueKeyword | SchemaSyntax::FalseKeyword => Some(Self::Bool(node)),
            _ => None,
        }
    }

    #[must_use]
    pub fn syntax(&self) -> &SchemaNode<'a> {
        match self {
            Self::Integer(token) => token.syntax(),
            Self::String(token) => token.syntax(),
            Self::Bool(node) => node,
        }
    }

    /// Returns the boolean value if this is a `true` or `false` literal.
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(node) => match node.value() {
                SchemaSyntax::TrueKeyword => Some(true),
                SchemaSyntax::FalseKeyword => Some(false),
                _ => None,
            },
            _ => None,
        }
    }
}

ast_node!(TypeDeclaration, SchemaSyntax::CommonTypeDeclaration);

impl<'a> TypeDeclaration<'a> {
    /// Returns an iterator over annotations on this type declaration.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the name of the type being defined.
    ///
    /// ```cedarschema
    /// type EmailAddress = String;
    /// //   ^^^^^^^^^^^^ name
    /// ```
    #[must_use]
    pub fn name(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == SchemaSyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }

    /// Returns the type definition (right-hand side of `=`).
    ///
    /// ```cedarschema
    /// type EmailAddress = String;
    /// //                  ^^^^^^ definition
    /// ```
    #[must_use]
    pub fn definition(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

ast_node!(TypeList, SchemaSyntax::TypeList);

impl<'a> TypeList<'a> {
    /// Returns an iterator over the type names in this list.
    pub fn types(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.node.children().filter_map(Name::cast)
    }
}

ast_node!(AttributeDeclaration, SchemaSyntax::AttributeDeclaration);

impl<'a> AttributeDeclaration<'a> {
    /// Returns an iterator over annotations on this attribute.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the attribute name (identifier or string).
    ///
    /// ```cedarschema
    /// entity User { name: String, "special-attr": Long };
    /// //            ^^^^          ^^^^^^^^^^^^^^ can be identifier or string
    /// ```
    #[must_use]
    pub fn name(&self) -> Option<AttrKey<'a>> {
        self.node.children().find_map(AttrKey::cast)
    }

    /// Returns `true` if this attribute is optional (has `?` after the name).
    ///
    /// ```cedarschema
    /// entity User { name: String, email?: String };
    /// //                          ^^^^^^ is_optional returns true
    /// ```
    #[must_use]
    pub fn is_optional(&self) -> bool {
        self.node
            .children()
            .any(|node| node.value() == SchemaSyntax::Question)
    }

    /// Returns the attribute's type.
    #[must_use]
    pub fn attribute_type(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

/// Attribute key (name) which can be an identifier, a quoted string, or a keyword.
///
/// ```cedarschema
/// entity User {
///     name: String,           // identifier key
///     "special-attr": Long,   // string key (for reserved words or special chars)
///     String?: __cedar::String, // keyword key (reserved word used as attribute name)
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub enum AttrKey<'a> {
    Identifier(IdentifierToken<'a>),
    String(StringToken<'a>),
    Keyword(SchemaNode<'a>),
}

impl<'a> AttrKey<'a> {
    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        match node.value() {
            SchemaSyntax::Identifier => IdentifierToken::cast(node).map(Self::Identifier),
            SchemaSyntax::String => StringToken::cast(node).map(Self::String),
            kind if kind.is_keyword() => Some(Self::Keyword(node)),
            _ => None,
        }
    }

    #[must_use]
    pub fn syntax(&self) -> &SchemaNode<'a> {
        match self {
            Self::Identifier(token) => token.syntax(),
            Self::String(token) => token.syntax(),
            Self::Keyword(node) => node,
        }
    }
}

ast_node!(RecordType, SchemaSyntax::RecordType);

impl<'a> RecordType<'a> {
    /// Returns an iterator over attribute declarations in this record type.
    pub fn attributes(&self) -> impl Iterator<Item = AttributeDeclaration<'a>> + use<'a> {
        self.node.children().filter_map(AttributeDeclaration::cast)
    }
}
