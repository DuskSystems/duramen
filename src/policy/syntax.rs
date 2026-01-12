//! Cedar Policy Syntax.
//!
//! # References
//!
//! - [Syntax](https://docs.cedarpolicy.com/policies/syntax-policy.html)
//! - [Operators](https://docs.cedarpolicy.com/policies/syntax-operators.html)
//! - [Grammar](https://docs.cedarpolicy.com/policies/syntax-grammar.html)
//! - [LALRPOP](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/parser/grammar.lalrpop)

/// Syntax kinds for Cedar policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PolicySyntax {
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

    /// Keyword `template`.
    TemplateKeyword,

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
    /// `=>` symbol.
    FatArrow,
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

    /// Root node containing zero or more policies.
    ///
    /// ```cedar
    /// permit(principal, action, resource);
    /// forbid(principal, action, resource);
    /// ```
    PolicySet,

    /// Single policy statement.
    ///
    /// ```cedar
    /// @id("policy1")
    /// permit(principal, action, resource)
    /// when { principal.active };
    /// ```
    Policy,

    /// Annotation on a policy.
    ///
    /// ```cedar
    /// @id("policy1")
    /// @description("Allow active users")
    /// permit(principal, action, resource);
    /// ```
    Annotation,

    /// Scope variable definition (`principal`, `action`, or `resource` clause).
    ///
    /// ```cedar
    /// permit(
    ///     principal == User::"alice",
    ///     action in [Action::"read", Action::"write"],
    ///     resource is Document
    /// );
    /// ```
    VariableDefinition,

    /// Condition clause (`when` or `unless` with expression body).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.active && resource.public }
    /// unless { resource.classified };
    /// ```
    Condition,

    // Expression nodes
    /// Wrapper for a complete expression.
    Expression,

    /// Conditional expression.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { if principal.admin then true else false };
    /// ```
    IfExpression,

    /// Binary expression with operator token.
    ///
    /// Includes: `||` `&&` `==` `!=` `<` `<=` `>` `>=` `in` `+` `-` `*` `/` `%`
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.level >= 5 && resource.public };
    /// ```
    BinaryExpression,

    /// Has expression checking for attribute presence.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal has department };
    /// ```
    HasExpression,

    /// Like expression for pattern matching.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.email like "*@example.com" };
    /// ```
    LikeExpression,

    /// Is expression for type checking.
    ///
    /// ```cedar
    /// permit(principal is User, action, resource is Document);
    /// ```
    IsExpression,

    /// Unary expression with prefix operator (`!` or `-`).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { !resource.classified };
    /// ```
    UnaryExpression,

    /// Member access chain (field access, method call, or index access).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { resource.tags.contains("public") };
    /// ```
    MemberExpression,

    // Primary expressions
    /// Literal value (`true`, `false`, integers, strings).
    ///
    /// ```cedar
    /// permit(principal, action, resource) when { true };
    /// permit(principal, action, resource) when { context.count == 42 };
    /// ```
    LiteralExpression,

    /// Qualified path expression (type path without entity ID).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { Namespace::extension(principal) };
    /// ```
    PathExpression,

    /// Entity reference with type and string identifier.
    ///
    /// ```cedar
    /// permit(principal == User::"alice", action, resource);
    /// ```
    EntityReference,

    /// Template slot placeholder.
    ///
    /// ```cedar
    /// permit(principal == ?principal, action, resource == ?resource);
    /// ```
    SlotExpression,

    /// Parenthesized expression for grouping.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { (context.a || context.b) && context.c };
    /// ```
    ParenExpression,

    /// List expression.
    ///
    /// ```cedar
    /// permit(principal, action in [Action::"read", Action::"write"], resource);
    /// ```
    ListExpression,

    /// Record expression.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { { "name": "alice", "active": true } == context.user };
    /// ```
    RecordExpression,

    // Accessors (children of MemberExpression)
    /// Field access (`.identifier`).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.department == "engineering" };
    /// ```
    FieldAccess,

    /// Method call with arguments (`.method(args)`).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { resource.tags.contains("public") };
    /// ```
    MethodCall,

    /// Index access with string key (`["key"]`).
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { context["custom-header"] == "allowed" };
    /// ```
    IndexAccess,

    // Supporting structures
    /// Qualified name (`Foo::Bar::Baz`).
    Name,

    /// Single segment of a qualified name.
    PathSegment,

    /// Key-value pair in a record expression.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { { "name": "alice", "active": true } == context.user };
    /// ```
    RecordEntry,

    /// Parenthesized argument list for method/function calls.
    ArgumentList,

    // Experimental extensions
    /// Template declaration prefix.
    ///
    /// Declares template slots explicitly before the policy.
    ///
    /// ```cedar
    /// template(?principal, ?resource) =>
    /// permit(principal, action, resource);
    /// ```
    TemplateDeclaration,

    /// Slot definition in a template declaration.
    ///
    /// ```cedar
    /// template(?principal: User, ?resource: Document) =>
    /// permit(principal, action, resource);
    /// ```
    Slot,

    /// Type reference with optional generic parameters.
    ///
    /// ```cedar
    /// template(?principal: Namespace::User<Admin>) =>
    /// permit(principal, action, resource);
    /// ```
    TypeReference,

    /// Entity reference with record initializer.
    ///
    /// ```cedar
    /// permit(principal == User::{id: "1", email: "alice@example.com"}, action, resource);
    /// ```
    EntityRecord,

    /// Key-value pair in an entity record initializer.
    ///
    /// ```cedar
    /// permit(principal == User::{id: "1", name: "alice", active: true}, action, resource);
    /// ```
    RefInit,

    /// Unknown expression for partial evaluation and symbolic analysis.
    ///
    /// This node type is not produced by parsing Cedar source code directly.
    /// Instead, it is used during analysis phases to represent values that
    /// are not yet known (e.g., unlinked template slots, symbolic values).
    ///
    /// ```cedar
    /// // Not valid Cedar syntax - used internally for analysis
    /// permit(principal, action, resource) when { unknown("x") };
    /// ```
    UnknownExpression,

    /// Error recovery node wrapping invalid tokens during parsing.
    Error,
}

