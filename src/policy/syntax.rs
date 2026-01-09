//! Cedar Policy Syntax.
//!
//! # References
//!
//! - [Syntax](https://docs.cedarpolicy.com/policies/syntax-policy.html)
//! - [Operators](https://docs.cedarpolicy.com/policies/syntax-operators.html)
//! - [Grammar](https://docs.cedarpolicy.com/policies/syntax-grammar.html)
//! - [LALRPOP](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/parser/grammar.lalrpop)

/// Token kinds for Cedar policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PolicyTokenKind {
    /// Integer literal.
    Integer,
    /// String literal.
    String,
    /// Identifier.
    Identifier,

    /// Keyword `permit`.
    PermitKeyword,
    /// Keyword `forbid`.
    ForbidKeyword,

    /// Keyword `principal`.
    PrincipalKeyword,
    /// Keyword `action`.
    ActionKeyword,
    /// Keyword `resource`.
    ResourceKeyword,
    /// Keyword `context`.
    ContextKeyword,

    /// Keyword `when`.
    WhenKeyword,
    /// Keyword `unless`.
    UnlessKeyword,

    /// Keyword `true`.
    TrueKeyword,
    /// Keyword `false`.
    FalseKeyword,

    /// Keyword `like`.
    LikeKeyword,

    /// Keyword `if`.
    IfKeyword,
    /// Keyword `then`.
    ThenKeyword,
    /// Keyword `else`.
    ElseKeyword,

    /// Keyword `in`.
    InKeyword,
    /// Keyword `has`.
    HasKeyword,
    /// Keyword `is`.
    IsKeyword,

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

    /// `=` symbol.
    Equal,
    /// `==` symbol.
    Equal2,
    /// `!` symbol.
    Not,
    /// `!=` symbol.
    NotEqual,
    /// `<` symbol.
    LessThan,
    /// `<=` symbol.
    LessEqual,
    /// `>` symbol.
    GreaterThan,
    /// `>=` symbol.
    GreaterEqual,
    /// `&` symbol.
    Ampersand,
    /// `&&` symbol.
    Ampersand2,
    /// `|` symbol.
    Pipe,
    /// `||` symbol.
    Pipe2,
    /// `+` symbol.
    Plus,
    /// `-` symbol.
    Minus,
    /// `*` symbol.
    Asterisk,
    /// `/` symbol.
    Slash,
    /// `%` symbol.
    Percent,

    /// Comment.
    Comment,
    /// Whitespace.
    Whitespace,

    /// End of file.
    Eof,
    /// Unknown token.
    Unknown,
}

impl PolicyTokenKind {
    /// Returns the keyword kind for the given text, if any.
    #[must_use]
    pub fn from_keyword(value: &str) -> Option<Self> {
        match value {
            "permit" => Some(Self::PermitKeyword),
            "forbid" => Some(Self::ForbidKeyword),
            "principal" => Some(Self::PrincipalKeyword),
            "action" => Some(Self::ActionKeyword),
            "resource" => Some(Self::ResourceKeyword),
            "context" => Some(Self::ContextKeyword),
            "when" => Some(Self::WhenKeyword),
            "unless" => Some(Self::UnlessKeyword),
            "true" => Some(Self::TrueKeyword),
            "false" => Some(Self::FalseKeyword),
            "like" => Some(Self::LikeKeyword),
            "if" => Some(Self::IfKeyword),
            "then" => Some(Self::ThenKeyword),
            "else" => Some(Self::ElseKeyword),
            "in" => Some(Self::InKeyword),
            "has" => Some(Self::HasKeyword),
            "is" => Some(Self::IsKeyword),
            _ => None,
        }
    }
}
