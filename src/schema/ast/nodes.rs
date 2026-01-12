use core::fmt::{self, Write};

use super::{
    AstNode, AstToken as _, Decl, IdentifierToken, IntegerToken, SchemaNode, StringToken, TypeExpr,
};
use crate::schema::SchemaSyntax;

/// Root node of a Cedar schema.
///
/// A schema can contain namespace declarations and/or top-level declarations.
///
/// ```cedarschema
/// namespace Acme {
///     entity User;
/// }
///
/// entity GlobalAdmin;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Schema<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for Schema<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::Schema
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> Schema<'a> {
    /// Returns an iterator over namespace declarations in this schema.
    pub fn namespaces(&self) -> impl Iterator<Item = Namespace<'a>> + 'a {
        self.node.children().filter_map(Namespace::cast)
    }

    /// Returns an iterator over top-level declarations (outside namespaces).
    pub fn declarations(&self) -> impl Iterator<Item = Decl<'a>> + 'a {
        self.node.children().filter_map(Decl::cast)
    }
}

/// Namespace declaration containing entity, action, and type declarations.
///
/// ```cedarschema
/// namespace Acme::Corp {
///     entity User in [Group] {
///         name: String,
///         email?: String,
///     };
///     entity Group;
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Namespace<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for Namespace<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::Namespace
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> Namespace<'a> {
    /// Returns an iterator over annotations on this namespace.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + 'a {
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
    pub fn declarations(&self) -> impl Iterator<Item = Decl<'a>> + 'a {
        self.node.children().filter_map(Decl::cast)
    }
}

/// Annotation on a declaration.
///
/// ```cedarschema
/// @doc("User accounts in the system")
/// entity User;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Annotation<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for Annotation<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::Annotation
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> Annotation<'a> {
    /// Returns the annotation name (identifier after `@`).
    ///
    /// ```cedarschema
    /// @doc("description")
    /// // ^^^ name is "doc"
    /// ```
    #[must_use]
    pub fn name(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == SchemaSyntax::Identifier)
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

/// Qualified name consisting of one or more `::` separated segments.
///
/// ```cedarschema
/// entity User in [Acme::Corp::Group];
/// //              ^^^^^^^^^^^^^^^^ Name with segments [Acme, Corp, Group]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Name<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for Name<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::Name
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> Name<'a> {
    /// Returns an iterator over the identifier segments of this name.
    ///
    /// ```cedarschema
    /// entity User in [Acme::Corp::Group];
    /// //              ^^^^ ^^^^ ^^^^^ segments: ["Acme", "Corp", "Group"]
    /// ```
    pub fn segments(&self) -> impl Iterator<Item = IdentifierToken<'a>> + 'a {
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

/// Entity type declaration.
///
/// ```cedarschema
/// entity User in [Group] {
///     name: String,
///     email?: String,
/// };
/// ```
///
/// Entities can also be declared as enums:
///
/// ```cedarschema
/// entity Status = ["active", "inactive", "pending"];
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EntityDecl<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for EntityDecl<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::EntityDeclaration
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> EntityDecl<'a> {
    /// Returns an iterator over annotations on this entity declaration.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + 'a {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns an iterator over entity type names being declared.
    ///
    /// Multiple names can be declared together: `entity User, Admin, Guest;`
    pub fn names(&self) -> impl Iterator<Item = IdentifierToken<'a>> + 'a {
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

/// Entity parent types clause.
///
/// ```cedarschema
/// entity User in [Group, Team];
/// //          ^^^^^^^^^^^^^^^^ EntityParents
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EntityParents<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for EntityParents<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::EntityParents
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> EntityParents<'a> {
    /// Returns the list of parent entity types.
    #[must_use]
    pub fn type_list(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

/// Entity attributes block.
///
/// ```cedarschema
/// entity User {
///     name: String,
///     email?: String,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EntityAttributes<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for EntityAttributes<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::EntityAttributes
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> EntityAttributes<'a> {
    /// Returns the record type defining the attributes.
    #[must_use]
    pub fn record_type(&self) -> Option<RecordType<'a>> {
        self.node.children().find_map(RecordType::cast)
    }
}

/// Entity tags type specification.
///
/// ```cedarschema
/// entity Document tags String;
/// //              ^^^^^^^^^^^ EntityTags
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EntityTags<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for EntityTags<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::EntityTags
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> EntityTags<'a> {
    /// Returns the type of tag values.
    #[must_use]
    pub fn tag_type(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

/// Enum type definition for entity types.
///
/// ```cedarschema
/// entity Status = ["active", "inactive", "pending"];
/// //            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ EnumType
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EnumType<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for EnumType<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::EnumType
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> EnumType<'a> {
    /// Returns an iterator over enum variant strings.
    ///
    /// ```cedarschema
    /// entity Status = ["active", "inactive"];
    /// //               ^^^^^^^^  ^^^^^^^^^^ variants
    /// ```
    pub fn variants(&self) -> impl Iterator<Item = StringToken<'a>> + 'a {
        self.node
            .children()
            .filter(|node| node.value() == SchemaSyntax::String)
            .filter_map(StringToken::cast)
    }
}

/// Action declaration.
///
/// ```cedarschema
/// action read, write appliesTo {
///     principal: [User, Admin],
///     resource: [Document, Folder],
///     context: { ip?: ipaddr },
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ActionDecl<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for ActionDecl<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::ActionDeclaration
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> ActionDecl<'a> {
    /// Returns an iterator over annotations on this action declaration.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + 'a {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns an iterator over action names being declared.
    ///
    /// Multiple actions can be declared together: `action read, write, delete;`
    pub fn names(&self) -> impl Iterator<Item = IdentifierToken<'a>> + 'a {
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

/// Action parent actions clause.
///
/// ```cedarschema
/// action read in [access, view];
/// //          ^^^^^^^^^^^^^^^^^ ActionParents
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ActionParents<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for ActionParents<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::ActionParents
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> ActionParents<'a> {
    /// Returns an iterator over parent action names.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + 'a {
        self.node.children().filter_map(Name::cast)
    }
}