impl PolicySyntax {
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
            "template" => Some(Self::TemplateKeyword),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_trivial(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    #[must_use]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::TrueKeyword | Self::FalseKeyword | Self::Integer | Self::String
        )
    }

    #[must_use]
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::PermitKeyword
                | Self::ForbidKeyword
                | Self::PrincipalKeyword
                | Self::ActionKeyword
                | Self::ResourceKeyword
                | Self::ContextKeyword
                | Self::WhenKeyword
                | Self::UnlessKeyword
                | Self::TrueKeyword
                | Self::FalseKeyword
                | Self::LikeKeyword
                | Self::IfKeyword
                | Self::ThenKeyword
                | Self::ElseKeyword
                | Self::InKeyword
                | Self::HasKeyword
                | Self::IsKeyword
                | Self::TemplateKeyword
        )
    }

    #[must_use]
    pub const fn is_primitive(self) -> bool {
        matches!(self, Self::Identifier | Self::Integer | Self::String)
    }

    #[must_use]
    pub const fn is_token(self) -> bool {
        matches!(
            self,
            Self::Integer
                | Self::String
                | Self::Identifier
                | Self::PermitKeyword
                | Self::ForbidKeyword
                | Self::PrincipalKeyword
                | Self::ActionKeyword
                | Self::ResourceKeyword
                | Self::ContextKeyword
                | Self::WhenKeyword
                | Self::UnlessKeyword
                | Self::TrueKeyword
                | Self::FalseKeyword
                | Self::LikeKeyword
                | Self::IfKeyword
                | Self::ThenKeyword
                | Self::ElseKeyword
                | Self::InKeyword
                | Self::HasKeyword
                | Self::IsKeyword
                | Self::TemplateKeyword
                | Self::OpenParenthesis
                | Self::CloseParenthesis
                | Self::OpenBrace
                | Self::CloseBrace
                | Self::OpenBracket
                | Self::CloseBracket
                | Self::Comma
                | Self::Semicolon
                | Self::Colon
                | Self::Colon2
                | Self::At
                | Self::Dot
                | Self::Question
                | Self::Equal
                | Self::Equal2
                | Self::FatArrow
                | Self::Not
                | Self::NotEqual
                | Self::LessThan
                | Self::LessEqual
                | Self::GreaterThan
                | Self::GreaterEqual
                | Self::Ampersand
                | Self::Ampersand2
                | Self::Pipe
                | Self::Pipe2
                | Self::Plus
                | Self::Minus
                | Self::Asterisk
                | Self::Slash
                | Self::Percent
                | Self::Comment
                | Self::Whitespace
                | Self::Eof
                | Self::Unknown
        )
    }

    #[must_use]
    pub const fn is_node(self) -> bool {
        !self.is_token()
    }

    #[must_use]
    pub const fn expected_message(self) -> &'static str {
        match self {
            Self::OpenParenthesis => "expected `(`",
            Self::CloseParenthesis => "expected `)`",
            Self::OpenBrace => "expected `{`",
            Self::CloseBrace => "expected `}`",
            Self::OpenBracket => "expected `[`",
            Self::CloseBracket => "expected `]`",
            Self::Comma => "expected `,`",
            Self::Semicolon => "expected `;`",
            Self::Colon => "expected `:`",
            Self::Colon2 => "expected `::`",
            Self::At => "expected `@`",
            Self::Equal => "expected `=`",
            Self::Equal2 => "expected `==`",
            Self::ThenKeyword => "expected `then`",
            Self::ElseKeyword => "expected `else`",
            Self::Identifier => "expected identifier",
            Self::String => "expected string",
            Self::Integer => "expected integer",
            _ => "unexpected token",
        }
    }
}
