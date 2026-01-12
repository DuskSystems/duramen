use super::{ActionDecl, AstNode, EntityDecl, Name, RecordType, SchemaNode, TypeDecl};
use crate::schema::SchemaSyntax;

/// A set type expression representing a collection of values.
///
/// ```cedarschema
/// entity Document {
///     readers: Set<User>,
/// //           ^^^^^^^^^^ SetType with element User
///     tags: Set<String>,
/// //        ^^^^^^^^^^^ SetType with element String
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SetType<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for SetType<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::SetType
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> SetType<'a> {
    /// Returns the element type of this set.
    ///
    /// ```cedarschema
    /// entity Document {
    ///     readers: Set<User>,
    /// //               ^^^^ element type
    /// };
    /// ```
    #[must_use]
    pub fn element(&self) -> Option<TypeExpr<'a>> {
        self.node.children().find_map(TypeExpr::cast)
    }
}

/// A reference to an entity type used in type expressions.
///
/// Entity type references are used when a type expression needs to refer to
/// an entity rather than a primitive type. They include the `entity` keyword
/// wrapper when used in attribute types.
///
/// ```cedarschema
/// entity Document {
///     owner: User,
/// //         ^^^^ EntityTypeRef (implicit entity reference)
///     reviewers: Set<User>,
/// //                 ^^^^ element could be EntityTypeRef
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EntityTypeRef<'a> {
    node: SchemaNode<'a>,
}

impl<'a> AstNode<'a> for EntityTypeRef<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        kind == SchemaSyntax::EntityType
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        &self.node
    }
}

impl<'a> EntityTypeRef<'a> {
    /// Returns the qualified name of the referenced entity type.
    ///
    /// ```cedarschema
    /// entity Document {
    ///     owner: Org::User,
    /// //         ^^^^^^^^^ name (qualified)
    /// };
    /// ```
    #[must_use]
    pub fn name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }
}

/// A declaration within a namespace or at the schema root.
///
/// This enum represents the three kinds of declarations that can appear
/// in a Cedar schema: entity types, actions, and common type aliases.
///
/// ```cedarschema
/// namespace App {
///     entity User { };              // Decl::Entity
///     type Email = String;          // Decl::Type
///     action view appliesTo { ... } // Decl::Action
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Decl<'a> {
    /// An entity type declaration.
    ///
    /// ```cedarschema
    /// entity User { name: String };
    /// ```
    Entity(EntityDecl<'a>),

    /// An action declaration.
    ///
    /// ```cedarschema
    /// action view appliesTo { principal: User, resource: Document };
    /// ```
    Action(ActionDecl<'a>),

    /// A common type alias declaration.
    ///
    /// ```cedarschema
    /// type Email = String;
    /// ```
    Type(TypeDecl<'a>),
}

impl<'a> AstNode<'a> for Decl<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        matches!(
            kind,
            SchemaSyntax::EntityDeclaration
                | SchemaSyntax::ActionDeclaration
                | SchemaSyntax::CommonTypeDeclaration
        )
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        match node.value() {
            SchemaSyntax::EntityDeclaration => EntityDecl::cast(node).map(Self::Entity),
            SchemaSyntax::ActionDeclaration => ActionDecl::cast(node).map(Self::Action),
            SchemaSyntax::CommonTypeDeclaration => TypeDecl::cast(node).map(Self::Type),
            _ => None,
        }
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        match self {
            Self::Entity(inner) => inner.syntax(),
            Self::Action(inner) => inner.syntax(),
            Self::Type(inner) => inner.syntax(),
        }
    }
}

/// A type expression that can appear in attribute declarations and type aliases.
///
/// Cedar schemas support several kinds of type expressions:
/// - **Name**: A type name reference (primitive types like `String`, `Long`, `Bool`,
///   or type aliases)
/// - **Set**: A set type like `Set<String>` or `Set<User>`
/// - **Record**: An inline record type with named attributes
/// - **Entity**: An entity type reference
///
/// ```cedarschema
/// type Metadata = {
///     name: String,              // TypeExpr::Name
///     count: Long,               // TypeExpr::Name
///     tags: Set<String>,         // TypeExpr::Set
///     nested: { x: Long },       // TypeExpr::Record
///     owner: User,               // TypeExpr::Entity or TypeExpr::Name
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub enum TypeExpr<'a> {
    /// A type name reference (primitive types or type aliases).
    ///
    /// ```cedarschema
    /// entity User {
    ///     name: String,
    /// //        ^^^^^^ TypeExpr::Name
    ///     age: Long,
    /// //       ^^^^ TypeExpr::Name
    /// };
    /// ```
    Name(Name<'a>),

    /// A set type containing elements of another type.
    ///
    /// ```cedarschema
    /// entity Document {
    ///     tags: Set<String>,
    /// //        ^^^^^^^^^^^ TypeExpr::Set
    /// };
    /// ```
    Set(SetType<'a>),

    /// An inline record type with named attributes.
    ///
    /// ```cedarschema
    /// entity User {
    ///     address: { street: String, city: String },
    /// //           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ TypeExpr::Record
    /// };
    /// ```
    Record(RecordType<'a>),

    /// An entity type reference.
    ///
    /// ```cedarschema
    /// entity Document {
    ///     owner: User,
    /// //         ^^^^ TypeExpr::Entity
    /// };
    /// ```
    Entity(EntityTypeRef<'a>),
}

impl<'a> AstNode<'a> for TypeExpr<'a> {
    fn can_cast(kind: SchemaSyntax) -> bool {
        matches!(
            kind,
            SchemaSyntax::Name
                | SchemaSyntax::SetType
                | SchemaSyntax::RecordType
                | SchemaSyntax::EntityType
        )
    }

    fn cast(node: SchemaNode<'a>) -> Option<Self> {
        match node.value() {
            SchemaSyntax::Name => Name::cast(node).map(Self::Name),
            SchemaSyntax::SetType => SetType::cast(node).map(Self::Set),
            SchemaSyntax::RecordType => RecordType::cast(node).map(Self::Record),
            SchemaSyntax::EntityType => EntityTypeRef::cast(node).map(Self::Entity),
            _ => None,
        }
    }

    fn syntax(&self) -> &SchemaNode<'a> {
        match self {
            Self::Name(inner) => inner.syntax(),
            Self::Set(inner) => inner.syntax(),
            Self::Record(inner) => inner.syntax(),
            Self::Entity(inner) => inner.syntax(),
        }
    }
}
