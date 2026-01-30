use super::expr::Expr;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum ClauseKind {
    When,
    Unless,
}

impl ClauseKind {
    #[must_use]
    pub const fn is_when(self) -> bool {
        matches!(self, Self::When)
    }

    #[must_use]
    pub const fn is_unless(self) -> bool {
        matches!(self, Self::Unless)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Clause {
    kind: ClauseKind,
    body: Expr,
}

impl Clause {
    #[must_use]
    pub const fn new(kind: ClauseKind, body: Expr) -> Self {
        Self { kind, body }
    }

    #[must_use]
    pub const fn when(body: Expr) -> Self {
        Self::new(ClauseKind::When, body)
    }

    #[must_use]
    pub const fn unless(body: Expr) -> Self {
        Self::new(ClauseKind::Unless, body)
    }

    #[must_use]
    pub const fn kind(&self) -> ClauseKind {
        self.kind
    }

    #[must_use]
    pub const fn body(&self) -> &Expr {
        &self.body
    }

    #[must_use]
    pub fn into_body(self) -> Expr {
        self.body
    }

    #[must_use]
    pub fn has_slot(&self) -> bool {
        self.body.has_slot()
    }
}