/// Action `appliesTo` clause specifying principal, resource, and context types.
///
/// ```cedarschema
/// action read appliesTo {
///     principal: [User, Admin],
///     resource: [Document],
///     context: { ip?: ipaddr },
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AppliesTo<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for AppliesTo<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::AppliesTo
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

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

/// Principal types in an `appliesTo` clause.
///
/// ```cedarschema
/// principal: [User, Admin]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PrincipalTypes<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for PrincipalTypes<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::PrincipalTypes
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> PrincipalTypes<'a> {
    /// Returns the list of principal entity types.
    #[must_use]
    pub fn type_list(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

/// Resource types in an `appliesTo` clause.
///
/// ```cedarschema
/// resource: [Document, Folder]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ResourceTypes<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for ResourceTypes<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::ResourceTypes
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> ResourceTypes<'a> {
    /// Returns the list of resource entity types.
    #[must_use]
    pub fn type_list(&self) -> Option<TypeList<'a>> {
        self.node.children().find_map(TypeList::cast)
    }
}

/// Context type in an `appliesTo` clause.
///
/// ```cedarschema
/// context: { ip?: ipaddr, authenticated: Bool }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ContextType<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for ContextType<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::ContextType
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> ContextType<'a> {
    /// Returns the context type expression (typically a record type).
    #[must_use]
    pub fn type_expr(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

/// Action attributes block.
///
/// ```cedarschema
/// action read attributes { priority: 1 };
/// //          ^^^^^^^^^^^^^^^^^^^^^^^^^ ActionAttributes
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ActionAttributes<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for ActionAttributes<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::ActionAttributes
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> ActionAttributes<'a> {
    /// Returns an iterator over attribute entries.
    pub fn entries(&self) -> impl Iterator<Item = AttributeEntry<'a>> + 'a {
        self.node.children().filter_map(AttributeEntry::cast)
    }
}

/// Key-value entry in action attributes.
///
/// ```cedarschema
/// action read attributes { priority: 1, enabled: true };
/// //                       ^^^^^^^^^^^  ^^^^^^^^^^^^^ entries
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AttributeEntry<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for AttributeEntry<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::AttributeEntry
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

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

/// Common type declaration (type alias).
///
/// ```cedarschema
/// type EmailAddress = String;
/// type UserInfo = { name: String, email?: EmailAddress };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TypeDecl<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for TypeDecl<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::CommonTypeDeclaration
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> TypeDecl<'a> {
    /// Returns an iterator over annotations on this type declaration.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + 'a {
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

/// List of type references.
///
/// ```cedarschema
/// entity User in [Group, Team, Organization];
/// //             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ TypeList
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TypeList<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for TypeList<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::TypeList
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> TypeList<'a> {
    /// Returns an iterator over the type names in this list.
    pub fn types(&self) -> impl Iterator<Item = Name<'a>> + 'a {
        self.node.children().filter_map(Name::cast)
    }
}

/// Attribute declaration in a record type.
///
/// ```cedarschema
/// entity User {
///     name: String,
///     email?: String,  // optional attribute
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AttributeDecl<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for AttributeDecl<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::AttributeDeclaration
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> AttributeDecl<'a> {
    /// Returns an iterator over annotations on this attribute.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + 'a {
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
    pub fn attr_type(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

/// Attribute key (name) which can be an identifier or a quoted string.
///
/// ```cedarschema
/// entity User {
///     name: String,           // identifier key
///     "special-attr": Long,   // string key (for reserved words or special chars)
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub enum AttrKey<'a> {
    Identifier(IdentifierToken<'a>),
    String(StringToken<'a>),
}

impl<'a> AttrKey<'a> {
    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        match node.value() {
            SchemaSyntax::Identifier => IdentifierToken::cast(node).map(Self::Identifier),
            SchemaSyntax::String => StringToken::cast(node).map(Self::String),
            _ => None,
        }
    }

    #[must_use]
    pub fn syntax(&self) -> &SchemaNode<'a> {
        match self {
            Self::Identifier(token) => token.syntax(),
            Self::String(token) => token.syntax(),
        }
    }
}

/// Record type definition with named attributes.
///
/// ```cedarschema
/// type UserInfo = {
///     name: String,
///     email?: String,
///     age: Long,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RecordType<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for RecordType<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::RecordType
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> RecordType<'a> {
    /// Returns an iterator over attribute declarations in this record type.
    pub fn attrs(&self) -> impl Iterator<Item = AttributeDecl<'a>> + 'a {
        self.node.children().filter_map(AttributeDecl::cast)
    }
}
