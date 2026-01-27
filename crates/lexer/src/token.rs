/// A token produced by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,

    /// Length in bytes.
    pub len: usize,
}

impl Token {
    /// Creates a new token.
    #[must_use]
    pub const fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

/// The kind of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Integer: `123`
    Integer,
    /// String: `"hello"`
    String,
    /// Unterminated string: `"hello`
    StringUnterminated,

    /// Identifier: `name`
    Identifier,

    /// `else`
    Else,
    /// `false`
    False,
    /// `has`
    Has,
    /// `if`
    If,
    /// `in`
    In,
    /// `is`
    Is,
    /// `like`
    Like,
    /// `then`
    Then,
    /// `true`
    True,

    /// `action`
    Action,
    /// `context`
    Context,
    /// `forbid`
    Forbid,
    /// `permit`
    Permit,
    /// `principal`
    Principal,
    /// `resource`
    Resource,
    /// `unless`
    Unless,
    /// `when`
    When,

    /// `appliesTo`
    AppliesTo,
    /// `entity`
    Entity,
    /// `enum`
    Enum,
    /// `namespace`
    Namespace,
    /// `tags`
    Tags,
    /// `type`
    Type,

    /// `(`
    OpenParenthesis,
    /// `)`
    CloseParenthesis,
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
    /// `.`
    Dot,
    /// `?`
    Question,
    /// `;`
    Semicolon,

    /// `&`
    Ampersand,
    /// `&&`
    Ampersand2,
    /// `!`
    Exclamation,
    /// `!=`
    NotEqual,
    /// `=`
    Equal,
    /// `==`
    Equal2,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanOrEqual,
    /// `<`
    LessThan,
    /// `<=`
    LessThanOrEqual,
    /// `-`
    Minus,
    /// `||`
    Pipe2,
    /// `%`
    Percent,
    /// `|`
    Pipe,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `*`
    Star,

    /// Comment: `// ...`
    Comment,
    /// Whitespace
    Whitespace,

    /// Unknown token
    Unknown,
}

impl TokenKind {
    /// Returns the keyword kind for the given text.
    #[must_use]
    pub fn from_identifier(text: &str) -> Self {
        match text {
            "else" => Self::Else,
            "false" => Self::False,
            "has" => Self::Has,
            "if" => Self::If,
            "in" => Self::In,
            "is" => Self::Is,
            "like" => Self::Like,
            "then" => Self::Then,
            "true" => Self::True,
            "action" => Self::Action,
            "context" => Self::Context,
            "forbid" => Self::Forbid,
            "permit" => Self::Permit,
            "principal" => Self::Principal,
            "resource" => Self::Resource,
            "unless" => Self::Unless,
            "when" => Self::When,
            "appliesTo" => Self::AppliesTo,
            "entity" => Self::Entity,
            "enum" => Self::Enum,
            "namespace" => Self::Namespace,
            "tags" => Self::Tags,
            "type" => Self::Type,
            _ => Self::Identifier,
        }
    }

    /// Returns the punctuation kind.
    #[must_use]
    pub const fn from_punctuation(current: u8, next: Option<u8>) -> Option<(Self, u8)> {
        let (kind, len) = match current {
            b'(' => (Self::OpenParenthesis, 1),
            b')' => (Self::CloseParenthesis, 1),
            b'{' => (Self::OpenBrace, 1),
            b'}' => (Self::CloseBrace, 1),
            b'[' => (Self::OpenBracket, 1),
            b']' => (Self::CloseBracket, 1),
            b',' => (Self::Comma, 1),
            b';' => (Self::Semicolon, 1),
            b'@' => (Self::At, 1),
            b'.' => (Self::Dot, 1),
            b'?' => (Self::Question, 1),
            b'+' => (Self::Plus, 1),
            b'-' => (Self::Minus, 1),
            b'*' => (Self::Star, 1),
            b'/' => (Self::Slash, 1),
            b'%' => (Self::Percent, 1),
            b':' if matches!(next, Some(b':')) => (Self::Colon2, 2),
            b':' => (Self::Colon, 1),
            b'=' if matches!(next, Some(b'=')) => (Self::Equal2, 2),
            b'=' => (Self::Equal, 1),
            b'!' if matches!(next, Some(b'=')) => (Self::NotEqual, 2),
            b'!' => (Self::Exclamation, 1),
            b'<' if matches!(next, Some(b'=')) => (Self::LessThanOrEqual, 2),
            b'<' => (Self::LessThan, 1),
            b'>' if matches!(next, Some(b'=')) => (Self::GreaterThanOrEqual, 2),
            b'>' => (Self::GreaterThan, 1),
            b'&' if matches!(next, Some(b'&')) => (Self::Ampersand2, 2),
            b'&' => (Self::Ampersand, 1),
            b'|' if matches!(next, Some(b'|')) => (Self::Pipe2, 2),
            b'|' => (Self::Pipe, 1),
            _ => return None,
        };

        Some((kind, len))
    }

    /// Checks if this is a trivial token.
    #[must_use]
    pub const fn is_trivial(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}
