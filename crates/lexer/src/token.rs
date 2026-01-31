use core::fmt;

/// A token produced by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,

    /// Length in bytes.
    pub len: u32,
}

impl Token {
    /// Creates a new token.
    #[must_use]
    pub const fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

/// The kind of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Integer literal: `123`.
    Integer,
    /// String literal: `"hello"`.
    String,
    /// Unterminated string: `"hello`.
    StringUnterminated,
    /// Identifier: `name`.
    Identifier,

    /// Keyword: `action`.
    Action,
    /// Keyword: `appliesTo`.
    AppliesTo,
    /// Keyword: `attributes`.
    Attributes,
    /// Keyword: `Bool`.
    Bool,
    /// Keyword: `context`.
    Context,
    /// Keyword: `else`.
    Else,
    /// Keyword: `entity`.
    Entity,
    /// Keyword: `enum`.
    Enum,
    /// Keyword: `false`.
    False,
    /// Keyword: `forbid`.
    Forbid,
    /// Keyword: `has`.
    Has,
    /// Keyword: `if`.
    If,
    /// Keyword: `in`.
    In,
    /// Keyword: `is`.
    Is,
    /// Keyword: `like`.
    Like,
    /// Keyword: `Long`.
    Long,
    /// Keyword: `namespace`.
    Namespace,
    /// Keyword: `permit`.
    Permit,
    /// Keyword: `principal`.
    Principal,
    /// Keyword: `resource`.
    Resource,
    /// Keyword: `Set`.
    Set,
    /// Keyword: `String`.
    StringType,
    /// Keyword: `tags`.
    Tags,
    /// Keyword: `then`.
    Then,
    /// Keyword: `true`.
    True,
    /// Keyword: `type`.
    Type,
    /// Keyword: `unless`.
    Unless,
    /// Keyword: `when`.
    When,

    /// Open parenthesis: `(`.
    OpenParen,
    /// Close parenthesis: `)`.
    CloseParen,
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
    Question,
    /// Semicolon: `;`.
    Semicolon,

    /// Logical and: `&&`.
    Amp2,
    /// Logical not: `!`.
    Bang,
    /// Not equal: `!=`.
    BangEq,
    /// Equals: `=`.
    Eq,
    /// Double equals: `==`.
    Eq2,
    /// Greater than: `>`.
    Gt,
    /// Greater than or equal: `>=`.
    GtEq,
    /// Less than: `<`.
    Lt,
    /// Less than or equal: `<=`.
    LtEq,
    /// Minus: `-`.
    Minus,
    /// Percent: `%`.
    Percent,
    /// Logical or: `||`.
    Pipe2,
    /// Plus: `+`.
    Plus,
    /// Slash: `/`.
    Slash,
    /// Star: `*`.
    Star,

    /// Line comment: `// ...`.
    Comment,
    /// Whitespace.
    Whitespace,

    /// Unrecognized token.
    Unknown,
}

impl TokenKind {
    /// Returns the keyword kind for the given text.
    #[must_use]
    pub fn from_identifier(text: &str) -> Self {
        match text {
            "action" => Self::Action,
            "appliesTo" => Self::AppliesTo,
            "attributes" => Self::Attributes,
            "Bool" => Self::Bool,
            "context" => Self::Context,
            "else" => Self::Else,
            "entity" => Self::Entity,
            "enum" => Self::Enum,
            "false" => Self::False,
            "forbid" => Self::Forbid,
            "has" => Self::Has,
            "if" => Self::If,
            "in" => Self::In,
            "is" => Self::Is,
            "like" => Self::Like,
            "Long" => Self::Long,
            "namespace" => Self::Namespace,
            "permit" => Self::Permit,
            "principal" => Self::Principal,
            "resource" => Self::Resource,
            "Set" => Self::Set,
            "String" => Self::StringType,
            "tags" => Self::Tags,
            "then" => Self::Then,
            "true" => Self::True,
            "type" => Self::Type,
            "unless" => Self::Unless,
            "when" => Self::When,
            _ => Self::Identifier,
        }
    }

