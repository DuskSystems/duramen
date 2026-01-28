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
    /// `else`
    Else,
    /// `entity`
    Entity,
    /// `enum`
    Enum,
    /// `false`
    False,
    /// `forbid`
    Forbid,
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
    /// `Long`
    Long,
    /// `namespace`
    Namespace,
    /// `permit`
    Permit,
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
    /// `then`
    Then,
    /// `true`
    True,
    /// `type`
    Type,
    /// `unless`
    Unless,
    /// `when`
    When,

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
    /// `.`
    Dot,
    /// `?`
    Question,
    /// `;`
    Semicolon,

    /// `&&`
    Amp2,
    /// `!`
    Bang,
    /// `!=`
    BangEq,
    /// `=`
    Eq,
    /// `==`
    Eq2,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `-`
    Minus,
    /// `%`
    Percent,
    /// `||`
    Pipe2,
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
}
