use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::integer::Integer;
use super::literal::Literal;
use super::ops::{BinaryOp, UnaryOp};
use super::pattern::Pattern;
use super::slot::SlotId;
use super::unknown::Unknown;
use super::var::Var;
use crate::common::{EntityType, Name};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Expr<T = ()> {
    kind: ExprKind<T>,
    data: T,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ExprKind<T = ()> {
    Literal(Literal),
    Var(Var),
    Slot(SlotId),
    Unknown(Unknown),
    Unary {
        op: UnaryOp,
        arg: Arc<Expr<T>>,
    },
    Binary {
        op: BinaryOp,
        left: Arc<Expr<T>>,
        right: Arc<Expr<T>>,
    },
    And {
        left: Arc<Expr<T>>,
        right: Arc<Expr<T>>,
    },
    Or {
        left: Arc<Expr<T>>,
        right: Arc<Expr<T>>,
    },
    If {
        cond: Arc<Expr<T>>,
        then_expr: Arc<Expr<T>>,
        else_expr: Arc<Expr<T>>,
    },
    GetAttr {
        expr: Arc<Expr<T>>,
        attr: String,
    },
    HasAttr {
        expr: Arc<Expr<T>>,
        attr: String,
    },
    Is {
        expr: Arc<Expr<T>>,
        entity_type: EntityType,
    },
    Like {
        expr: Arc<Expr<T>>,
        pattern: Pattern,
    },
    Set(Arc<Vec<Expr<T>>>),
    Record(Arc<BTreeMap<String, Expr<T>>>),
    ExtensionCall {
        fn_name: Name,
        args: Arc<Vec<Expr<T>>>,
    },
}

impl<T> Expr<T> {
    #[must_use]
    pub const fn new(kind: ExprKind<T>, data: T) -> Self {
        Self { kind, data }
    }

    #[must_use]
    pub const fn kind(&self) -> &ExprKind<T> {
        &self.kind
    }

    #[must_use]
    pub const fn data(&self) -> &T {
        &self.data
    }

    #[must_use]
    pub fn into_parts(self) -> (ExprKind<T>, T) {
        (self.kind, self.data)
    }
}

impl<T: Default> Expr<T> {
    #[must_use]
    pub fn literal(lit: Literal) -> Self {
        Self::new(ExprKind::Literal(lit), T::default())
    }

    #[must_use]
    pub fn bool(value: bool) -> Self {
        Self::literal(Literal::Bool(value))
    }

    #[must_use]
    pub fn long(value: Integer) -> Self {
        Self::literal(Literal::Long(value))
    }

    #[must_use]
    pub fn string(value: String) -> Self {
        Self::literal(Literal::String(value))
    }

    #[must_use]
    pub fn var(var: Var) -> Self {
        Self::new(ExprKind::Var(var), T::default())
    }

    #[must_use]
    pub fn slot(slot: SlotId) -> Self {
        Self::new(ExprKind::Slot(slot), T::default())
    }

    #[must_use]
    pub fn unknown(unknown: Unknown) -> Self {
        Self::new(ExprKind::Unknown(unknown), T::default())
    }

    #[must_use]
    pub fn unary(op: UnaryOp, arg: Self) -> Self {
        Self::new(
            ExprKind::Unary {
                op,
                arg: Arc::new(arg),
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn binary(op: BinaryOp, left: Self, right: Self) -> Self {
        Self::new(
            ExprKind::Binary {
                op,
                left: Arc::new(left),
                right: Arc::new(right),
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn and(left: Self, right: Self) -> Self {
        Self::new(
            ExprKind::And {
                left: Arc::new(left),
                right: Arc::new(right),
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn or(left: Self, right: Self) -> Self {
        Self::new(
            ExprKind::Or {
                left: Arc::new(left),
                right: Arc::new(right),
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn if_then_else(cond: Self, then_expr: Self, else_expr: Self) -> Self {
        Self::new(
            ExprKind::If {
                cond: Arc::new(cond),
                then_expr: Arc::new(then_expr),
                else_expr: Arc::new(else_expr),
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn get_attr(expr: Self, attr: String) -> Self {
        Self::new(
            ExprKind::GetAttr {
                expr: Arc::new(expr),
                attr,
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn has_attr(expr: Self, attr: String) -> Self {
        Self::new(
            ExprKind::HasAttr {
                expr: Arc::new(expr),
                attr,
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn is(expr: Self, entity_type: EntityType) -> Self {
        Self::new(
            ExprKind::Is {
                expr: Arc::new(expr),
                entity_type,
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn like(expr: Self, pattern: Pattern) -> Self {
        Self::new(
            ExprKind::Like {
                expr: Arc::new(expr),
                pattern,
            },
            T::default(),
        )
    }

    #[must_use]
    pub fn empty_set() -> Self {
        Self::new(ExprKind::Set(Arc::new(Vec::new())), T::default())
    }

    #[must_use]
    pub fn set(elements: Vec<Self>) -> Self {
        Self::new(ExprKind::Set(Arc::new(elements)), T::default())
    }

    #[must_use]
    pub fn empty_record() -> Self {
        Self::new(ExprKind::Record(Arc::new(BTreeMap::new())), T::default())
    }

    #[must_use]
    pub fn record(fields: BTreeMap<String, Self>) -> Self {
        Self::new(ExprKind::Record(Arc::new(fields)), T::default())
    }

    #[must_use]
    pub fn extension_call(fn_name: Name, args: Vec<Self>) -> Self {
        Self::new(
            ExprKind::ExtensionCall {
                fn_name,
                args: Arc::new(args),
            },
            T::default(),
        )
    }
}

impl<T> Expr<T> {
    #[must_use]
    pub const fn is_literal(&self) -> bool {
        matches!(self.kind, ExprKind::Literal(_))
    }

    #[must_use]
    pub const fn is_var(&self) -> bool {
        matches!(self.kind, ExprKind::Var(_))
    }

    #[must_use]
    pub fn has_slot(&self) -> bool {
        match &self.kind {
            ExprKind::Literal(_) | ExprKind::Var(_) | ExprKind::Unknown(_) => false,
            ExprKind::Slot(_) => true,
            ExprKind::Unary { arg, .. } => arg.has_slot(),
            ExprKind::Binary { left, right, .. }
            | ExprKind::And { left, right }
            | ExprKind::Or { left, right } => left.has_slot() || right.has_slot(),
            ExprKind::If {
                cond,
                then_expr,
                else_expr,
            } => cond.has_slot() || then_expr.has_slot() || else_expr.has_slot(),
            ExprKind::GetAttr { expr, .. }
            | ExprKind::HasAttr { expr, .. }
            | ExprKind::Is { expr, .. }
            | ExprKind::Like { expr, .. } => expr.has_slot(),
            ExprKind::Set(elements) => elements.iter().any(Self::has_slot),
            ExprKind::Record(fields) => fields.values().any(Self::has_slot),
            ExprKind::ExtensionCall { args, .. } => args.iter().any(Self::has_slot),
        }
    }

    #[must_use]
    pub fn has_unknown(&self) -> bool {
        match &self.kind {
            ExprKind::Literal(_) | ExprKind::Var(_) | ExprKind::Slot(_) => false,
            ExprKind::Unknown(_) => true,
            ExprKind::Unary { arg, .. } => arg.has_unknown(),
            ExprKind::Binary { left, right, .. }
            | ExprKind::And { left, right }
            | ExprKind::Or { left, right } => left.has_unknown() || right.has_unknown(),
            ExprKind::If {
                cond,
                then_expr,
                else_expr,
            } => cond.has_unknown() || then_expr.has_unknown() || else_expr.has_unknown(),
            ExprKind::GetAttr { expr, .. }
            | ExprKind::HasAttr { expr, .. }
            | ExprKind::Is { expr, .. }
            | ExprKind::Like { expr, .. } => expr.has_unknown(),
            ExprKind::Set(elements) => elements.iter().any(Self::has_unknown),
            ExprKind::Record(fields) => fields.values().any(Self::has_unknown),
            ExprKind::ExtensionCall { args, .. } => args.iter().any(Self::has_unknown),
        }
    }
}
