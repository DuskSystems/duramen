//! EST expression types — no invalid states, derive-only serde.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::value::LiteralValue;

// ============================================================
// Variables & Slots
// ============================================================

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Var {
    #[cfg_attr(feature = "serde", serde(rename = "principal"))]
    Principal,
    #[cfg_attr(feature = "serde", serde(rename = "action"))]
    Action,
    #[cfg_attr(feature = "serde", serde(rename = "resource"))]
    Resource,
    #[cfg_attr(feature = "serde", serde(rename = "context"))]
    Context,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SlotId {
    #[cfg_attr(feature = "serde", serde(rename = "?principal"))]
    Principal,
    #[cfg_attr(feature = "serde", serde(rename = "?resource"))]
    Resource,
}

// ============================================================
// Pattern Elements
// ============================================================

/// Non-empty string for pattern literals.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PatternLiteral(String);

impl PatternLiteral {
    /// Creates a new pattern literal. Returns `None` if empty.
    #[must_use]
    pub fn new(s: String) -> Option<Self> {
        if s.is_empty() { None } else { Some(Self(s)) }
    }

    /// Creates a new pattern literal without checking.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the string is empty.
    #[must_use]
    pub fn new_unchecked(s: String) -> Self {
        debug_assert!(!s.is_empty(), "pattern literal cannot be empty");
        Self(s)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Pattern element for `like` expressions.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "wire::PatternElem", into = "wire::PatternElem")
)]
pub enum PatternElem {
    /// Matches any sequence of characters.
    Wildcard,
    /// Matches this exact non-empty string.
    Literal(PatternLiteral),
}

impl PatternElem {
    #[must_use]
    pub const fn wildcard() -> Self {
        Self::Wildcard
    }

    #[must_use]
    pub fn literal(s: String) -> Option<Self> {
        PatternLiteral::new(s).map(Self::Literal)
    }
}

// ============================================================
// Extension Calls — struct with guaranteed fn_name
// ============================================================

/// Extension function call with guaranteed `fn_name`.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "wire::ExtensionCall", into = "wire::ExtensionCall")
)]
pub struct ExtensionCall {
    pub fn_name: String,
    pub args: Vec<Expr>,
}

impl ExtensionCall {
    #[must_use]
    pub fn new<S: Into<String>>(fn_name: S, args: Vec<Expr>) -> Self {
        Self {
            fn_name: fn_name.into(),
            args,
        }
    }

    #[must_use]
    pub fn fn_name(&self) -> &str {
        &self.fn_name
    }

    #[must_use]
    pub fn args(&self) -> &[Expr] {
        &self.args
    }
}

