use core::fmt;

use duramen_lexer::TokenKind;

/// Token syntax kinds (leaves).
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Token {
    /// Identifier: `name`.
    Identifier,
    /// Integer literal: `123`.
    Integer,
    /// String literal: `"hello"`.
    String,

    /// Keyword: `action`.
    ActionKeyword,
    /// Keyword: `appliesTo`.
    AppliesToKeyword,
    /// Keyword: `attributes`.
    AttributesKeyword,
    /// Keyword: `Bool`.
    BoolKeyword,
    /// Keyword: `context`.
    ContextKeyword,
    /// Keyword: `else`.
    ElseKeyword,
    /// Keyword: `entity`.
    EntityKeyword,
    /// Keyword: `enum`.
    EnumKeyword,
    /// Keyword: `false`.
    FalseKeyword,
    /// Keyword: `forbid`.
    ForbidKeyword,
    /// Keyword: `has`.
    HasKeyword,
    /// Keyword: `if`.
    IfKeyword,
    /// Keyword: `in`.
    InKeyword,
    /// Keyword: `is`.
    IsKeyword,
    /// Keyword: `like`.
    LikeKeyword,
    /// Keyword: `Long`.
    LongKeyword,
    /// Keyword: `namespace`.
    NamespaceKeyword,
    /// Keyword: `permit`.
    PermitKeyword,
    /// Keyword: `principal`.
    PrincipalKeyword,
    /// Keyword: `resource`.
    ResourceKeyword,
    /// Keyword: `Set`.
    SetKeyword,
    /// Keyword: `String`.
    StringKeyword,
    /// Keyword: `tags`.
    TagsKeyword,
    /// Keyword: `then`.
    ThenKeyword,
    /// Keyword: `true`.
    TrueKeyword,
    /// Keyword: `type`.
    TypeKeyword,
    /// Keyword: `unless`.
    UnlessKeyword,
    /// Keyword: `when`.
    WhenKeyword,

    /// Open parenthesis: `(`.
    OpenParenthesis,
    /// Close parenthesis: `)`.
    CloseParenthesis,
    /// Open brace: `{`.
    OpenBrace,
    /// Close brace: `}`.
    CloseBrace,
    /// Open bracket: `[`.
    OpenBracket,
    /// Close bracket: `]`.
    CloseBracket,

    /// At sign: `@`.
    At,
    /// Colon: `:`.
    Colon,
    /// Path separator: `::`.
    PathSeparator,
    /// Comma: `,`.
    Comma,
    /// Dot: `.`.
    Dot,
    /// Question mark: `?`.
    QuestionMark,
    /// Semicolon: `;`.
    Semicolon,

    /// Logical and: `&&`.
    And,
    /// Assign: `=`.
    Assign,
    /// Equal: `==`.
    Equal,
    /// Greater than: `>`.
    Greater,
    /// Greater than or equal: `>=`.
    GreaterEqual,
    /// Less than: `<`.
    Less,
    /// Less than or equal: `<=`.
    LessEqual,
    /// Subtract: `-`.
    Subtract,
    /// Logical not: `!`.
    Not,
    /// Not equal: `!=`.
    NotEqual,
    /// Logical or: `||`.
    Or,
    /// Add: `+`.
    Add,
    /// Multiply: `*`.
    Multiply,

    /// Line comment: `// ...`.
    Comment,
    /// Newline.
    Newline,
    /// Whitespace.
    Whitespace,

    /// Unknown token from the lexer.
    Unknown,
}

impl Token {
    /// Checks if this is a whitespace token.
    #[must_use]
    pub const fn is_whitespace(self) -> bool {
        matches!(self, Self::Whitespace)
    }

    /// Checks if this is a newline token.
    #[must_use]
    pub const fn is_newline(self) -> bool {
        matches!(self, Self::Newline)
    }

