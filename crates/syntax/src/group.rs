use core::fmt;

/// Group syntax kinds (groups of tokens).
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum Group {
    /// Error recovery wrapper.
    Error,

    /// Annotation: `@id("value")`.
    Annotation,
    /// Qualified name: `Namespace::Type`.
    Name,

    /// Root node for policies.
    Policies,
    /// Policy statement.
    Policy,
    /// Scope: `(principal, action, resource)`.
    Scope,
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

impl Group {
    /// Checks if this is an error group.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Error => "error",

            Self::Annotation => "annotation",
            Self::Name => "name",

            Self::Policies => "policies",
            Self::Policy => "policy",
            Self::Scope => "scope",
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
