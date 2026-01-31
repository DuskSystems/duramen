use core::ops::Range;

use syntree::{Builder, FlavorDefault, Node, Tree};

use crate::CstNode;

mod syntax;
pub use syntax::PolicySyntax;

pub type PolicyTree = Tree<PolicySyntax, FlavorDefault>;
pub type PolicyBuilder = Builder<PolicySyntax>;
pub type PolicyNode<'a> = Node<'a, PolicySyntax, FlavorDefault>;

macro_rules! cst_node {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name<'a> {
            node: PolicyNode<'a>,
        }

        impl<'a> CstNode<'a> for $name<'a> {
            type Syntax = PolicySyntax;

            fn can_cast(kind: PolicySyntax) -> bool {
                kind == $kind
            }

            fn cast(node: PolicyNode<'a>) -> Option<Self> {
                Self::can_cast(node.value()).then_some(Self { node })
            }

            fn syntax(&self) -> PolicyNode<'a> {
                self.node
            }
        }
    };
}

cst_node!(Policies, PolicySyntax::Policies);
impl<'a> Policies<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Policy<'a>> + use<'a> {
        self.node.children().skip_tokens().filter_map(Policy::cast)
    }
}

cst_node!(Policy, PolicySyntax::Policy);
impl<'a> Policy<'a> {
    #[must_use]
    pub fn effect(&self) -> Option<Effect> {
        self.node.children().find_map(|child| match child.value() {
            PolicySyntax::Permit => Some(Effect::Permit),
            PolicySyntax::Forbid => Some(Effect::Forbid),
            _ => None,
        })
    }

    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
        self.node.children().filter_map(Annotation::cast)
    }

    pub fn variables(&self) -> impl Iterator<Item = VariableDef<'a>> + use<'a> {
        self.node.children().filter_map(VariableDef::cast)
    }

    pub fn conditions(&self) -> impl Iterator<Item = Condition<'a>> + use<'a> {
        self.node.children().filter_map(Condition::cast)
    }
}

cst_node!(Annotation, PolicySyntax::Annotation);
impl Annotation<'_> {
    #[must_use]
    pub fn name<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|node| node.value().is_identifier())
            .map(|node| &source[node.range()])
    }

    #[must_use]
    pub fn value<'s>(&self, source: &'s str) -> Option<&'s str> {
        let child = self
            .node
            .children()
            .find(|child| child.value() == PolicySyntax::String)?;

        let text = &source[child.range()];
        text.get(1..text.len().saturating_sub(1))
    }
}

cst_node!(VariableDef, PolicySyntax::VariableDef);
impl<'a> VariableDef<'a> {
    #[must_use]
    pub fn variable(&self) -> Option<Variable> {
        self.node
            .children()
            .find_map(|child| Variable::from_kind(child.value()))
    }

    #[must_use]
    pub fn entity_type(&self) -> Option<Name<'a>> {
        let mut found = false;
        for child in self.node.children() {
            if child.value() == PolicySyntax::Is {
                found = true;
            } else if found && let Some(name) = Name::cast(child) {
                return Some(name);
            }
        }

        None
    }

    #[must_use]
    pub fn constraint(&self) -> Option<Expression<'a>> {
        let mut found_operator = false;
        for child in self.node.children() {
            match child.value() {
                PolicySyntax::Eq2 | PolicySyntax::In => {
                    found_operator = true;
                }
                _ if found_operator => {
                    if let Some(expr) = Expression::cast(child) {
                        return Some(expr);
                    }
                }
                _ => {}
            }
        }
        None
    }

    #[must_use]
    pub fn operator(&self) -> Option<RelOp> {
        self.node.children().find_map(|child| match child.value() {
            PolicySyntax::Eq2 => Some(RelOp::Eq),
            PolicySyntax::In => Some(RelOp::In),
            _ => None,
        })
    }
}

cst_node!(Condition, PolicySyntax::Condition);
impl<'a> Condition<'a> {
    #[must_use]
    pub fn kind(&self) -> Option<ConditionKind> {
        self.node.children().find_map(|child| match child.value() {
            PolicySyntax::When => Some(ConditionKind::When),
            PolicySyntax::Unless => Some(ConditionKind::Unless),
            _ => None,
        })
    }

    #[must_use]
    pub fn expr(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

cst_node!(Name, PolicySyntax::Name);
impl<'a> Name<'a> {
    pub fn segments<'s>(&self, source: &'s str) -> impl Iterator<Item = &'s str> + use<'a, 's> {
        self.node
            .children()
            .filter(|node| node.value().is_identifier())
            .map(|node| &source[node.range()])
    }