// ============================================================
// Expressions
// ============================================================

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Expr {
    Builtin(ExprBuiltin),
    ExtensionCall(ExtensionCall),
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExprBuiltin {
    /// Literal value (bool, long, string, entity only — no Set/Record).
    Value(LiteralValue),

    /// Variable reference.
    Var(Var),

    /// Template slot.
    Slot(SlotId),

    /// Set construction (expression, not literal).
    Set(Vec<Expr>),

    /// Record construction.
    Record(BTreeMap<String, Expr>),

    // --- Unary ---
    #[cfg_attr(feature = "serde", serde(rename = "!"))]
    Not { arg: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "neg"))]
    Neg { arg: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "isEmpty"))]
    IsEmpty { arg: Box<Expr> },

    // --- Binary comparison ---
    #[cfg_attr(feature = "serde", serde(rename = "=="))]
    Eq { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "!="))]
    NotEq { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "<"))]
    Less { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "<="))]
    LessEq { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = ">"))]
    Greater { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = ">="))]
    GreaterEq { left: Box<Expr>, right: Box<Expr> },

    // --- Logical ---
    #[cfg_attr(feature = "serde", serde(rename = "&&"))]
    And { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "||"))]
    Or { left: Box<Expr>, right: Box<Expr> },

    // --- Arithmetic ---
    #[cfg_attr(feature = "serde", serde(rename = "+"))]
    Add { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Sub { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    Mul { left: Box<Expr>, right: Box<Expr> },

    // --- Membership ---
    #[cfg_attr(feature = "serde", serde(rename = "in"))]
    In { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "contains"))]
    Contains { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "containsAll"))]
    ContainsAll { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "containsAny"))]
    ContainsAny { left: Box<Expr>, right: Box<Expr> },

    // --- Attributes & Tags ---
    #[cfg_attr(feature = "serde", serde(rename = "."))]
    GetAttr { left: Box<Expr>, attr: String },

    #[cfg_attr(feature = "serde", serde(rename = "has"))]
    HasAttr { left: Box<Expr>, attr: String },

    #[cfg_attr(feature = "serde", serde(rename = "getTag"))]
    GetTag { left: Box<Expr>, right: Box<Expr> },

    #[cfg_attr(feature = "serde", serde(rename = "hasTag"))]
    HasTag { left: Box<Expr>, right: Box<Expr> },

    // --- Pattern matching ---
    #[cfg_attr(feature = "serde", serde(rename = "like"))]
    Like {
        left: Box<Expr>,
        pattern: Vec<PatternElem>,
    },

    // --- Type test ---
    #[cfg_attr(feature = "serde", serde(rename = "is"))]
    Is {
        left: Box<Expr>,
        entity_type: String,
        #[cfg_attr(
            feature = "serde",
            serde(rename = "in", skip_serializing_if = "Option::is_none")
        )]
        in_expr: Option<Box<Expr>>,
    },

    // --- Conditional ---
    #[cfg_attr(feature = "serde", serde(rename = "if-then-else"))]
    If {
        #[cfg_attr(feature = "serde", serde(rename = "if"))]
        cond: Box<Expr>,
        #[cfg_attr(feature = "serde", serde(rename = "then"))]
        then_expr: Box<Expr>,
        #[cfg_attr(feature = "serde", serde(rename = "else"))]
        else_expr: Box<Expr>,
    },
}

// ============================================================
// Expr constructors
// ============================================================

impl Expr {
    #[must_use]
    pub const fn value(v: LiteralValue) -> Self {
        Self::Builtin(ExprBuiltin::Value(v))
    }

    #[must_use]
    pub const fn var(v: Var) -> Self {
        Self::Builtin(ExprBuiltin::Var(v))
    }

    #[must_use]
    pub const fn slot(s: SlotId) -> Self {
        Self::Builtin(ExprBuiltin::Slot(s))
    }

    #[must_use]
    pub const fn set(elements: Vec<Self>) -> Self {
        Self::Builtin(ExprBuiltin::Set(elements))
    }

    #[must_use]
    pub const fn record(fields: BTreeMap<String, Self>) -> Self {
        Self::Builtin(ExprBuiltin::Record(fields))
    }

    #[must_use]
    pub fn not(arg: Self) -> Self {
        Self::Builtin(ExprBuiltin::Not { arg: Box::new(arg) })
    }

    #[must_use]
    pub fn neg(arg: Self) -> Self {
        Self::Builtin(ExprBuiltin::Neg { arg: Box::new(arg) })
    }

    #[must_use]
    pub fn is_empty(arg: Self) -> Self {
        Self::Builtin(ExprBuiltin::IsEmpty { arg: Box::new(arg) })
    }

