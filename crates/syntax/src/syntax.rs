use core::convert::TryFrom;
use core::fmt;

use duramen_lexer::TokenKind;

/// Syntax kinds for Cedar policies and schemas.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Syntax {
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
    /// Modulo: `%`.
    Modulo,
    /// Add: `+`.
    Add,
    /// Divide: `/`.
    Divide,
    /// Multiply: `*`.
    Multiply,

    /// Line comment: `// ...`.
    Comment,
    /// Newline: `\n` or `\r\n`.
    Newline,
    /// Same-line whitespace: spaces, tabs.
    Whitespace,

    /// Unknown token from the lexer.
    Unknown,

    /// Error node.
    Error,

    /// Annotation: `@id("value")`.
    Annotation,
    /// Qualified name: `Namespace::Type`.
    Name,

    /// Root node for policies.
    Policies,
    /// Policy statement.
    Policy,
    /// Variable definition: `principal == User::"alice"`.
    VariableDefinition,
    /// Condition clause: `when { ... }`.
    Condition,

    /// If expression: `if x then y else z`.
    IfExpression,
    /// Or expression: `a || b`.
    OrExpression,
    /// And expression: `a && b`.
    AndExpression,
    /// Relation expression: `a < b`.
    RelationExpression,
    /// Sum expression: `a + b`.
    SumExpression,
    /// Product expression: `a * b`.
    ProductExpression,
    /// Has expression: `x has field`.
    HasExpression,
    /// Like expression: `x like "pattern"`.
    LikeExpression,
    /// Is expression: `x is Type`.
    IsExpression,
    /// Unary expression: `!x` or `-x`.
    UnaryExpression,
    /// Member expression: `x.field`.
    MemberExpression,

    /// Literal: `true`, `123`, `"string"`.
    Literal,
    /// Entity reference: `Type::"id"`.
    EntityReference,
    /// Slot: `?principal`.
    Slot,
    /// Parenthesized expression: `(x)`.
    Parenthesized,
    /// List: `[a, b, c]`.
    List,
    /// Record: `{ key: value }`.
    Record,

    /// Field access: `.field`.
    Field,
    /// Method call: `.method(args)`.
    Call,
    /// Index access: `[index]`.
    Index,

    /// Record entry: `key: value`.
    RecordEntry,
    /// Argument list: `(a, b, c)`.
    Arguments,

    /// Root node for schemas.
    Schema,

    /// Namespace declaration.
    NamespaceDeclaration,
    /// Entity declaration.
    EntityDeclaration,
    /// Action declaration.
    ActionDeclaration,
    /// Type declaration.
    TypeDeclaration,

    /// Entity parents: `in [Group]`.
    EntityParents,
    /// Entity attributes: `= { ... }`.
    EntityAttributes,
    /// Entity tags: `tags Type`.
    EntityTags,

    /// Applies-to clause.
    AppliesToClause,
    /// Principal types.
    PrincipalTypes,
    /// Resource types.
    ResourceTypes,
    /// Context type.
    ContextType,
    /// Action parents.
    ActionParents,
    /// Action attributes.
    ActionAttributes,

    /// Type expression wrapper.
    TypeExpression,
    /// Set type: `Set<T>`.
    SetType,
    /// Record type: `{ ... }`.
    RecordType,
    /// Entity type reference.
    EntityType,
    /// Enum type: `enum ["a", "b"]`.
    EnumType,

    /// Attribute declaration.
    AttributeDeclaration,
    /// Type list: `[A, B]`.
    Types,
}

impl Syntax {
    /// Checks if this is a trivial token.
    #[must_use]
    pub fn is_trivial(self) -> bool {
        TokenKind::try_from(self).is_ok_and(TokenKind::is_trivial)
    }

