use core::fmt;

use duramen_lexer::TokenKind;

/// Syntax kinds for Cedar policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PolicySyntax {
    /// Integer: `123`
    Integer,
    /// String: `"hello"`
    String,
    /// Identifier: `name`
    Identifier,

    /// `action`
    Action,
    /// `context`
    Context,
    /// `else`
    Else,
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
    /// `permit`
    Permit,
    /// `principal`
    Principal,
    /// `resource`
    Resource,
    /// `then`
    Then,
    /// `true`
    True,
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

    /// End of file
    Eof,
    /// Unknown token
    Unknown,
    /// Error node
    Error,

    /// Root node.
    ///
    /// ```cedar
    /// permit(principal, action, resource);
    /// ```
    Policies,

    /// Policy statement.
    ///
    /// ```cedar
    /// permit(principal == User::"alice", action, resource);
    /// ```
    Policy,

    /// Annotation.
    ///
    /// ```cedar
    /// @id("policy-1")
    /// ```
    Annotation,

    /// Variable definition.
    ///
    /// ```cedar
    /// principal == User::"alice"
    /// ```
    VariableDef,

    /// Condition clause.
    ///
    /// ```cedar
    /// when { principal.jobLevel >= 10 }
    /// ```
    Condition,

    /// If expression.
    ///
    /// ```cedar
    /// if true then 1 else 2
    /// ```
    IfExpression,

    /// Or expression.
    ///
    /// ```cedar
    /// true || false
    /// ```
    OrExpression,

    /// And expression.
    ///
    /// ```cedar
    /// true && false
    /// ```
    AndExpression,

    /// Relational expression.
    ///
    /// ```cedar
    /// principal.jobLevel >= 10
    /// ```
    Relation,

    /// Sum expression.
    ///
    /// ```cedar
    /// 1 + 2
    /// ```
    Sum,

    /// Product expression.
    ///
    /// ```cedar
    /// 2 * 3
    /// ```
    Product,

    /// Has expression.
    ///
    /// ```cedar
    /// resource has admins
    /// ```
    HasExpression,

    /// Like expression.
    ///
    /// ```cedar
    /// resource.path like "/home/*"
    /// ```
    LikeExpression,

    /// Is expression.
    ///
    /// ```cedar
    /// resource is Photo
    /// ```
    IsExpression,

    /// Unary expression.
    ///
    /// ```cedar
    /// !context.usedMFA
    /// ```
    Unary,

    /// Member expression.
    ///
    /// ```cedar
    /// principal.department
    /// ```
    Member,

    /// Literal.
    ///
    /// ```cedar
    /// true
    /// ```
    Literal,

    /// Entity reference.
    ///
    /// ```cedar
    /// User::"alice"
    /// ```
    EntityReference,

    /// Slot.
    ///
    /// ```cedar
    /// ?principal
    /// ```
    Slot,

    /// Parenthesized expression.
    ///
    /// ```cedar
    /// (1 + 2)
    /// ```
    Parenthesized,

    /// List.
    ///
    /// ```cedar
    /// ["view", "edit", "delete"]
    /// ```
    List,

    /// Record.
    ///
    /// ```cedar
    /// {name: "alice", age: 30}
    /// ```
    Record,

    /// Field access.
    ///
    /// ```cedar
    /// principal.department
    /// ```
    FieldAccess,

    /// Method call.
    ///
    /// ```cedar
    /// ip("127.0.0.1").isLoopback()
    /// ```
    MethodCall,

    /// Index access.
    ///
    /// ```cedar
    /// resource.roles["admin"]
    /// ```
    IndexAccess,

    /// Qualified name.
    ///
    /// ```cedar
    /// S3::Action
    /// ```
    Name,

    /// Record entry.
    ///
    /// ```cedar
    /// name: "alice"
    /// ```
    RecordEntry,

    /// Argument list.
    ///
    /// ```cedar
    /// ("192.0.2.0/24")
    /// ```
    ArgumentList,
}

