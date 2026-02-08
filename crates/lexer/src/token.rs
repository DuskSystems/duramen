use core::fmt;

/// A token produced by the lexer.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,

    /// Length in bytes.
    pub len: usize,
}

impl Token {
    /// Creates a new token.
    #[must_use]
    pub(crate) const fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

/// The kind of token.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TokenKind {
    /// Identifier: `name`.
    Identifier,
    /// Integer literal: `123`.
    Integer,
    /// String literal: `"hello"`.
    String,
    /// Unterminated string literal: `"hello`.
    StringUnterminated,

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
    /// Double colon: `::`.
    Colon2,
    /// Comma: `,`.
    Comma,
    /// Dot: `.`.
    Dot,
    /// Question mark: `?`.
    QuestionMark,
    /// Semicolon: `;`.
    Semicolon,

    /// Ampersand: `&`.
    Ampersand,
    /// Two ampersands: `&&`.
    Ampersand2,
    /// Asterisk: `*`.
    Asterisk,
    /// Bang: `!`.
    Bang,
    /// Bang equals: `!=`.
    BangEquals,
    /// Equals: `=`.
    Equals,
    /// Two equals: `==`.
    Equals2,
    /// Greater than: `>`.
    GreaterThan,
    /// Greater than equals: `>=`.
    GreaterThanEquals,
    /// Less than: `<`.
    LessThan,
    /// Less than equals: `<=`.
    LessThanEquals,
    /// Minus: `-`.
    Minus,
    /// Percent: `%`.
    Percent,
    /// Pipe: `|`.
    Pipe,
    /// Two pipes: `||`.
    Pipe2,
    /// Plus: `+`.
    Plus,
    /// Slash: `/`.
    Slash,

    /// Comment: `// ...`.
    Comment,
    /// Whitespace.
    Whitespace,

    /// Unrecognized token.
    Unknown,
    /// End of file.
    Eof,
}

impl TokenKind {
    /// Checks if this is a trivial token.
    #[must_use]
    pub const fn is_trivial(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
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

    /// Checks if this is a scope variable keyword.
    #[must_use]
    pub const fn is_variable(self) -> bool {
        matches!(
            self,
            Self::PrincipalKeyword
                | Self::ActionKeyword
                | Self::ResourceKeyword
                | Self::ContextKeyword
        )
    }

    /// Checks if this is a comparison operator.
    #[must_use]
    pub const fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::LessThan
                | Self::LessThanEquals
                | Self::GreaterThan
                | Self::GreaterThanEquals
                | Self::Equals2
                | Self::BangEquals
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

    /// Checks if this is a declaration keyword.
    #[must_use]
    pub const fn is_declaration(self) -> bool {
        matches!(
            self,
            Self::EntityKeyword | Self::ActionKeyword | Self::TypeKeyword | Self::NamespaceKeyword
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

    /// Returns the keyword kind for the given text.
    #[must_use]
    pub(crate) fn from_identifier(text: &str) -> Self {
        match text {
            "action" => Self::ActionKeyword,
            "appliesTo" => Self::AppliesToKeyword,
            "attributes" => Self::AttributesKeyword,
            "Bool" => Self::BoolKeyword,
            "context" => Self::ContextKeyword,
            "else" => Self::ElseKeyword,
            "entity" => Self::EntityKeyword,
            "enum" => Self::EnumKeyword,
            "false" => Self::FalseKeyword,
            "forbid" => Self::ForbidKeyword,
            "has" => Self::HasKeyword,
            "if" => Self::IfKeyword,
            "in" => Self::InKeyword,
            "is" => Self::IsKeyword,
            "like" => Self::LikeKeyword,
            "Long" => Self::LongKeyword,
            "namespace" => Self::NamespaceKeyword,
            "permit" => Self::PermitKeyword,
            "principal" => Self::PrincipalKeyword,
            "resource" => Self::ResourceKeyword,
            "Set" => Self::SetKeyword,
            "String" => Self::StringKeyword,
            "tags" => Self::TagsKeyword,
            "then" => Self::ThenKeyword,
            "true" => Self::TrueKeyword,
            "type" => Self::TypeKeyword,
            "unless" => Self::UnlessKeyword,
            "when" => Self::WhenKeyword,

            _ => Self::Identifier,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Identifier => "identifier",
            Self::Integer => "integer",
            Self::String => "string",
            Self::StringUnterminated => "unterminated string",

            Self::ActionKeyword => "`action`",
            Self::AppliesToKeyword => "`appliesTo`",
            Self::AttributesKeyword => "`attributes`",
            Self::BoolKeyword => "`Bool`",
            Self::ContextKeyword => "`context`",
            Self::ElseKeyword => "`else`",
            Self::EntityKeyword => "`entity`",
            Self::EnumKeyword => "`enum`",
            Self::FalseKeyword => "`false`",
            Self::ForbidKeyword => "`forbid`",
            Self::HasKeyword => "`has`",
            Self::IfKeyword => "`if`",
            Self::InKeyword => "`in`",
            Self::IsKeyword => "`is`",
            Self::LikeKeyword => "`like`",
            Self::LongKeyword => "`Long`",
            Self::NamespaceKeyword => "`namespace`",
            Self::PermitKeyword => "`permit`",
            Self::PrincipalKeyword => "`principal`",
            Self::ResourceKeyword => "`resource`",
            Self::SetKeyword => "`Set`",
            Self::StringKeyword => "`String`",
            Self::TagsKeyword => "`tags`",
            Self::ThenKeyword => "`then`",
            Self::TrueKeyword => "`true`",
            Self::TypeKeyword => "`type`",
            Self::UnlessKeyword => "`unless`",
            Self::WhenKeyword => "`when`",

            Self::OpenParenthesis => "`(`",
            Self::CloseParenthesis => "`)`",
            Self::OpenBrace => "`{`",
            Self::CloseBrace => "`}`",
            Self::OpenBracket => "`[`",
            Self::CloseBracket => "`]`",

            Self::At => "`@`",
            Self::Colon => "`:`",
            Self::Colon2 => "`::`",
            Self::Comma => "`,`",
            Self::Dot => "`.`",
            Self::QuestionMark => "`?`",
            Self::Semicolon => "`;`",

            Self::Ampersand => "`&`",
            Self::Ampersand2 => "`&&`",
            Self::Asterisk => "`*`",
            Self::Bang => "`!`",
            Self::BangEquals => "`!=`",
            Self::Equals => "`=`",
            Self::Equals2 => "`==`",
            Self::GreaterThan => "`>`",
            Self::GreaterThanEquals => "`>=`",
            Self::LessThan => "`<`",
            Self::LessThanEquals => "`<=`",
            Self::Minus => "`-`",
            Self::Percent => "`%`",
            Self::Pipe => "`|`",
            Self::Pipe2 => "`||`",
            Self::Plus => "`+`",
            Self::Slash => "`/`",

            Self::Comment => "comment",
            Self::Whitespace => "whitespace",

            Self::Unknown => "unknown",
            Self::Eof => "end of file",
        };

        f.write_str(text)
    }
}