    /// Checks if this is a comment token.
    #[must_use]
    pub const fn is_comment(self) -> bool {
        matches!(self, Self::Comment)
    }

    /// Checks if this is a keyword token.
    #[must_use]
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::ActionKeyword
                | Self::AppliesToKeyword
                | Self::AttributesKeyword
                | Self::BoolKeyword
                | Self::ContextKeyword
                | Self::ElseKeyword
                | Self::EntityKeyword
                | Self::EnumKeyword
                | Self::FalseKeyword
                | Self::ForbidKeyword
                | Self::HasKeyword
                | Self::IfKeyword
                | Self::InKeyword
                | Self::IsKeyword
                | Self::LikeKeyword
                | Self::LongKeyword
                | Self::NamespaceKeyword
                | Self::PermitKeyword
                | Self::PrincipalKeyword
                | Self::ResourceKeyword
                | Self::SetKeyword
                | Self::StringKeyword
                | Self::TagsKeyword
                | Self::ThenKeyword
                | Self::TrueKeyword
                | Self::TypeKeyword
                | Self::UnlessKeyword
                | Self::WhenKeyword
        )
    }

    /// Checks if this can be used as an identifier.
    #[must_use]
    pub const fn is_identifier(self) -> bool {
        self.is_keyword() || matches!(self, Self::Identifier)
    }

    /// Checks if this is a reserved keyword in policy context.
    #[must_use]
    pub const fn is_reserved(self) -> bool {
        matches!(
            self,
            Self::IfKeyword
                | Self::TrueKeyword
                | Self::FalseKeyword
                | Self::ThenKeyword
                | Self::ElseKeyword
                | Self::HasKeyword
                | Self::LikeKeyword
                | Self::IsKeyword
                | Self::InKeyword
        )
    }

    /// Checks if this is a literal token.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::FalseKeyword | Self::Integer | Self::String | Self::TrueKeyword
        )
    }

    /// Checks if this is a string token.
    #[must_use]
    pub const fn is_string(self) -> bool {
        matches!(self, Self::String)
    }
}

impl From<TokenKind> for Token {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Identifier => Self::Identifier,
            TokenKind::Integer => Self::Integer,
            TokenKind::String => Self::String,

            TokenKind::ActionKeyword => Self::ActionKeyword,
            TokenKind::AppliesToKeyword => Self::AppliesToKeyword,
            TokenKind::AttributesKeyword => Self::AttributesKeyword,
            TokenKind::BoolKeyword => Self::BoolKeyword,
            TokenKind::ContextKeyword => Self::ContextKeyword,
            TokenKind::ElseKeyword => Self::ElseKeyword,
            TokenKind::EntityKeyword => Self::EntityKeyword,
            TokenKind::EnumKeyword => Self::EnumKeyword,
            TokenKind::FalseKeyword => Self::FalseKeyword,
            TokenKind::ForbidKeyword => Self::ForbidKeyword,
            TokenKind::HasKeyword => Self::HasKeyword,
            TokenKind::IfKeyword => Self::IfKeyword,
            TokenKind::InKeyword => Self::InKeyword,
            TokenKind::IsKeyword => Self::IsKeyword,
            TokenKind::LikeKeyword => Self::LikeKeyword,
            TokenKind::LongKeyword => Self::LongKeyword,
            TokenKind::NamespaceKeyword => Self::NamespaceKeyword,
            TokenKind::PermitKeyword => Self::PermitKeyword,
            TokenKind::PrincipalKeyword => Self::PrincipalKeyword,
            TokenKind::ResourceKeyword => Self::ResourceKeyword,
            TokenKind::SetKeyword => Self::SetKeyword,
            TokenKind::StringKeyword => Self::StringKeyword,
            TokenKind::TagsKeyword => Self::TagsKeyword,
            TokenKind::ThenKeyword => Self::ThenKeyword,
            TokenKind::TrueKeyword => Self::TrueKeyword,
            TokenKind::TypeKeyword => Self::TypeKeyword,
            TokenKind::UnlessKeyword => Self::UnlessKeyword,
            TokenKind::WhenKeyword => Self::WhenKeyword,