    /// Returns the punctuation kind.
    #[must_use]
    pub const fn from_punctuation(current: u8, next: Option<u8>) -> Option<(Self, u8)> {
        let (kind, len) = match current {
            b'(' => (Self::OpenParen, 1),
            b')' => (Self::CloseParen, 1),
            b'{' => (Self::OpenBrace, 1),
            b'}' => (Self::CloseBrace, 1),
            b'[' => (Self::OpenBracket, 1),
            b']' => (Self::CloseBracket, 1),
            b'@' => (Self::At, 1),
            b':' if matches!(next, Some(b':')) => (Self::Colon2, 2),
            b':' => (Self::Colon, 1),
            b',' => (Self::Comma, 1),
            b'.' => (Self::Dot, 1),
            b'?' => (Self::Question, 1),
            b';' => (Self::Semicolon, 1),
            b'&' if matches!(next, Some(b'&')) => (Self::Amp2, 2),
            b'|' if matches!(next, Some(b'|')) => (Self::Pipe2, 2),
            b'!' if matches!(next, Some(b'=')) => (Self::BangEq, 2),
            b'!' => (Self::Bang, 1),
            b'=' if matches!(next, Some(b'=')) => (Self::Eq2, 2),
            b'=' => (Self::Eq, 1),
            b'<' if matches!(next, Some(b'=')) => (Self::LtEq, 2),
            b'<' => (Self::Lt, 1),
            b'>' if matches!(next, Some(b'=')) => (Self::GtEq, 2),
            b'>' => (Self::Gt, 1),
            b'+' => (Self::Plus, 1),
            b'-' => (Self::Minus, 1),
            b'*' => (Self::Star, 1),
            b'/' => (Self::Slash, 1),
            b'%' => (Self::Percent, 1),
            _ => return None,
        };

        Some((kind, len))
    }

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
            Self::Action
                | Self::AppliesTo
                | Self::Attributes
                | Self::Bool
                | Self::Context
                | Self::Else
                | Self::Entity
                | Self::Enum
                | Self::False
                | Self::Forbid
                | Self::Has
                | Self::If
                | Self::In
                | Self::Is
                | Self::Like
                | Self::Long
                | Self::Namespace
                | Self::Permit
                | Self::Principal
                | Self::Resource
                | Self::Set
                | Self::StringType
                | Self::Tags
                | Self::Then
                | Self::True
                | Self::Type
                | Self::Unless
                | Self::When
        )
    }

    /// Checks if this is a scope variable keyword.
    #[must_use]
    pub const fn is_variable(self) -> bool {
        matches!(
            self,
            Self::Principal | Self::Action | Self::Resource | Self::Context
        )
    }

    /// Checks if this is a comparison operator.
    #[must_use]
    pub const fn is_comparison(self) -> bool {
        matches!(
            self,
            Self::Lt | Self::LtEq | Self::Gt | Self::GtEq | Self::Eq2 | Self::BangEq | Self::In
        )
    }

    /// Checks if this is a literal token.
    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::True | Self::False | Self::Integer | Self::String | Self::StringUnterminated
        )
    }

    /// Checks if this is a schema declaration keyword.
    #[must_use]
    pub const fn is_declaration(self) -> bool {
        matches!(
            self,
            Self::Entity | Self::Action | Self::Type | Self::Namespace
        )
    }

    /// Checks if this can be used as an identifier.
    #[must_use]
    pub const fn is_identifier(self) -> bool {
        matches!(self, Self::Identifier) || self.is_keyword()
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Integer => "integer",
            Self::String => "string",
            Self::StringUnterminated => "unterminated string",
            Self::Identifier => "identifier",
            Self::Action => "`action`",
            Self::AppliesTo => "`appliesTo`",
            Self::Attributes => "`attributes`",
            Self::Bool => "`Bool`",
            Self::Context => "`context`",
            Self::Else => "`else`",
            Self::Entity => "`entity`",
            Self::Enum => "`enum`",
            Self::False => "`false`",
            Self::Forbid => "`forbid`",
            Self::Has => "`has`",
            Self::If => "`if`",
            Self::In => "`in`",
            Self::Is => "`is`",
            Self::Like => "`like`",
            Self::Long => "`Long`",
            Self::Namespace => "`namespace`",
            Self::Permit => "`permit`",
            Self::Principal => "`principal`",
            Self::Resource => "`resource`",
            Self::Set => "`Set`",
            Self::StringType => "`String`",
            Self::Tags => "`tags`",
            Self::Then => "`then`",
            Self::True => "`true`",
            Self::Type => "`type`",
            Self::Unless => "`unless`",
            Self::When => "`when`",
            Self::OpenParen => "`(`",
            Self::CloseParen => "`)`",
            Self::OpenBrace => "`{`",
            Self::CloseBrace => "`}`",
            Self::OpenBracket => "`[`",
            Self::CloseBracket => "`]`",
            Self::At => "`@`",
            Self::Colon => "`:`",
            Self::Colon2 => "`::`",
            Self::Comma => "`,`",
            Self::Dot => "`.`",
            Self::Question => "`?`",
            Self::Semicolon => "`;`",
            Self::Amp2 => "`&&`",
            Self::Bang => "`!`",
            Self::BangEq => "`!=`",
            Self::Eq => "`=`",
            Self::Eq2 => "`==`",
            Self::Gt => "`>`",
            Self::GtEq => "`>=`",
            Self::Lt => "`<`",
            Self::LtEq => "`<=`",
            Self::Minus => "`-`",
            Self::Percent => "`%`",
            Self::Pipe2 => "`||`",
            Self::Plus => "`+`",
            Self::Slash => "`/`",
            Self::Star => "`*`",
            Self::Comment => "comment",
            Self::Whitespace => "whitespace",
            Self::Unknown => "unknown",
        };

        f.write_str(text)
    }
}
