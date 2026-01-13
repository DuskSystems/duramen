use super::{
    AstNode, AstToken as _, BinaryOperator, IdentifierToken, IntegerToken, LiteralKind, Name,
    PolicyNode, SlotKind, StringToken, UnaryOperator, Variable,
};
use crate::policy::PolicySyntax;

/// Literal value expression (`true`, `false`, integers, strings).
///
/// ```cedar
/// permit(principal, action, resource)
/// when { context.count == 42 && context.enabled == true };
/// //                      ^^ integer literal
/// //                                               ^^^^ boolean literal
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LiteralExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for LiteralExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::LiteralExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> LiteralExpression<'a> {
    /// Returns the kind of literal (boolean, integer, or string).
    #[must_use]
    pub fn kind(&self) -> Option<LiteralKind> {
        let token = self.token()?;
        LiteralKind::from_kind(token.value())
    }

    /// Returns the underlying token node.
    #[must_use]
    pub fn token(&self) -> Option<PolicyNode<'a>> {
        self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::TrueKeyword
                    | PolicySyntax::FalseKeyword
                    | PolicySyntax::Integer
                    | PolicySyntax::String
            )
        })
    }

    /// Returns the boolean value if this is a `true` or `false` literal.
    ///
    /// ```cedar
    /// permit(principal, action, resource) when { true };
    /// //                                         ^^^^ as_bool returns Some(true)
    /// ```
    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        match self.token()?.value() {
            PolicySyntax::TrueKeyword => Some(true),
            PolicySyntax::FalseKeyword => Some(false),
            _ => None,
        }
    }

    /// Returns the integer token if this is an integer literal.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { context.level >= 5 };
    /// //                     ^ as_integer returns Some(IntegerToken)
    /// ```
    #[must_use]
    pub fn as_integer(&self) -> Option<IntegerToken<'a>> {
        let token = self.token()?;
        if token.value() == PolicySyntax::Integer {
            IntegerToken::cast(token)
        } else {
            None
        }
    }

    /// Returns the string token if this is a string literal.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { context.name == "alice" };
    /// //                     ^^^^^^^ as_string returns Some(StringToken)
    /// ```
    #[must_use]
    pub fn as_string(&self) -> Option<StringToken<'a>> {
        let token = self.token()?;
        if token.value() == PolicySyntax::String {
            StringToken::cast(token)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntityRefExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for EntityRefExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::EntityReference
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> EntityRefExpression<'a> {
    #[must_use]
    pub fn entity_type(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    #[must_use]
    pub fn entity_id(&self) -> Option<StringToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::String)
            .and_then(StringToken::cast)
    }
}