    /// Checks if this is an error or unknown token.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Unknown)
    }

    /// Checks if this is a reserved keyword in policy context.
    #[must_use]
    pub fn is_reserved(self) -> bool {
        TokenKind::try_from(self).is_ok_and(TokenKind::is_reserved)
    }

    /// Checks if this can be used as an identifier.
    #[must_use]
    pub fn is_identifier(self) -> bool {
        TokenKind::try_from(self).is_ok_and(TokenKind::is_identifier)
    }

    /// Checks if this is a keyword token.
    #[must_use]
    pub fn is_keyword(self) -> bool {
        TokenKind::try_from(self).is_ok_and(TokenKind::is_keyword)
    }

    /// Checks if this is a literal token.
    #[must_use]
    pub fn is_literal(self) -> bool {
        TokenKind::try_from(self).is_ok_and(TokenKind::is_literal)
    }

    /// Checks if this is a string token.
    #[must_use]
    pub const fn is_string(self) -> bool {
        matches!(self, Self::String)
    }

    /// Checks if this is a token.
    #[must_use]
    pub fn is_token(self) -> bool {
        TokenKind::try_from(self).is_ok()
    }

    /// Checks if this is a node.
    #[must_use]
    pub fn is_node(self) -> bool {
        !self.is_token()
    }
}

impl fmt::Display for Syntax {
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
            Self::Modulo => "%",
            Self::Add => "+",
            Self::Divide => "/",
            Self::Multiply => "*",

            Self::Comment => "comment",
            Self::Newline => "newline",
            Self::Whitespace => "whitespace",

            Self::Unknown => "unknown",
            Self::Error => "error",

            Self::Annotation => "annotation",
            Self::Name => "name",

            Self::Policies => "policies",
            Self::Policy => "policy",
            Self::VariableDefinition => "variable definition",
            Self::Condition => "condition",

            Self::IfExpression => "if expression",
            Self::OrExpression => "or expression",
            Self::AndExpression => "and expression",
            Self::RelationExpression => "relation expression",
            Self::SumExpression => "sum expression",
            Self::ProductExpression => "product expression",
            Self::HasExpression => "has expression",
            Self::LikeExpression => "like expression",
            Self::IsExpression => "is expression",
            Self::UnaryExpression => "unary expression",
            Self::MemberExpression => "member expression",

            Self::Literal => "literal",
            Self::EntityReference => "entity reference",
            Self::Slot => "slot",
            Self::Parenthesized => "parenthesized expression",
            Self::List => "list",
            Self::Record => "record",

            Self::Field => "field",
            Self::Call => "method call",
            Self::Index => "index",

            Self::RecordEntry => "record entry",
            Self::Arguments => "arguments",

            Self::Schema => "schema",

            Self::NamespaceDeclaration => "namespace declaration",
            Self::EntityDeclaration => "entity declaration",
            Self::ActionDeclaration => "action declaration",
            Self::TypeDeclaration => "type declaration",

            Self::EntityParents => "entity parents",
            Self::EntityAttributes => "entity attributes",
            Self::EntityTags => "entity tags",

            Self::AppliesToClause => "applies-to clause",
            Self::PrincipalTypes => "principal types",
            Self::ResourceTypes => "resource types",
            Self::ContextType => "context type",
            Self::ActionParents => "action parents",
            Self::ActionAttributes => "action attributes",

            Self::TypeExpression => "type expression",
            Self::SetType => "set type",
            Self::RecordType => "record type",
            Self::EntityType => "entity type",
            Self::EnumType => "enum type",

            Self::AttributeDeclaration => "attribute declaration",
            Self::Types => "types",
        };

        f.write_str(text)
    }
}

impl From<TokenKind> for Syntax {
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

            TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::StringUnterminated
            | TokenKind::Unknown
            | TokenKind::Eof => Self::Unknown,
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
            TokenKind::Percent => Self::Modulo,
            TokenKind::Pipe2 => Self::Or,
            TokenKind::Plus => Self::Add,
            TokenKind::Slash => Self::Divide,
            TokenKind::Asterisk => Self::Multiply,

            TokenKind::Comment => Self::Comment,
            TokenKind::Newline => Self::Newline,
            TokenKind::Whitespace => Self::Whitespace,
        }
    }
}

impl TryFrom<Syntax> for TokenKind {
    type Error = ();

