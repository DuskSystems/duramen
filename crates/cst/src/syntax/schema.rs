use core::fmt;

use duramen_lexer::TokenKind;

/// Syntax kinds for Cedar schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchemaSyntax {
    /// String: `"hello"`
    String,
    /// Identifier: `name`
    Identifier,

    /// `action`
    Action,
    /// `appliesTo`
    AppliesTo,
    /// `attributes`
    Attributes,
    /// `Bool`
    Bool,
    /// `context`
    Context,
    /// `entity`
    Entity,
    /// `enum`
    Enum,
    /// `false`
    False,
    /// `in`
    In,
    /// `Long`
    Long,
    /// `namespace`
    Namespace,
    /// `principal`
    Principal,
    /// `resource`
    Resource,
    /// `Set`
    Set,
    /// `String` (type keyword)
    StringType,
    /// `tags`
    Tags,
    /// `true`
    True,
    /// `type`
    Type,

    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,

    /// `@`
    At,
    /// `:`
    Colon,
    /// `::`
    Colon2,
    /// `,`
    Comma,
    /// `?`
    Question,
    /// `;`
    Semicolon,

    /// `=`
    Eq,
    /// `>`
    Gt,
    /// `<`
    Lt,

    /// Comment: `// ...`
    Comment,
    /// Whitespace
    Whitespace,

    /// End of file
    Eof,
    /// Unknown token
    Unknown,
    /// Error node
    Error,

    /// Root node.
    ///
    /// ```cedarschema
    /// namespace MyApp { entity User; }
    /// ```
    Schema,

    /// Namespace declaration.
    ///
    /// ```cedarschema
    /// namespace MyApp { entity User; }
    /// ```
    NamespaceDeclaration,

    /// Entity declaration.
    ///
    /// ```cedarschema
    /// entity User in [Group] = { "name": String };
    /// ```
    EntityDeclaration,

    /// Action declaration.
    ///
    /// ```cedarschema
    /// action view appliesTo { principal: [User], resource: [Doc] };
    /// ```
    ActionDeclaration,

    /// Type declaration.
    ///
    /// ```cedarschema
    /// type Email = String;
    /// ```
    TypeDeclaration,

    /// Annotation.
    ///
    /// ```cedarschema
    /// @doc("description")
    /// ```
    Annotation,

    /// Entity parents.
    ///
    /// ```cedarschema
    /// in [Group]
    /// ```
    EntityParents,

    /// Entity attributes.
    ///
    /// ```cedarschema
    /// = { "name": String }
    /// ```
    EntityAttributes,

    /// Entity tags.
    ///
    /// ```cedarschema
    /// tags String
    /// ```
    EntityTags,

    /// Applies-to clause.
    ///
    /// ```cedarschema
    /// appliesTo { principal: [User], resource: [Doc] }
    /// ```
    AppliesToClause,

    /// Principal types.
    ///
    /// ```cedarschema
    /// principal: [User, Admin]
    /// ```
    PrincipalTypes,

    /// Resource types.
    ///
    /// ```cedarschema
    /// resource: [Document]
    /// ```
    ResourceTypes,

    /// Context type.
    ///
    /// ```cedarschema
    /// context: { "ip": String }
    /// ```
    ContextType,

    /// Action parents.
    ///
    /// ```cedarschema
    /// in [review]
    /// ```
    ActionParents,

    /// Action attributes.
    ActionAttributes,

    /// Attribute entry.
    AttributeEntry,

    /// Type expression.
    TypeExpr,

    /// Set type.
    ///
    /// ```cedarschema
    /// Set<User>
    /// ```
    SetType,

    /// Record type.
    ///
    /// ```cedarschema
    /// { "name": String }
    /// ```
    RecordType,

    /// Entity type.
    ///
    /// ```cedarschema
    /// Organization::Employee
    /// ```
    EntityType,

    /// Enum type.
    ///
    /// ```cedarschema
    /// enum ["active", "inactive"]
    /// ```
    EnumType,

    /// Qualified name.
    ///
    /// ```cedarschema
    /// Organization::Employee
    /// ```
    Name,

    /// Attribute declaration.
    ///
    /// ```cedarschema
    /// "phone"?: String
    /// ```
    AttributeDeclaration,

    /// Enum variant.
    ///
    /// ```cedarschema
    /// "active"
    /// ```
    EnumVariant,

    /// Type list.
    ///
    /// ```cedarschema
    /// [User, Admin]
    /// ```
    TypeList,
}

