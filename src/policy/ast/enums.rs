use crate::policy::PolicySyntax;

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
            PolicySyntax::PrincipalKeyword => Some(Self::Principal),
            PolicySyntax::ActionKeyword => Some(Self::Action),
            PolicySyntax::ResourceKeyword => Some(Self::Resource),
            PolicySyntax::ContextKeyword => Some(Self::Context),
            _ => None,
        }
    }

    #[must_use]
    pub fn from_text(value: &str) -> Option<Self> {
        match value {
            "principal" => Some(Self::Principal),
            "action" => Some(Self::Action),
            "resource" => Some(Self::Resource),
            "context" => Some(Self::Context),
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
pub enum Effect {
    Permit,
    Forbid,
}

impl Effect {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::PermitKeyword => Some(Self::Permit),
            PolicySyntax::ForbidKeyword => Some(Self::Forbid),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permit => "permit",
            Self::Forbid => "forbid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionKind {
    When,
    Unless,
}

impl ConditionKind {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::WhenKeyword => Some(Self::When),
            PolicySyntax::UnlessKeyword => Some(Self::Unless),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::When => "when",
            Self::Unless => "unless",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
    Neg,
}

impl UnaryOperator {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Not => Some(Self::Not),
            PolicySyntax::Minus => Some(Self::Neg),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Not => "!",
            Self::Neg => "-",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Or,
    And,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    In,
    Add,
    Sub,
    Mul,
}

impl BinaryOperator {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::Pipe2 => Some(Self::Or),
            PolicySyntax::Ampersand2 => Some(Self::And),
            PolicySyntax::Equal2 => Some(Self::Eq),
            PolicySyntax::NotEqual => Some(Self::Neq),
            PolicySyntax::LessThan => Some(Self::Lt),
            PolicySyntax::LessEqual => Some(Self::Lte),
            PolicySyntax::GreaterThan => Some(Self::Gt),
            PolicySyntax::GreaterEqual => Some(Self::Gte),
            PolicySyntax::InKeyword => Some(Self::In),
            PolicySyntax::Plus => Some(Self::Add),
            PolicySyntax::Minus => Some(Self::Sub),
            PolicySyntax::Asterisk => Some(Self::Mul),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Or => "||",
            Self::And => "&&",
            Self::Eq => "==",
            Self::Neq => "!=",
            Self::Lt => "<",
            Self::Lte => "<=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::In => "in",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    True,
    False,
    Integer,
    String,
}

impl LiteralKind {
    #[must_use]
    pub const fn from_kind(kind: PolicySyntax) -> Option<Self> {
        match kind {
            PolicySyntax::TrueKeyword => Some(Self::True),
            PolicySyntax::FalseKeyword => Some(Self::False),
            PolicySyntax::Integer => Some(Self::Integer),
            PolicySyntax::String => Some(Self::String),
            _ => None,
        }
    }
}

/// Template slot identifier.
///
/// Cedar templates support exactly two slots: `?principal` and `?resource`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotKind {
    Principal,
    Resource,
}

impl SlotKind {
    #[must_use]
    pub fn from_text(value: &str) -> Option<Self> {
        match value {
            "principal" => Some(Self::Principal),
            "resource" => Some(Self::Resource),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Principal => "principal",
            Self::Resource => "resource",
        }
    }
}