    #[must_use]
    pub fn equal(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Eq {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn not_eq(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::NotEq {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn less(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Less {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn less_eq(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::LessEq {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn greater(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Greater {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn greater_eq(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::GreaterEq {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn and(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::And {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn or(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Or {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn is_in(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::In {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn add(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Add {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn sub(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Sub {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn mul(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Mul {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn contains(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::Contains {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn contains_all(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::ContainsAll {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn contains_any(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::ContainsAny {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn get_tag(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::GetTag {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn has_tag(left: Self, right: Self) -> Self {
        Self::Builtin(ExprBuiltin::HasTag {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[must_use]
    pub fn get_attr<S: Into<String>>(left: Self, attr: S) -> Self {
        Self::Builtin(ExprBuiltin::GetAttr {
            left: Box::new(left),
            attr: attr.into(),
        })
    }

    #[must_use]
    pub fn has_attr<S: Into<String>>(left: Self, attr: S) -> Self {
        Self::Builtin(ExprBuiltin::HasAttr {
            left: Box::new(left),
            attr: attr.into(),
        })
    }

    #[must_use]
    pub fn like(left: Self, pattern: Vec<PatternElem>) -> Self {
        Self::Builtin(ExprBuiltin::Like {
            left: Box::new(left),
            pattern,
        })
    }

    #[must_use]
    pub fn is_type<S: Into<String>>(left: Self, entity_type: S) -> Self {
        Self::Builtin(ExprBuiltin::Is {
            left: Box::new(left),
            entity_type: entity_type.into(),
            in_expr: None,
        })
    }

    #[must_use]
    pub fn is_type_in<S: Into<String>>(left: Self, entity_type: S, in_expr: Self) -> Self {
        Self::Builtin(ExprBuiltin::Is {
            left: Box::new(left),
            entity_type: entity_type.into(),
            in_expr: Some(Box::new(in_expr)),
        })
    }

    #[must_use]
    pub fn if_then_else(cond: Self, then_expr: Self, else_expr: Self) -> Self {
        Self::Builtin(ExprBuiltin::If {
            cond: Box::new(cond),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        })
    }

    #[must_use]
    pub fn extension_call<S: Into<String>>(fn_name: S, args: Vec<Self>) -> Self {
        Self::ExtensionCall(ExtensionCall::new(fn_name, args))
    }

    #[must_use]
    pub const fn is_extension_call(&self) -> bool {
        matches!(self, Self::ExtensionCall(_))
    }
}

// ============================================================
// Wire format types
// ============================================================

#[cfg(feature = "serde")]
mod wire {
    use alloc::collections::BTreeMap;
    use alloc::string::String;
    use alloc::vec::Vec;

    use super::Expr;

    /// Wire format for extension calls: `{"fn_name": [args...]}`.
    #[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(transparent)]
    pub struct ExtensionCall(pub BTreeMap<String, Vec<Expr>>);

    /// Wire format for pattern elements.
    #[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
    pub enum PatternElem {
        Wildcard,
        Literal(String),
    }
}

// ============================================================
// Wire ↔ Strict conversions
// ============================================================

#[cfg(feature = "serde")]
mod conversions {
    use super::{BTreeMap, ExtensionCall, PatternElem, PatternLiteral, wire};

    #[derive(Clone, Debug)]
    pub struct InvalidExpr(&'static str);

    impl core::fmt::Display for InvalidExpr {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(self.0)
        }
    }

    #[cfg(feature = "std")]
    impl core::error::Error for InvalidExpr {}

    // --- ExtensionCall ---

    impl TryFrom<wire::ExtensionCall> for ExtensionCall {
        type Error = InvalidExpr;

        fn try_from(value: wire::ExtensionCall) -> Result<Self, Self::Error> {
            let mut iter = value.0.into_iter();
            match (iter.next(), iter.next()) {
                (Some((fn_name, args)), None) => Ok(Self { fn_name, args }),
                (None, _) => Err(InvalidExpr("extension call requires function name")),
                (Some(_), Some(_)) => Err(InvalidExpr("extension call has multiple keys")),
            }
        }
    }

    impl From<ExtensionCall> for wire::ExtensionCall {
        fn from(value: ExtensionCall) -> Self {
            let mut map = BTreeMap::new();
            map.insert(value.fn_name, value.args);
            Self(map)
        }
    }

    // --- PatternElem ---

    impl TryFrom<wire::PatternElem> for PatternElem {
        type Error = InvalidExpr;

        fn try_from(value: wire::PatternElem) -> Result<Self, Self::Error> {
            match value {
                wire::PatternElem::Wildcard => Ok(Self::Wildcard),
                wire::PatternElem::Literal(s) if s.is_empty() => {
                    Err(InvalidExpr("pattern literal cannot be empty"))
                }
                wire::PatternElem::Literal(s) => Ok(Self::Literal(PatternLiteral(s))),
            }
        }
    }

    impl From<PatternElem> for wire::PatternElem {
        fn from(value: PatternElem) -> Self {
            match value {
                PatternElem::Wildcard => Self::Wildcard,
                PatternElem::Literal(lit) => Self::Literal(lit.into_string()),
            }
        }
    }
}