    fn try_from(value: Syntax) -> Result<Self, Self::Error> {
        match value {
            Syntax::Identifier => Ok(Self::Identifier),
            Syntax::Integer => Ok(Self::Integer),
            Syntax::String => Ok(Self::String),

            Syntax::ActionKeyword => Ok(Self::ActionKeyword),
            Syntax::AppliesToKeyword => Ok(Self::AppliesToKeyword),
            Syntax::AttributesKeyword => Ok(Self::AttributesKeyword),
            Syntax::BoolKeyword => Ok(Self::BoolKeyword),
            Syntax::ContextKeyword => Ok(Self::ContextKeyword),
            Syntax::ElseKeyword => Ok(Self::ElseKeyword),
            Syntax::EntityKeyword => Ok(Self::EntityKeyword),
            Syntax::EnumKeyword => Ok(Self::EnumKeyword),
            Syntax::FalseKeyword => Ok(Self::FalseKeyword),
            Syntax::ForbidKeyword => Ok(Self::ForbidKeyword),
            Syntax::HasKeyword => Ok(Self::HasKeyword),
            Syntax::IfKeyword => Ok(Self::IfKeyword),
            Syntax::InKeyword => Ok(Self::InKeyword),
            Syntax::IsKeyword => Ok(Self::IsKeyword),
            Syntax::LikeKeyword => Ok(Self::LikeKeyword),
            Syntax::LongKeyword => Ok(Self::LongKeyword),
            Syntax::NamespaceKeyword => Ok(Self::NamespaceKeyword),
            Syntax::PermitKeyword => Ok(Self::PermitKeyword),
            Syntax::PrincipalKeyword => Ok(Self::PrincipalKeyword),
            Syntax::ResourceKeyword => Ok(Self::ResourceKeyword),
            Syntax::SetKeyword => Ok(Self::SetKeyword),
            Syntax::StringKeyword => Ok(Self::StringKeyword),
            Syntax::TagsKeyword => Ok(Self::TagsKeyword),
            Syntax::ThenKeyword => Ok(Self::ThenKeyword),
            Syntax::TrueKeyword => Ok(Self::TrueKeyword),
            Syntax::TypeKeyword => Ok(Self::TypeKeyword),
            Syntax::UnlessKeyword => Ok(Self::UnlessKeyword),
            Syntax::WhenKeyword => Ok(Self::WhenKeyword),

            Syntax::OpenParenthesis => Ok(Self::OpenParenthesis),
            Syntax::CloseParenthesis => Ok(Self::CloseParenthesis),
            Syntax::OpenBrace => Ok(Self::OpenBrace),
            Syntax::CloseBrace => Ok(Self::CloseBrace),
            Syntax::OpenBracket => Ok(Self::OpenBracket),
            Syntax::CloseBracket => Ok(Self::CloseBracket),

            Syntax::At => Ok(Self::At),
            Syntax::Colon => Ok(Self::Colon),
            Syntax::PathSeparator => Ok(Self::Colon2),
            Syntax::Comma => Ok(Self::Comma),
            Syntax::Dot => Ok(Self::Dot),
            Syntax::QuestionMark => Ok(Self::QuestionMark),
            Syntax::Semicolon => Ok(Self::Semicolon),

            Syntax::And => Ok(Self::Ampersand2),
            Syntax::Assign => Ok(Self::Equals),
            Syntax::Equal => Ok(Self::Equals2),
            Syntax::Greater => Ok(Self::GreaterThan),
            Syntax::GreaterEqual => Ok(Self::GreaterThanEquals),
            Syntax::Less => Ok(Self::LessThan),
            Syntax::LessEqual => Ok(Self::LessThanEquals),
            Syntax::Subtract => Ok(Self::Minus),
            Syntax::Not => Ok(Self::Bang),
            Syntax::NotEqual => Ok(Self::BangEquals),
            Syntax::Or => Ok(Self::Pipe2),
            Syntax::Modulo => Ok(Self::Percent),
            Syntax::Add => Ok(Self::Plus),
            Syntax::Divide => Ok(Self::Slash),
            Syntax::Multiply => Ok(Self::Asterisk),

            Syntax::Comment => Ok(Self::Comment),
            Syntax::Newline => Ok(Self::Newline),
            Syntax::Whitespace => Ok(Self::Whitespace),

            Syntax::Unknown => Ok(Self::Unknown),

            _ => Err(()),
        }
    }
}
