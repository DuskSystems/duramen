//! Cedar Schema Syntax.
//!
//! # References
//!
//! - [Grammar](https://docs.cedarpolicy.com/schema/human-readable-schema-grammar.html)
//! - [Syntax](https://docs.cedarpolicy.com/schema/human-readable-schema.html)
//! - [LALRPOP](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-validator/src/cedar_schema/grammar.lalrpop)

/// Token kinds for Cedar schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchemaTokenKind {
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
    /// Keyword `enum`.
    EnumKeyword,
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
}

impl SchemaTokenKind {
    /// Returns the keyword kind for the given text, if any.
    #[must_use]
    pub fn from_keyword(value: &str) -> Option<Self> {
        match value {
            "namespace" => Some(Self::NamespaceKeyword),
            "entity" => Some(Self::EntityKeyword),
            "action" => Some(Self::ActionKeyword),
            "type" => Some(Self::TypeKeyword),
            "in" => Some(Self::InKeyword),
            "enum" => Some(Self::EnumKeyword),
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
            _ => None,
        }
    }
}
