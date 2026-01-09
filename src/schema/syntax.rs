//! Cedar Schema Syntax.
//!
//! # References
//!
//! - [Grammar](https://docs.cedarpolicy.com/schema/human-readable-schema-grammar.html)
//! - [Syntax](https://docs.cedarpolicy.com/schema/human-readable-schema.html)
//! - [LALRPOP](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-validator/src/cedar_schema/grammar.lalrpop)

/// Syntax kinds for Cedar schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchemaSyntax {
    /// Integer literal.
    Integer,
    /// String literal.
    String,
    /// Identifier.
    Identifier,

    /// Keyword `namespace`.
    NamespaceKeyword,
    /// Keyword `entity`.
    EntityKeyword,
    /// Keyword `action`.
    ActionKeyword,
    /// Keyword `type`.
    TypeKeyword,

    /// Keyword `in`.
    InKeyword,
    /// Keyword `tags`.
    TagsKeyword,

    /// Keyword `appliesTo`.
    AppliesToKeyword,
    /// Keyword `principal`.
    PrincipalKeyword,
    /// Keyword `resource`.
    ResourceKeyword,
    /// Keyword `context`.
    ContextKeyword,
    /// Keyword `attributes`.
    AttributesKeyword,

    /// Keyword `Bool`.
    BoolKeyword,
    /// Keyword `Long`.
    LongKeyword,
    /// Keyword `String`.
    StringKeyword,
    /// Keyword `Set`.
    SetKeyword,

    /// Keyword `enum`.
    EnumKeyword,
    /// Keyword `true`.
    TrueKeyword,
    /// Keyword `false`.
    FalseKeyword,

    /// `(` symbol.
    OpenParenthesis,
    /// `)` symbol.
    CloseParenthesis,
    /// `{` symbol.
    OpenBrace,
    /// `}` symbol.
    CloseBrace,
    /// `[` symbol.
    OpenBracket,
    /// `]` symbol.
    CloseBracket,

    /// `,` symbol.
    Comma,
    /// `;` symbol.
    Semicolon,
    /// `:` symbol.
    Colon,
    /// `::` symbol.
    Colon2,
    /// `@` symbol.
    At,
    /// `.` symbol.
    Dot,
    /// `?` symbol.
    Question,

    /// `<` symbol.
    LessThan,
    /// `>` symbol.
    GreaterThan,
    /// `=` symbol.
    Equal,
    /// `|` symbol.
    Pipe,
    /// `/` symbol.
    Slash,

    /// Comment.
    Comment,
    /// Whitespace.
    Whitespace,

    /// End of file.
    Eof,
    /// Unknown token.
    Unknown,

    /// Root node containing namespaces and declarations.
    ///
    /// ```cedarschema
    /// namespace Acme {
    ///     entity User;
    ///     entity Document;
    ///     action read appliesTo { principal: User, resource: Document };
    /// }
    /// ```
    Schema,

    /// Namespace declaration containing entity, action, and type definitions.
    ///
    /// ```cedarschema
    /// namespace Acme::Corp {
    ///     entity User;
    ///     entity Document;
    /// }
    /// ```
    Namespace,

    /// Entity type declaration.
    ///
    /// ```cedarschema
    /// entity User in [Group] = {
    ///     name: String,
    ///     email: String,
    ///     active?: Bool,
    /// };
    /// ```
    EntityDeclaration,

    /// Action declaration.
    ///
    /// ```cedarschema
    /// action read, write in [access] appliesTo {
    ///     principal: [User, Admin],
    ///     resource: Document,
    ///     context: { ip: String, timestamp: Long },
    /// };
    /// ```
    ActionDeclaration,

    /// Common type alias declaration.
    ///
    /// ```cedarschema
    /// type Email = String;
    /// type Metadata = { created: Long, modified: Long };
    /// ```
    CommonTypeDeclaration,

    // Entity components
    /// Entity parent types (`in [...]` clause).
    ///
    /// ```cedarschema
    /// entity User in [Group, Team];
    /// ```
    EntityParents,

    /// Entity attributes block.
    ///
    /// ```cedarschema
    /// entity User = {
    ///     name: String,
    ///     email?: String,
    /// };
    /// ```
    EntityAttributes,

    /// Entity tags specification.
    ///
    /// ```cedarschema
    /// entity Document tags Set<String>;
    /// ```
    EntityTags,

    // Action components
    /// Action applies-to clause specifying principal, resource, and context types.
    ///
    /// ```cedarschema
    /// action read appliesTo {
    ///     principal: [User, Admin],
    ///     resource: Document,
    ///     context: { ip: String },
    /// };
    /// ```
    AppliesTo,

    /// Principal types in an action declaration.
    ///
    /// ```cedarschema
    /// action read appliesTo { principal: [User, Admin], resource: Document };
    /// ```
    PrincipalTypes,

    /// Resource types in an action declaration.
    ///
    /// ```cedarschema
    /// action read appliesTo { principal: User, resource: [Document, Folder] };
    /// ```
    ResourceTypes,

    /// Context type in an action declaration.
    ///
    /// ```cedarschema
    /// action read appliesTo {
    ///     principal: User,
    ///     resource: Document,
    ///     context: { ip: String, timestamp: Long },
    /// };
    /// ```
    ContextType,

    /// Action parent groups.
    ///
    /// ```cedarschema
    /// action read in [readAccess, writeAccess] appliesTo {
    ///     principal: User,
    ///     resource: Document,
    /// };
    /// ```
    ActionParents,

    /// Action attributes block.
    ///
    /// ```cedarschema
    /// action read appliesTo { principal: User, resource: Document }
    ///     attributes { cost: 1, readonly: true };
    /// ```
    ActionAttributes,

    /// Key-value pair in action attributes.
    ///
    /// ```cedarschema
    /// action read appliesTo { principal: User, resource: Document }
    ///     attributes { cost: 1, readonly: true, name: "read" };
    /// ```
    AttributeEntry,

    // Type expressions
    /// Type expression wrapper.
    TypeExpression,

    /// Set type with element type.
    ///
    /// ```cedarschema
    /// type Tags = Set<String>;
    /// type Users = Set<User>;
    /// ```
    SetType,

    /// Record type with attribute declarations.
    ///
    /// ```cedarschema
    /// type Metadata = { name: String, age: Long, email?: String };
    /// ```
    RecordType,

    /// Entity type reference.
    ///
    /// ```cedarschema
    /// entity User;
    /// namespace Acme { entity Document; }
    /// type Admin = User;
    /// type Doc = Acme::Document;
    /// ```
    EntityType,

    /// Enum type definition.
    ///
    /// ```cedarschema
    /// entity Status enum ["active", "inactive", "pending"];
    /// ```
    EnumType,

    // Supporting structures
    /// Qualified name (`Foo::Bar::Baz`).
    Name,

    /// Single segment of a qualified name.
    PathSegment,

    /// Attribute declaration in a record or entity.
    ///
    /// ```cedarschema
    /// entity User = { name: String, email?: String };
    /// ```
    AttributeDeclaration,

    /// Enum variant string in an enum type.
    ///
    /// ```cedarschema
    /// entity Status enum ["active", "inactive"];
    /// ```
    EnumVariant,

    /// List of type references.
    ///
    /// ```cedarschema
    /// entity Admin in [User, Guest];
    /// ```
    TypeList,

    /// Annotation on a declaration.
    ///
    /// ```cedarschema
    /// @doc("User account entity")
    /// entity User;
    /// ```
    Annotation,

    /// Error recovery node wrapping invalid tokens during parsing.
    Error,
}

impl SchemaSyntax {
    /// Returns the keyword kind for the given text, if any.
    #[must_use]
    pub fn from_keyword(value: &str) -> Option<Self> {
        match value {
            "namespace" => Some(Self::NamespaceKeyword),
            "entity" => Some(Self::EntityKeyword),
            "action" => Some(Self::ActionKeyword),
            "type" => Some(Self::TypeKeyword),
            "in" => Some(Self::InKeyword),
            "tags" => Some(Self::TagsKeyword),
            "appliesTo" => Some(Self::AppliesToKeyword),
            "principal" => Some(Self::PrincipalKeyword),
            "resource" => Some(Self::ResourceKeyword),
            "context" => Some(Self::ContextKeyword),
            "attributes" => Some(Self::AttributesKeyword),
            "Bool" => Some(Self::BoolKeyword),
            "Long" => Some(Self::LongKeyword),
            "String" => Some(Self::StringKeyword),
            "Set" => Some(Self::SetKeyword),
            "enum" => Some(Self::EnumKeyword),
            "true" => Some(Self::TrueKeyword),
            "false" => Some(Self::FalseKeyword),
            _ => None,
        }
    }
}