    #[must_use]
    pub fn is_qualified(&self) -> bool {
        self.node
            .children()
            .filter(|node| node.value().is_identifier())
            .nth(1)
            .is_some()
    }

    #[must_use]
    pub fn basename<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .filter(|node| node.value().is_identifier())
            .last()
            .map(|node| &source[node.range()])
    }

    pub fn namespace<'s>(&self, source: &'s str) -> impl Iterator<Item = &'s str> + use<'a, 's> {
        let segments: alloc::vec::Vec<_> = self
            .node
            .children()
            .filter(|node| node.value().is_identifier())
            .collect();

        let count = segments.len().saturating_sub(1);
        segments
            .into_iter()
            .take(count)
            .map(|node| &source[node.range()])
    }

    #[must_use]
    pub fn has_reserved_segment(&self) -> bool {
        self.node
            .children()
            .any(|node| node.value().is_reserved_word())
    }

    #[must_use]
    pub fn first_reserved_segment<'s>(&self, source: &'s str) -> Option<(&'s str, Range<usize>)> {
        self.node
            .children()
            .find(|node| node.value().is_reserved_word())
            .map(|node| (&source[node.range()], node.range()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Permit,
    Forbid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionKind {
    When,
    Unless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variable {
    Principal,
    Action,
    Resource,
    Context,
}

impl Variable {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Principal => Some(Self::Principal),
            PolicySyntax::Action => Some(Self::Action),
            PolicySyntax::Resource => Some(Self::Resource),
            PolicySyntax::Context => Some(Self::Context),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Principal => "principal",
            Self::Action => "action",
            Self::Resource => "resource",
            Self::Context => "context",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelOp {
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Eq,
    NotEq,
    In,
}

impl RelOp {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Lt => Some(Self::Less),
            PolicySyntax::LtEq => Some(Self::LessEq),
            PolicySyntax::Gt => Some(Self::Greater),
            PolicySyntax::GtEq => Some(Self::GreaterEq),
            PolicySyntax::Eq2 => Some(Self::Eq),
            PolicySyntax::BangEq => Some(Self::NotEq),
            PolicySyntax::In => Some(Self::In),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddOp {
    Plus,
    Minus,
}

impl AddOp {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Plus => Some(Self::Plus),
            PolicySyntax::Minus => Some(Self::Minus),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MulOp {
    Times,
    Divide,
    Mod,
}

impl MulOp {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Star => Some(Self::Times),
            PolicySyntax::Slash => Some(Self::Divide),
            PolicySyntax::Percent => Some(Self::Mod),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Not,
    Neg,
}

impl UnaryOp {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Bang => Some(Self::Not),
            PolicySyntax::Minus => Some(Self::Neg),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Bool(bool),
    Int,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotKind {
    Principal,
    Resource,
    Other,
}

#[derive(Debug, Clone, Copy)]
pub enum Expression<'a> {
    If(IfExpression<'a>),
    Or(OrExpression<'a>),
    And(AndExpression<'a>),
    Relation(RelationExpression<'a>),
    Sum(SumExpression<'a>),
    Product(ProductExpression<'a>),
    Has(HasExpression<'a>),
    Like(LikeExpression<'a>),
    Is(IsExpression<'a>),
    Unary(UnaryExpression<'a>),
    Member(MemberExpression<'a>),
    Literal(LiteralExpression<'a>),
    EntityRef(EntityRefExpression<'a>),
    Slot(SlotExpression<'a>),
    Paren(ParenExpression<'a>),
    List(ListExpression<'a>),
    Record(RecordExpression<'a>),
    Name(Name<'a>),
}

impl<'a> CstNode<'a> for Expression<'a> {
    type Syntax = PolicySyntax;

    fn can_cast(kind: PolicySyntax) -> bool {
        matches!(
            kind,
            PolicySyntax::IfExpression
                | PolicySyntax::OrExpression
                | PolicySyntax::AndExpression
                | PolicySyntax::Relation
                | PolicySyntax::Sum
                | PolicySyntax::Product
                | PolicySyntax::HasExpression
                | PolicySyntax::LikeExpression
                | PolicySyntax::IsExpression
                | PolicySyntax::Unary
                | PolicySyntax::Member
                | PolicySyntax::Literal
                | PolicySyntax::EntityReference
                | PolicySyntax::Slot
                | PolicySyntax::Parenthesized
                | PolicySyntax::List
                | PolicySyntax::Record
                | PolicySyntax::Name
        )
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        match node.value() {
            PolicySyntax::IfExpression => IfExpression::cast(node).map(Self::If),
            PolicySyntax::OrExpression => OrExpression::cast(node).map(Self::Or),
            PolicySyntax::AndExpression => AndExpression::cast(node).map(Self::And),
            PolicySyntax::Relation => RelationExpression::cast(node).map(Self::Relation),
            PolicySyntax::Sum => SumExpression::cast(node).map(Self::Sum),
            PolicySyntax::Product => ProductExpression::cast(node).map(Self::Product),
            PolicySyntax::HasExpression => HasExpression::cast(node).map(Self::Has),
            PolicySyntax::LikeExpression => LikeExpression::cast(node).map(Self::Like),
            PolicySyntax::IsExpression => IsExpression::cast(node).map(Self::Is),
            PolicySyntax::Unary => UnaryExpression::cast(node).map(Self::Unary),
            PolicySyntax::Member => MemberExpression::cast(node).map(Self::Member),
            PolicySyntax::Literal => LiteralExpression::cast(node).map(Self::Literal),
            PolicySyntax::EntityReference => EntityRefExpression::cast(node).map(Self::EntityRef),
            PolicySyntax::Slot => SlotExpression::cast(node).map(Self::Slot),
            PolicySyntax::Parenthesized => ParenExpression::cast(node).map(Self::Paren),
            PolicySyntax::List => ListExpression::cast(node).map(Self::List),
            PolicySyntax::Record => RecordExpression::cast(node).map(Self::Record),
            PolicySyntax::Name => Name::cast(node).map(Self::Name),
            _ => None,
        }
    }

    fn syntax(&self) -> PolicyNode<'a> {
        match self {
            Self::If(expr) => expr.syntax(),
            Self::Or(expr) => expr.syntax(),
            Self::And(expr) => expr.syntax(),
            Self::Relation(expr) => expr.syntax(),
            Self::Sum(expr) => expr.syntax(),
            Self::Product(expr) => expr.syntax(),
            Self::Has(expr) => expr.syntax(),
            Self::Like(expr) => expr.syntax(),
            Self::Is(expr) => expr.syntax(),
            Self::Unary(expr) => expr.syntax(),
            Self::Member(expr) => expr.syntax(),
            Self::Literal(expr) => expr.syntax(),
            Self::EntityRef(expr) => expr.syntax(),
            Self::Slot(expr) => expr.syntax(),
            Self::Paren(expr) => expr.syntax(),
            Self::List(expr) => expr.syntax(),
            Self::Record(expr) => expr.syntax(),
            Self::Name(name) => name.syntax(),
        }
    }
}

cst_node!(IfExpression, PolicySyntax::IfExpression);
impl<'a> IfExpression<'a> {
    #[must_use]
    pub fn condition(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }

    #[must_use]
    pub fn then_expr(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
            .nth(1)
    }

    #[must_use]
    pub fn else_expr(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
            .nth(2)
    }
}

cst_node!(OrExpression, PolicySyntax::OrExpression);
impl<'a> OrExpression<'a> {
    pub fn operands(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
    }
}

cst_node!(AndExpression, PolicySyntax::AndExpression);
impl<'a> AndExpression<'a> {
    pub fn operands(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
    }
}

cst_node!(RelationExpression, PolicySyntax::Relation);
impl<'a> RelationExpression<'a> {
    #[must_use]
    pub fn left(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }

    #[must_use]
    pub fn operator(&self) -> Option<RelOp> {
        self.node
            .children()
            .find_map(|child| RelOp::from_kind(child.value()))
    }

    #[must_use]
    pub fn right(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
            .nth(1)
    }
}

cst_node!(SumExpression, PolicySyntax::Sum);
impl<'a> SumExpression<'a> {
    pub fn operands(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
    }

    pub fn operators(&self) -> impl Iterator<Item = AddOp> + use<'a> {
        self.node
            .children()
            .filter_map(|child| AddOp::from_kind(child.value()))
    }
}

cst_node!(ProductExpression, PolicySyntax::Product);
impl<'a> ProductExpression<'a> {
    pub fn operands(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
    }

    pub fn operators(&self) -> impl Iterator<Item = MulOp> + use<'a> {
        self.node
            .children()
            .filter_map(|child| MulOp::from_kind(child.value()))
    }
}

cst_node!(HasExpression, PolicySyntax::HasExpression);
impl<'a> HasExpression<'a> {
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }

    #[must_use]
    pub fn field<'s>(&self, source: &'s str) -> Option<(&'s str, bool)> {
        self.node
            .children()
            .find(|child| {
                child.value().is_token()
                    && !child.value().is_trivial()
                    && child.value() != PolicySyntax::Has
            })
            .map(|child| {
                let text = &source[child.range()];
                if child.value() == PolicySyntax::String {
                    (
                        text.get(1..text.len().saturating_sub(1)).unwrap_or(text),
                        true,
                    )
                } else {
                    (text, false)
                }
            })
    }

    #[must_use]
    pub fn is_field_reserved(&self) -> bool {
        self.node
            .children()
            .find(|child| {
                child.value().is_token()
                    && !child.value().is_trivial()
                    && child.value() != PolicySyntax::Has
            })
            .is_some_and(|child| child.value().is_reserved_word())
    }
}

cst_node!(LikeExpression, PolicySyntax::LikeExpression);
impl<'a> LikeExpression<'a> {
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }

    #[must_use]
    pub fn pattern<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| child.value() == PolicySyntax::String)
            .map(|child| {
                let text = &source[child.range()];
                text.get(1..text.len().saturating_sub(1)).unwrap_or(text)
            })
    }
}

cst_node!(IsExpression, PolicySyntax::IsExpression);
impl<'a> IsExpression<'a> {
    #[must_use]
    pub fn target(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }

    #[must_use]
    pub fn entity_type(&self) -> Option<Name<'a>> {
        let mut found_is = false;
        for child in self.node.children() {
            if child.value() == PolicySyntax::Is {
                found_is = true;
            } else if found_is && let Some(name) = Name::cast(child) {
                return Some(name);
            }
        }
        None
    }

    #[must_use]
    pub fn in_expr(&self) -> Option<Expression<'a>> {
        let mut found_in = false;
        for child in self.node.children() {
            if child.value() == PolicySyntax::In {
                found_in = true;
            } else if found_in && let Some(expr) = Expression::cast(child) {
                return Some(expr);
            }
        }

        None
    }
}

cst_node!(UnaryExpression, PolicySyntax::Unary);
impl<'a> UnaryExpression<'a> {
    pub fn operators(&self) -> impl Iterator<Item = UnaryOp> + use<'a> {
        self.node
            .children()
            .filter_map(|child| UnaryOp::from_kind(child.value()))
    }

    #[must_use]
    pub fn operand(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }
}

cst_node!(MemberExpression, PolicySyntax::Member);
impl<'a> MemberExpression<'a> {
    #[must_use]
    pub fn base(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }

    pub fn accesses(&self) -> impl Iterator<Item = MemberAccess<'a>> + use<'a> {
        self.node.children().filter_map(MemberAccess::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MemberAccess<'a> {
    Field(FieldAccess<'a>),
    Call(MethodCall<'a>),
    Index(IndexAccess<'a>),
}

impl<'a> CstNode<'a> for MemberAccess<'a> {
    type Syntax = PolicySyntax;

    fn can_cast(kind: PolicySyntax) -> bool {
        matches!(
            kind,
            PolicySyntax::FieldAccess | PolicySyntax::MethodCall | PolicySyntax::IndexAccess
        )
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        match node.value() {
            PolicySyntax::FieldAccess => FieldAccess::cast(node).map(Self::Field),
            PolicySyntax::MethodCall => MethodCall::cast(node).map(Self::Call),
            PolicySyntax::IndexAccess => IndexAccess::cast(node).map(Self::Index),
            _ => None,
        }
    }

    fn syntax(&self) -> PolicyNode<'a> {
        match self {
            Self::Field(access) => access.syntax(),
            Self::Call(access) => access.syntax(),
            Self::Index(access) => access.syntax(),
        }
    }
}

cst_node!(FieldAccess, PolicySyntax::FieldAccess);
impl FieldAccess<'_> {
    #[must_use]
    pub fn field<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| {
                child.value().is_token()
                    && !child.value().is_trivial()
                    && child.value() != PolicySyntax::Dot
            })
            .map(|child| &source[child.range()])
    }

    #[must_use]
    pub fn is_field_reserved(&self) -> bool {
        self.node
            .children()
            .find(|child| {
                child.value().is_token()
                    && !child.value().is_trivial()
                    && child.value() != PolicySyntax::Dot
            })
            .is_some_and(|child| child.value().is_reserved_word())
    }
}

cst_node!(MethodCall, PolicySyntax::MethodCall);
impl<'a> MethodCall<'a> {
    #[must_use]
    pub fn name<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| child.value() == PolicySyntax::Identifier || child.value().is_keyword())
            .map(|child| &source[child.range()])
    }

    pub fn arguments(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .find_map(ArgumentList::cast)
            .into_iter()
            .flat_map(|args| args.iter())
    }
}

cst_node!(IndexAccess, PolicySyntax::IndexAccess);
impl<'a> IndexAccess<'a> {
    #[must_use]
    pub fn index(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }
}

cst_node!(LiteralExpression, PolicySyntax::Literal);
impl LiteralExpression<'_> {
    #[must_use]
    pub fn kind(&self) -> Option<LiteralKind> {
        self.node.children().find_map(|child| match child.value() {
            PolicySyntax::True => Some(LiteralKind::Bool(true)),
            PolicySyntax::False => Some(LiteralKind::Bool(false)),
            PolicySyntax::Integer => Some(LiteralKind::Int),
            PolicySyntax::String => Some(LiteralKind::String),
            _ => None,
        })
    }

    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        self.node.children().find_map(|child| match child.value() {
            PolicySyntax::True => Some(true),
            PolicySyntax::False => Some(false),
            _ => None,
        })
    }

    #[must_use]
    pub fn as_int<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| child.value() == PolicySyntax::Integer)
            .map(|child| &source[child.range()])
    }

    #[must_use]
    pub fn as_string<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| child.value() == PolicySyntax::String)
            .map(|child| {
                let text = &source[child.range()];
                text.get(1..text.len().saturating_sub(1)).unwrap_or(text)
            })
    }
}

cst_node!(EntityRefExpression, PolicySyntax::EntityReference);
impl<'a> EntityRefExpression<'a> {
    #[must_use]
    pub fn type_name(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    #[must_use]
    pub fn id<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| child.value() == PolicySyntax::String)
            .map(|child| {
                let text = &source[child.range()];
                text.get(1..text.len().saturating_sub(1)).unwrap_or(text)
            })
    }

    pub fn entries(&self) -> impl Iterator<Item = RecordEntry<'a>> + use<'a> {
        self.node.children().filter_map(RecordEntry::cast)
    }
}

cst_node!(SlotExpression, PolicySyntax::Slot);
impl SlotExpression<'_> {
    #[must_use]
    pub fn kind(&self) -> Option<SlotKind> {
        self.node.children().find_map(|child| match child.value() {
            PolicySyntax::Principal => Some(SlotKind::Principal),
            PolicySyntax::Resource => Some(SlotKind::Resource),
            PolicySyntax::Identifier => Some(SlotKind::Other),
            _ => None,
        })
    }

    #[must_use]
    pub fn name<'s>(&self, source: &'s str) -> Option<&'s str> {
        self.node
            .children()
            .find(|child| {
                matches!(
                    child.value(),
                    PolicySyntax::Identifier | PolicySyntax::Principal | PolicySyntax::Resource
                )
            })
            .map(|child| &source[child.range()])
    }
}

cst_node!(ParenExpression, PolicySyntax::Parenthesized);
impl<'a> ParenExpression<'a> {
    #[must_use]
    pub fn inner(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }
}

cst_node!(ListExpression, PolicySyntax::List);
impl<'a> ListExpression<'a> {
    pub fn elements(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .find_map(ArgumentList::cast)
            .into_iter()
            .flat_map(|args| args.iter())
    }
}

cst_node!(RecordExpression, PolicySyntax::Record);
impl<'a> RecordExpression<'a> {
    pub fn entries(&self) -> impl Iterator<Item = RecordEntry<'a>> + use<'a> {
        self.node.children().filter_map(RecordEntry::cast)
    }
}

cst_node!(RecordEntry, PolicySyntax::RecordEntry);
impl<'a> RecordEntry<'a> {
    #[must_use]
    pub fn key<'s>(&self, source: &'s str) -> Option<(&'s str, bool)> {
        self.node
            .children()
            .find(|child| child.value().is_token() && !child.value().is_trivial())
            .map(|child| {
                let text = &source[child.range()];
                if child.value() == PolicySyntax::String {
                    (
                        text.get(1..text.len().saturating_sub(1)).unwrap_or(text),
                        true,
                    )
                } else {
                    (text, false)
                }
            })
    }

    #[must_use]
    pub fn is_key_reserved(&self) -> bool {
        self.node
            .children()
            .find(|child| child.value().is_token() && !child.value().is_trivial())
            .is_some_and(|child| child.value().is_reserved_word())
    }

    #[must_use]
    pub fn value(&self) -> Option<Expression<'a>> {
        self.node
            .children()
            .skip_tokens()
            .find_map(Expression::cast)
    }
}

cst_node!(ArgumentList, PolicySyntax::ArgumentList);
impl<'a> ArgumentList<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        self.node
            .children()
            .skip_tokens()
            .filter_map(Expression::cast)
    }
}