impl SchemaSyntax {
    /// Checks if this is a trivial token.
    #[must_use]
    pub const fn is_trivial(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    /// Checks if this is an identifier.
    #[must_use]
    pub const fn is_identifier(self) -> bool {
        matches!(self, Self::Identifier)
    }

    /// Checks if this is a literal token.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(self, Self::True | Self::False | Self::String)
    }

    /// Checks if this is a keyword.
    #[must_use]
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::Action
                | Self::AppliesTo
                | Self::Attributes
                | Self::Bool
                | Self::Context
                | Self::Entity
                | Self::Enum
                | Self::False
                | Self::In
                | Self::Long
                | Self::Namespace
                | Self::Principal
                | Self::Resource
                | Self::Set
                | Self::StringType
                | Self::Tags
                | Self::True
                | Self::Type
        )
    }

    /// Checks if this keyword can appear in a name position.
    #[must_use]
    pub const fn is_name_keyword(self) -> bool {
        matches!(
            self,
            Self::Bool
                | Self::Long
                | Self::StringType
                | Self::Set
                | Self::True
                | Self::False
                | Self::In
        )
    }

    /// Checks if this is a declaration keyword.
    #[must_use]
    pub const fn is_declaration_keyword(self) -> bool {
        matches!(
            self,
            Self::Entity | Self::Action | Self::Type | Self::Namespace
        )
    }

    /// Checks if this is a token.
    #[must_use]
    pub const fn is_token(self) -> bool {
        matches!(
            self,
            Self::String
                | Self::Identifier
                | Self::Action
                | Self::AppliesTo
                | Self::Attributes
                | Self::Bool
                | Self::Context
                | Self::Entity
                | Self::Enum
                | Self::False
                | Self::In
                | Self::Long
                | Self::Namespace
                | Self::Principal
                | Self::Resource
                | Self::Set
                | Self::StringType
                | Self::Tags
                | Self::True
                | Self::Type
                | Self::OpenParen
                | Self::CloseParen
                | Self::OpenBrace
                | Self::CloseBrace
                | Self::OpenBracket
                | Self::CloseBracket
                | Self::At
                | Self::Colon
                | Self::Colon2
                | Self::Comma
                | Self::Question
                | Self::Semicolon
                | Self::Eq
                | Self::Gt
                | Self::Lt
                | Self::Comment
                | Self::Whitespace
                | Self::Eof
                | Self::Unknown
        )
    }

    /// Checks if this is a node.
    #[must_use]
    pub const fn is_node(self) -> bool {
        !self.is_token()
    }
}