/// Template slot placeholder (`?principal` or `?resource`).
///
/// Slots are used in policy templates to create parameterized policies.
///
/// ```cedar
/// permit(principal == ?principal, action, resource == ?resource);
/// //                  ^^^^^^^^^^ slot for principal entity
/// //                                                  ^^^^^^^^^ slot for resource entity
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SlotExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for SlotExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::SlotExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> SlotExpression<'a> {
    /// Returns the identifier token following the `?`.
    ///
    /// This returns the raw identifier regardless of whether it's a valid slot name.
    /// Use `slot_kind()` to check if it's a recognized slot.
    ///
    /// ```cedar
    /// permit(principal == ?principal, action, resource);
    /// //                   ^^^^^^^^^ slot_id returns "principal"
    /// ```
    #[must_use]
    pub fn slot_id(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }

    /// Returns the slot kind if this is a valid slot (`?principal` or `?resource`).
    ///
    /// Returns `None` for invalid slots like `?foo`, which linters can detect
    /// by checking `slot_id()` when this returns `None`.
    ///
    /// ```cedar
    /// permit(principal == ?principal, action, resource);
    /// //                  ^^^^^^^^^^ slot_kind returns Some(SlotKind::Principal)
    /// permit(principal == ?foo, action, resource);
    /// //                  ^^^^ slot_kind returns None (invalid slot)
    /// ```
    #[must_use]
    pub fn slot_kind(&self, source: &str) -> Option<SlotKind> {
        let id = self.slot_id()?;
        SlotKind::from_text(id.text(source))
    }

    /// Returns `true` if this is a valid Cedar slot (`?principal` or `?resource`).
    #[must_use]
    pub fn is_valid(&self, source: &str) -> bool {
        self.slot_kind(source).is_some()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParenExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for ParenExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::ParenExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> ParenExpression<'a> {
    #[must_use]
    pub fn inner(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for ListExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::ListExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> ListExpression<'a> {
    pub fn elements(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node.children().filter_map(Expression::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecordExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for RecordExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::RecordExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> RecordExpression<'a> {
    pub fn entries(&self) -> impl Iterator<Item = RecordEntry<'a>> + use<'a> {
        self.node.children().filter_map(RecordEntry::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecordEntry<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for RecordEntry<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::RecordEntry
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> RecordEntry<'a> {
    #[must_use]
    pub fn key(&self) -> Option<AttrKey<'a>> {
        self.node.children().find_map(AttrKey::cast)
    }

    #[must_use]
    pub fn value(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnaryExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for UnaryExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::UnaryExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> UnaryExpression<'a> {
    #[must_use]
    pub fn operator(&self) -> Option<UnaryOperator> {
        let token = self.operator_token()?;
        UnaryOperator::from_kind(token.value())
    }

    #[must_use]
    pub fn operator_token(&self) -> Option<PolicyNode<'a>> {
        self.node
            .children()
            .find(|node| matches!(node.value(), PolicySyntax::Not | PolicySyntax::Minus))
    }

    #[must_use]
    pub fn operand(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for BinaryExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::BinaryExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> BinaryExpression<'a> {
    #[must_use]
    pub fn left(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn operator(&self) -> Option<BinaryOperator> {
        let token = self.operator_token()?;
        BinaryOperator::from_kind(token.value())
    }

    #[must_use]
    pub fn operator_token(&self) -> Option<PolicyNode<'a>> {
        self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::Pipe2
                    | PolicySyntax::Ampersand2
                    | PolicySyntax::Equal2
                    | PolicySyntax::NotEqual
                    | PolicySyntax::LessThan
                    | PolicySyntax::LessEqual
                    | PolicySyntax::GreaterThan
                    | PolicySyntax::GreaterEqual
                    | PolicySyntax::InKeyword
                    | PolicySyntax::Plus
                    | PolicySyntax::Minus
                    | PolicySyntax::Asterisk
                    | PolicySyntax::Pipe
                    | PolicySyntax::Ampersand
                    | PolicySyntax::Equal
                    | PolicySyntax::Slash
                    | PolicySyntax::Percent
            )
        })
    }

    #[must_use]
    pub fn right(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HasExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for HasExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::HasExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> HasExpression<'a> {
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn attribute(&self) -> Option<AttrKey<'a>> {
        self.node.children().find_map(AttrKey::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AttrKey<'a> {
    Identifier(IdentifierToken<'a>),
    String(StringToken<'a>),
}

impl<'a> AttrKey<'a> {
    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        match node.value() {
            PolicySyntax::Identifier => IdentifierToken::cast(node).map(Self::Identifier),
            PolicySyntax::String => StringToken::cast(node).map(Self::String),
            _ => None,
        }
    }

    #[must_use]
    pub fn syntax(&self) -> &PolicyNode<'a> {
        match self {
            Self::Identifier(token) => token.syntax(),
            Self::String(token) => token.syntax(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LikeExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for LikeExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::LikeExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> LikeExpression<'a> {
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn pattern(&self) -> Option<StringToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::String)
            .and_then(StringToken::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IsExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for IsExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::IsExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> IsExpression<'a> {
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn entity_type(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    #[must_use]
    pub fn in_entity(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IfExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for IfExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::IfExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> IfExpression<'a> {
    #[must_use]
    pub fn condition(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn then_branch(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }

    #[must_use]
    pub fn else_branch(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FieldExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for FieldExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::FieldAccess
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> FieldExpression<'a> {
    #[must_use]
    pub fn receiver(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn field(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MethodCallExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for MethodCallExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::MethodCall
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> MethodCallExpression<'a> {
    #[must_use]
    pub fn receiver(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn method(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }

    pub fn arguments(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::ArgumentList)
            .into_iter()
            .flat_map(|args| args.children().filter_map(Expression::cast))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IndexExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for IndexExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::IndexAccess
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> IndexExpression<'a> {
    #[must_use]
    pub fn receiver(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn index(&self) -> Option<Expression<'a>> {
        self.node.children().filter_map(Expression::cast).nth(1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PathExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for PathExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::PathExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> PathExpression<'a> {
    /// Returns the qualified name path.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { Namespace::Type::function(principal) };
    /// //     ^^^^^^^^^^^^^^^^^^^^^^^^^ path
    /// ```
    #[must_use]
    pub fn path(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns the variable if this path is a simple variable reference.
    ///
    /// Returns `None` if the path has multiple segments or isn't a recognized variable.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.active };
    /// //     ^^^^^^^^^ as_variable returns Some(Variable::Principal)
    /// ```
    #[must_use]
    pub fn as_variable(&self, source: &str) -> Option<Variable> {
        let name = self.path()?;
        let mut segments = name.segments();
        let first = segments.next()?;
        if segments.next().is_some() {
            return None;
        }
        Variable::from_text(first.text(source))
    }

    /// Returns `true` if this is a qualified path with multiple segments.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { Namespace::extension(principal) };
    /// //     ^^^^^^^^^^^^^^^^^^^^ is_qualified returns true
    /// //     principal            is_qualified returns false
    /// ```
    #[must_use]
    pub fn is_qualified(&self) -> bool {
        self.path().is_some_and(|name| {
            let mut segments = name.segments();
            segments.next().is_some() && segments.next().is_some()
        })
    }
}

/// Unknown expression for partial evaluation and symbolic analysis.
///
/// This node type is not produced by parsing Cedar source code directly.
/// Instead, it is used during analysis phases to represent values that
/// are not yet known (e.g., unlinked template slots, symbolic values).
///
/// Use cases:
/// - **Type checking**: Propagating unknown types through expressions
/// - **Symbolic analysis**: Reasoning about policies with unbound variables
/// - **IDE features**: Representing incomplete expressions during editing
#[derive(Debug, Clone, Copy)]
pub struct UnknownExpression<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for UnknownExpression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::UnknownExpression
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> UnknownExpression<'a> {
    /// Returns the name identifying this unknown, if present.
    ///
    /// The name typically describes what value is unknown, such as
    /// a variable name or attribute path.
    #[must_use]
    pub fn name(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Expression<'a> {
    Literal(LiteralExpression<'a>),
    EntityRef(EntityRefExpression<'a>),
    Slot(SlotExpression<'a>),
    Paren(ParenExpression<'a>),
    List(ListExpression<'a>),
    Record(RecordExpression<'a>),
    Unary(UnaryExpression<'a>),
    Binary(BinaryExpression<'a>),
    Has(HasExpression<'a>),
    Like(LikeExpression<'a>),
    Is(IsExpression<'a>),
    If(IfExpression<'a>),
    Field(FieldExpression<'a>),
    MethodCall(MethodCallExpression<'a>),
    Index(IndexExpression<'a>),
    Path(PathExpression<'a>),
    Unknown(UnknownExpression<'a>),
}

impl<'a> AstNode<'a> for Expression<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        matches!(
            kind,
            PolicySyntax::LiteralExpression
                | PolicySyntax::EntityReference
                | PolicySyntax::SlotExpression
                | PolicySyntax::ParenExpression
                | PolicySyntax::ListExpression
                | PolicySyntax::RecordExpression
                | PolicySyntax::UnaryExpression
                | PolicySyntax::BinaryExpression
                | PolicySyntax::HasExpression
                | PolicySyntax::LikeExpression
                | PolicySyntax::IsExpression
                | PolicySyntax::IfExpression
                | PolicySyntax::FieldAccess
                | PolicySyntax::MethodCall
                | PolicySyntax::IndexAccess
                | PolicySyntax::PathExpression
                | PolicySyntax::UnknownExpression
        )
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        match node.value() {
            PolicySyntax::LiteralExpression => LiteralExpression::cast(node).map(Self::Literal),
            PolicySyntax::EntityReference => EntityRefExpression::cast(node).map(Self::EntityRef),
            PolicySyntax::SlotExpression => SlotExpression::cast(node).map(Self::Slot),
            PolicySyntax::ParenExpression => ParenExpression::cast(node).map(Self::Paren),
            PolicySyntax::ListExpression => ListExpression::cast(node).map(Self::List),
            PolicySyntax::RecordExpression => RecordExpression::cast(node).map(Self::Record),
            PolicySyntax::UnaryExpression => UnaryExpression::cast(node).map(Self::Unary),
            PolicySyntax::BinaryExpression => BinaryExpression::cast(node).map(Self::Binary),
            PolicySyntax::HasExpression => HasExpression::cast(node).map(Self::Has),
            PolicySyntax::LikeExpression => LikeExpression::cast(node).map(Self::Like),
            PolicySyntax::IsExpression => IsExpression::cast(node).map(Self::Is),
            PolicySyntax::IfExpression => IfExpression::cast(node).map(Self::If),
            PolicySyntax::FieldAccess => FieldExpression::cast(node).map(Self::Field),
            PolicySyntax::MethodCall => MethodCallExpression::cast(node).map(Self::MethodCall),
            PolicySyntax::IndexAccess => IndexExpression::cast(node).map(Self::Index),
            PolicySyntax::PathExpression => PathExpression::cast(node).map(Self::Path),
            PolicySyntax::UnknownExpression => UnknownExpression::cast(node).map(Self::Unknown),
            _ => None,
        }
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        match self {
            Self::Literal(inner) => inner.syntax(),
            Self::EntityRef(inner) => inner.syntax(),
            Self::Slot(inner) => inner.syntax(),
            Self::Paren(inner) => inner.syntax(),
            Self::List(inner) => inner.syntax(),
            Self::Record(inner) => inner.syntax(),
            Self::Unary(inner) => inner.syntax(),
            Self::Binary(inner) => inner.syntax(),
            Self::Has(inner) => inner.syntax(),
            Self::Like(inner) => inner.syntax(),
            Self::Is(inner) => inner.syntax(),
            Self::If(inner) => inner.syntax(),
            Self::Field(inner) => inner.syntax(),
            Self::MethodCall(inner) => inner.syntax(),
            Self::Index(inner) => inner.syntax(),
            Self::Path(inner) => inner.syntax(),
            Self::Unknown(inner) => inner.syntax(),
        }
    }
}

/// An accessor in a member access chain (field, method call, or index).
///
/// This enum is returned by [`Expression::as_accessor`] to provide a unified view of the different accessor types.
#[derive(Debug, Clone, Copy)]
pub enum Accessor<'a> {
    /// Field access (`.identifier`).
    Field(FieldExpression<'a>),
    /// Method call (`.method(args)`).
    Method(MethodCallExpression<'a>),
    /// Index access (`["key"]`).
    Index(IndexExpression<'a>),
}

impl<'a> Accessor<'a> {
    /// Returns the receiver expression that this accessor operates on.
    #[must_use]
    pub fn receiver(&self) -> Option<Expression<'a>> {
        match self {
            Self::Field(expr) => expr.receiver(),
            Self::Method(expr) => expr.receiver(),
            Self::Index(expr) => expr.receiver(),
        }
    }
}

impl<'a> Expression<'a> {
    /// Returns the receiver expression if this is an accessor (field, method, or index).
    ///
    /// For chained access like `principal.department.name`, each accessor has a receiver:
    /// - `principal.department.name` → receiver is `principal.department`
    /// - `principal.department` → receiver is `principal`
    /// - `principal` → returns `None` (not an accessor)
    #[must_use]
    pub fn receiver(&self) -> Option<Self> {
        match self {
            Self::Field(expr) => expr.receiver(),
            Self::MethodCall(expr) => expr.receiver(),
            Self::Index(expr) => expr.receiver(),
            _ => None,
        }
    }

    /// Returns this expression as an accessor, if applicable.
    ///
    /// Returns `Some(Accessor)` for field access, method calls, and index access.
    /// Returns `None` for all other expression types.
    #[must_use]
    pub const fn as_accessor(&self) -> Option<Accessor<'a>> {
        match self {
            Self::Field(expr) => Some(Accessor::Field(*expr)),
            Self::MethodCall(expr) => Some(Accessor::Method(*expr)),
            Self::Index(expr) => Some(Accessor::Index(*expr)),
            _ => None,
        }
    }

    /// Returns `true` if this expression is an accessor (field, method, or index).
    #[must_use]
    pub const fn is_accessor(&self) -> bool {
        matches!(self, Self::Field(_) | Self::MethodCall(_) | Self::Index(_))
    }
}