            TokenKind::OpenParenthesis => Self::OpenParenthesis,
            TokenKind::CloseParenthesis => Self::CloseParenthesis,
            TokenKind::OpenBrace => Self::OpenBrace,
            TokenKind::CloseBrace => Self::CloseBrace,
            TokenKind::OpenBracket => Self::OpenBracket,
            TokenKind::CloseBracket => Self::CloseBracket,

            TokenKind::At => Self::At,
            TokenKind::Colon => Self::Colon,
            TokenKind::Colon2 => Self::PathSeparator,
            TokenKind::Comma => Self::Comma,
            TokenKind::Dot => Self::Dot,
            TokenKind::QuestionMark => Self::QuestionMark,
            TokenKind::Semicolon => Self::Semicolon,

            TokenKind::StringUnterminated | TokenKind::Unknown | TokenKind::Eof => Self::Unknown,
            TokenKind::Ampersand2 => Self::And,
            TokenKind::Bang => Self::Not,
            TokenKind::BangEquals => Self::NotEqual,
            TokenKind::Equals => Self::Assign,
            TokenKind::Equals2 => Self::Equal,
            TokenKind::GreaterThan => Self::Greater,
            TokenKind::GreaterThanEquals => Self::GreaterEqual,
            TokenKind::LessThan => Self::Less,
            TokenKind::LessThanEquals => Self::LessEqual,
            TokenKind::Minus => Self::Subtract,
            TokenKind::Pipe2 => Self::Or,
            TokenKind::Plus => Self::Add,
            TokenKind::Asterisk => Self::Multiply,

            TokenKind::Comment => Self::Comment,
            TokenKind::Newline => Self::Newline,
            TokenKind::Whitespace => Self::Whitespace,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Identifier => "identifier",
            Self::Integer => "integer",
            Self::String => "string",

            Self::ActionKeyword => "action",
            Self::AppliesToKeyword => "appliesTo",
            Self::AttributesKeyword => "attributes",
            Self::BoolKeyword => "Bool",
            Self::ContextKeyword => "context",
            Self::ElseKeyword => "else",
            Self::EntityKeyword => "entity",
            Self::EnumKeyword => "enum",
            Self::FalseKeyword => "false",
            Self::ForbidKeyword => "forbid",
            Self::HasKeyword => "has",
            Self::IfKeyword => "if",
            Self::InKeyword => "in",
            Self::IsKeyword => "is",
            Self::LikeKeyword => "like",
            Self::LongKeyword => "Long",
            Self::NamespaceKeyword => "namespace",
            Self::PermitKeyword => "permit",
            Self::PrincipalKeyword => "principal",
            Self::ResourceKeyword => "resource",
            Self::SetKeyword => "Set",
            Self::StringKeyword => "String",
            Self::TagsKeyword => "tags",
            Self::ThenKeyword => "then",
            Self::TrueKeyword => "true",
            Self::TypeKeyword => "type",
            Self::UnlessKeyword => "unless",
            Self::WhenKeyword => "when",

            Self::OpenParenthesis => "(",
            Self::CloseParenthesis => ")",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",

            Self::At => "@",
            Self::Colon => ":",
            Self::PathSeparator => "::",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::QuestionMark => "?",
            Self::Semicolon => ";",

            Self::And => "&&",
            Self::Assign => "=",
            Self::Equal => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Subtract => "-",
            Self::Not => "!",
            Self::NotEqual => "!=",
            Self::Or => "||",
            Self::Add => "+",
            Self::Multiply => "*",

            Self::Comment => "comment",
            Self::Newline => "newline",
            Self::Whitespace => "whitespace",

            Self::Unknown => "unknown",
        };

        f.write_str(text)
    }
}