impl PolicySyntax {
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
        matches!(
            self,
            Self::True | Self::False | Self::Integer | Self::String
        )
    }

    /// Checks if this is a keyword.
    #[must_use]
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::Action
                | Self::Context
                | Self::Else
                | Self::False
                | Self::Forbid
                | Self::Has
                | Self::If
                | Self::In
                | Self::Is
                | Self::Like
                | Self::Permit
                | Self::Principal
                | Self::Resource
                | Self::Then
                | Self::True
                | Self::Unless
                | Self::When
        )
    }

    /// Checks if this is a token.
    #[must_use]
    pub const fn is_token(self) -> bool {
        matches!(
            self,
            Self::Integer
                | Self::String
                | Self::Identifier
                | Self::Action
                | Self::Context
                | Self::Else
                | Self::False
                | Self::Forbid
                | Self::Has
                | Self::If
                | Self::In
                | Self::Is
                | Self::Like
                | Self::Permit
                | Self::Principal
                | Self::Resource
                | Self::Then
                | Self::True
                | Self::Unless
                | Self::When
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
                | Self::Dot
                | Self::Question
                | Self::Semicolon
                | Self::Amp2
                | Self::Bang
                | Self::BangEq
                | Self::Eq
                | Self::Eq2
                | Self::Gt
                | Self::GtEq
                | Self::Lt
                | Self::LtEq
                | Self::Minus
                | Self::Percent
                | Self::Pipe2
                | Self::Plus
                | Self::Slash
                | Self::Star
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

impl fmt::Display for PolicySyntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Integer => "integer",
            Self::String => "string",
            Self::Identifier => "identifier",
            Self::Action => "action",
            Self::Context => "context",
            Self::Else => "else",
            Self::False => "false",
            Self::Forbid => "forbid",
            Self::Has => "has",
            Self::If => "if",
            Self::In => "in",
            Self::Is => "is",
            Self::Like => "like",
            Self::Permit => "permit",
            Self::Principal => "principal",
            Self::Resource => "resource",
            Self::Then => "then",
            Self::True => "true",
            Self::Unless => "unless",
            Self::When => "when",
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
            Self::Dot => ".",
            Self::Question => "?",
            Self::Semicolon => ";",
            Self::Amp2 => "&&",
            Self::Bang => "!",
            Self::BangEq => "!=",
            Self::Eq => "=",
            Self::Eq2 => "==",
            Self::Gt => ">",
            Self::GtEq => ">=",
            Self::Lt => "<",
            Self::LtEq => "<=",
            Self::Minus => "-",
            Self::Percent => "%",
            Self::Pipe2 => "||",
            Self::Plus => "+",
            Self::Slash => "/",
            Self::Star => "*",
            Self::Comment => "comment",
            Self::Whitespace => "whitespace",
            Self::Eof => "end of file",
            Self::Unknown => "unknown",
            Self::Error => "error",
            Self::Policies => "policies",
            Self::Policy => "policy",
            Self::Annotation => "annotation",
            Self::VariableDef => "variable definition",
            Self::Condition => "condition",
            Self::IfExpression => "if expression",
            Self::OrExpression => "or expression",
            Self::AndExpression => "and expression",
            Self::Relation => "relational expression",
            Self::Sum => "sum expression",
            Self::Product => "product expression",
            Self::HasExpression => "has expression",
            Self::LikeExpression => "like expression",
            Self::IsExpression => "is expression",
            Self::Unary => "unary expression",
            Self::Member => "member expression",
            Self::Literal => "literal",
            Self::EntityReference => "entity reference",
            Self::Slot => "slot",
            Self::Parenthesized => "parenthesized expression",
            Self::List => "list",
            Self::Record => "record",
            Self::FieldAccess => "field access",
            Self::MethodCall => "method call",
            Self::IndexAccess => "index access",
            Self::Name => "name",
            Self::RecordEntry => "record entry",
            Self::ArgumentList => "argument list",
        };

        f.write_str(text)
    }
}

impl From<TokenKind> for PolicySyntax {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Integer => Self::Integer,
            TokenKind::String | TokenKind::StringUnterminated => Self::String,

            TokenKind::Identifier
            | TokenKind::AppliesTo
            | TokenKind::Attributes
            | TokenKind::Bool
            | TokenKind::Entity
            | TokenKind::Enum
            | TokenKind::Long
            | TokenKind::Namespace
            | TokenKind::Set
            | TokenKind::StringType
            | TokenKind::Tags
            | TokenKind::Type => Self::Identifier,

            TokenKind::Action => Self::Action,
            TokenKind::Context => Self::Context,
            TokenKind::Else => Self::Else,
            TokenKind::False => Self::False,
            TokenKind::Forbid => Self::Forbid,
            TokenKind::Has => Self::Has,
            TokenKind::If => Self::If,
            TokenKind::In => Self::In,
            TokenKind::Is => Self::Is,
            TokenKind::Like => Self::Like,
            TokenKind::Permit => Self::Permit,
            TokenKind::Principal => Self::Principal,
            TokenKind::Resource => Self::Resource,
            TokenKind::Then => Self::Then,
            TokenKind::True => Self::True,
            TokenKind::Unless => Self::Unless,
            TokenKind::When => Self::When,

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
            TokenKind::Dot => Self::Dot,
            TokenKind::Question => Self::Question,
            TokenKind::Semicolon => Self::Semicolon,

            TokenKind::Amp2 => Self::Amp2,
            TokenKind::Bang => Self::Bang,
            TokenKind::BangEq => Self::BangEq,
            TokenKind::Eq => Self::Eq,
            TokenKind::Eq2 => Self::Eq2,
            TokenKind::Gt => Self::Gt,
            TokenKind::GtEq => Self::GtEq,
            TokenKind::Lt => Self::Lt,
            TokenKind::LtEq => Self::LtEq,
            TokenKind::Minus => Self::Minus,
            TokenKind::Percent => Self::Percent,
            TokenKind::Pipe2 => Self::Pipe2,
            TokenKind::Plus => Self::Plus,
            TokenKind::Slash => Self::Slash,
            TokenKind::Star => Self::Star,

            TokenKind::Comment => Self::Comment,
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::Unknown => Self::Unknown,
        }
    }
}
