use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::clause::{Clause, ClauseKind};
use super::constraint::{ActionConstraint, PrincipalConstraint, ResourceConstraint};
use super::effect::Effect;
use super::expr::Expr;
use super::ops::UnaryOp;
use super::slot::SlotId;
use crate::common::{Annotations, EntityUid};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PolicyId(String);

impl PolicyId {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self(id)
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

impl AsRef<str> for PolicyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Template {
    id: PolicyId,
    annotations: Annotations,
    effect: Effect,
    principal: PrincipalConstraint,
    action: ActionConstraint,
    resource: ResourceConstraint,
    clauses: Vec<Clause>,
}

impl Template {
    #[must_use]
    pub const fn new(
        id: PolicyId,
        annotations: Annotations,
        effect: Effect,
        principal: PrincipalConstraint,
        action: ActionConstraint,
        resource: ResourceConstraint,
        clauses: Vec<Clause>,
    ) -> Self {
        Self {
            id,
            annotations,
            effect,
            principal,
            action,
            resource,
            clauses,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &PolicyId {
        &self.id
    }

    #[must_use]
    pub const fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    #[must_use]
    pub const fn effect(&self) -> Effect {
        self.effect
    }

    #[must_use]
    pub const fn principal(&self) -> &PrincipalConstraint {
        &self.principal
    }

    #[must_use]
    pub const fn action(&self) -> &ActionConstraint {
        &self.action
    }

    #[must_use]
    pub const fn resource(&self) -> &ResourceConstraint {
        &self.resource
    }

    #[must_use]
    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }

    #[must_use]
    pub fn condition(&self) -> Option<Expr> {
        if self.clauses.is_empty() {
            return None;
        }

        let mut iter = self.clauses.iter();
        let first = iter.next()?;
        let first_expr = clause_to_expr(first);

        Some(iter.fold(first_expr, |acc, clause| {
            Expr::and(acc, clause_to_expr(clause))
        }))
    }

    #[must_use]
    pub fn has_slots(&self) -> bool {
        self.principal.constraint().has_slot()
            || self.resource.constraint().has_slot()
            || self.clauses.iter().any(Clause::has_slot)
    }

    #[must_use]
    pub fn is_static(&self) -> bool {
        !self.has_slots()
    }
}

fn clause_to_expr(clause: &Clause) -> Expr {
    match clause.kind() {
        ClauseKind::When => clause.body().clone(),
        ClauseKind::Unless => Expr::unary(UnaryOp::Not, clause.body().clone()),
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct SlotValues(BTreeMap<SlotId, Arc<EntityUid>>);

impl SlotValues {
    #[must_use]
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }

    #[must_use]
    pub const fn from_map(map: BTreeMap<SlotId, Arc<EntityUid>>) -> Self {
        Self(map)
    }

    pub fn set_principal(&mut self, uid: EntityUid) {
        self.0.insert(SlotId::Principal, Arc::new(uid));
    }

    pub fn set_resource(&mut self, uid: EntityUid) {
        self.0.insert(SlotId::Resource, Arc::new(uid));
    }

    #[must_use]
    pub fn principal(&self) -> Option<&EntityUid> {
        self.0
            .get(&SlotId::Principal)
            .map(core::convert::AsRef::as_ref)
    }

    #[must_use]
    pub fn resource(&self) -> Option<&EntityUid> {
        self.0
            .get(&SlotId::Resource)
            .map(core::convert::AsRef::as_ref)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (SlotId, &EntityUid)> {
        self.0
            .iter()
            .map(|(slot_id, entity_uid)| (*slot_id, entity_uid.as_ref()))
    }

    #[must_use]
    pub fn into_map(self) -> BTreeMap<SlotId, Arc<EntityUid>> {
        self.0
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Policy {
    template: Arc<Template>,
    link_id: Option<PolicyId>,
    slot_values: SlotValues,
}

impl Policy {
    #[must_use]
    pub const fn new(
        template: Arc<Template>,
        link_id: Option<PolicyId>,
        slot_values: SlotValues,
    ) -> Self {
        Self {
            template,
            link_id,
            slot_values,
        }
    }

    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "TODO")]
    pub fn from_static(template: Template) -> Self {
        assert!(template.is_static(), "template has slots");
        Self {
            template: Arc::new(template),
            link_id: None,
            slot_values: SlotValues::new(),
        }
    }

    #[must_use]
    pub fn template(&self) -> &Template {
        &self.template
    }

    #[must_use]
    pub const fn link_id(&self) -> Option<&PolicyId> {
        self.link_id.as_ref()
    }

    #[must_use]
    pub const fn slot_values(&self) -> &SlotValues {
        &self.slot_values
    }

    #[must_use]
    pub fn id(&self) -> &PolicyId {
        self.link_id.as_ref().unwrap_or_else(|| self.template.id())
    }

    #[must_use]
    pub fn template_id(&self) -> &PolicyId {
        self.template.id()
    }

    #[must_use]
    pub fn annotations(&self) -> &Annotations {
        self.template.annotations()
    }

    #[must_use]
    pub fn effect(&self) -> Effect {
        self.template.effect()
    }

    #[must_use]
    pub const fn is_static(&self) -> bool {
        self.link_id.is_none()
    }
}