impl fmt::Display for SchemaSyntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::String => "string",
            Self::Identifier => "identifier",
            Self::Action => "action",
            Self::AppliesTo => "appliesTo",
            Self::Attributes => "attributes",
            Self::Bool => "Bool",
            Self::Context => "context",
            Self::Entity => "entity",
            Self::Enum => "enum",
            Self::False => "false",
            Self::In => "in",
            Self::Long => "Long",
            Self::Namespace => "namespace",
            Self::Principal => "principal",
            Self::Resource => "resource",
            Self::Set => "Set",
            Self::StringType => "String",
            Self::Tags => "tags",
            Self::True => "true",
            Self::Type => "type",
            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
            Self::At => "@",
            Self::Colon => ":",
            Self::Colon2 => "::",
            Self::Comma => ",",
            Self::Question => "?",
            Self::Semicolon => ";",
            Self::Eq => "=",
            Self::Gt => ">",
            Self::Lt => "<",
            Self::Comment => "comment",
            Self::Whitespace => "whitespace",
            Self::Eof => "end of file",
            Self::Unknown => "unknown",
            Self::Error => "error",
            Self::Schema => "schema",
            Self::NamespaceDeclaration => "namespace declaration",
            Self::EntityDeclaration => "entity declaration",
            Self::ActionDeclaration => "action declaration",
            Self::TypeDeclaration => "type declaration",
            Self::Annotation => "annotation",
            Self::EntityParents => "entity parents",
            Self::EntityAttributes => "entity attributes",
            Self::EntityTags => "entity tags",
            Self::AppliesToClause => "applies-to clause",
            Self::PrincipalTypes => "principal types",
            Self::ResourceTypes => "resource types",
            Self::ContextType => "context type",
            Self::ActionParents => "action parents",
            Self::ActionAttributes => "action attributes",
            Self::AttributeEntry => "attribute entry",
            Self::TypeExpr => "type expression",
            Self::SetType => "set type",
            Self::RecordType => "record type",
            Self::EntityType => "entity type",
            Self::EnumType => "enum type",
            Self::Name => "name",
            Self::AttributeDeclaration => "attribute declaration",
            Self::EnumVariant => "enum variant",
            Self::TypeList => "type list",
        };

        f.write_str(text)
    }
}

impl From<TokenKind> for SchemaSyntax {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::String | TokenKind::StringUnterminated => Self::String,

            TokenKind::Identifier
            | TokenKind::Permit
            | TokenKind::Forbid
            | TokenKind::When
            | TokenKind::Unless
            | TokenKind::Like
            | TokenKind::Has
            | TokenKind::Is
            | TokenKind::If
            | TokenKind::Then
            | TokenKind::Else => Self::Identifier,

            TokenKind::Action => Self::Action,
            TokenKind::AppliesTo => Self::AppliesTo,
            TokenKind::Attributes => Self::Attributes,
            TokenKind::Bool => Self::Bool,
            TokenKind::Context => Self::Context,
            TokenKind::Entity => Self::Entity,
            TokenKind::Enum => Self::Enum,
            TokenKind::False => Self::False,
            TokenKind::In => Self::In,
            TokenKind::Long => Self::Long,
            TokenKind::Namespace => Self::Namespace,
            TokenKind::Principal => Self::Principal,
            TokenKind::Resource => Self::Resource,
            TokenKind::Set => Self::Set,
            TokenKind::StringType => Self::StringType,
            TokenKind::Tags => Self::Tags,
            TokenKind::True => Self::True,
            TokenKind::Type => Self::Type,

            TokenKind::OpenParen => Self::OpenParen,
            TokenKind::CloseParen => Self::CloseParen,
            TokenKind::OpenBrace => Self::OpenBrace,
            TokenKind::CloseBrace => Self::CloseBrace,
            TokenKind::OpenBracket => Self::OpenBracket,
            TokenKind::CloseBracket => Self::CloseBracket,

            TokenKind::At => Self::At,
            TokenKind::Colon => Self::Colon,
            TokenKind::Colon2 => Self::Colon2,
            TokenKind::Comma => Self::Comma,
            TokenKind::Question => Self::Question,
            TokenKind::Semicolon => Self::Semicolon,

            TokenKind::Eq => Self::Eq,
            TokenKind::Gt => Self::Gt,
            TokenKind::Lt => Self::Lt,

            TokenKind::Comment => Self::Comment,
            TokenKind::Whitespace => Self::Whitespace,

            TokenKind::Integer
            | TokenKind::Dot
            | TokenKind::Amp2
            | TokenKind::Bang
            | TokenKind::BangEq
            | TokenKind::Eq2
            | TokenKind::GtEq
            | TokenKind::LtEq
            | TokenKind::Minus
            | TokenKind::Percent
            | TokenKind::Pipe2
            | TokenKind::Plus
            | TokenKind::Slash
            | TokenKind::Star
            | TokenKind::Unknown => Self::Unknown,
        }
    }
}
