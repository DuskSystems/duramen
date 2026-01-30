//! EST policy types — strict enums, derive-only serde.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::expr::Expr;
use super::value::EntityRef;

// ============================================================
// Shared Types
// ============================================================

/// Template slot identifier.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SlotId {
    #[cfg_attr(feature = "serde", serde(rename = "?principal"))]
    Principal,
    #[cfg_attr(feature = "serde", serde(rename = "?resource"))]
    Resource,
}

/// Either a concrete entity or a template slot.
///
/// Serializes as `{"entity": ...}` or `{"slot": ...}`.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EntityTarget {
    Entity { entity: EntityRef },
    Slot { slot: SlotId },
}

impl EntityTarget {
    #[must_use]
    pub const fn entity(e: EntityRef) -> Self {
        Self::Entity { entity: e }
    }

    #[must_use]
    pub const fn slot(s: SlotId) -> Self {
        Self::Slot { slot: s }
    }

    #[must_use]
    pub const fn is_slot(&self) -> bool {
        matches!(self, Self::Slot { .. })
    }
}

/// Target for `is...in` constraint.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InTarget {
    Entity { entity: EntityRef },
    Slot { slot: SlotId },
}

impl InTarget {
    #[must_use]
    pub const fn is_slot(&self) -> bool {
        matches!(self, Self::Slot { .. })
    }
}

// ============================================================
// Policy Effect
// ============================================================

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Effect {
    Permit,
    Forbid,
}

impl Effect {
    #[must_use]
    pub const fn is_permit(self) -> bool {
        matches!(self, Self::Permit)
    }

    #[must_use]
    pub const fn is_forbid(self) -> bool {
        matches!(self, Self::Forbid)
    }
}

// ============================================================
// Conditions
// ============================================================

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum ConditionKind {
    When,
    Unless,
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Condition {
    pub kind: ConditionKind,
    pub body: Expr,
}

impl Condition {
    #[must_use]
    pub const fn new(kind: ConditionKind, body: Expr) -> Self {
        Self { kind, body }
    }

    #[must_use]
    pub const fn when(body: Expr) -> Self {
        Self::new(ConditionKind::When, body)
    }

    #[must_use]
    pub const fn unless(body: Expr) -> Self {
        Self::new(ConditionKind::Unless, body)
    }
}

// ============================================================
// Principal Constraint
// ============================================================

/// Principal scope constraint.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "op"))]
pub enum PrincipalConstraint {
    /// Matches any principal.
    All,

    /// Principal must equal this entity/slot.
    #[cfg_attr(feature = "serde", serde(rename = "=="))]
    Eq {
        #[cfg_attr(feature = "serde", serde(flatten))]
        target: EntityTarget,
    },

    /// Principal must be in this entity/slot's hierarchy.
    #[cfg_attr(feature = "serde", serde(rename = "in"))]
    In {
        #[cfg_attr(feature = "serde", serde(flatten))]
        target: EntityTarget,
    },

    /// Principal must be of this entity type.
    #[cfg_attr(feature = "serde", serde(rename = "is"))]
    Is {
        entity_type: String,
        #[cfg_attr(
            feature = "serde",
            serde(rename = "in", default, skip_serializing_if = "Option::is_none")
        )]
        in_target: Option<InTarget>,
    },
}

impl PrincipalConstraint {
    #[must_use]
    pub const fn any() -> Self {
        Self::All
    }

    #[must_use]
    pub const fn eq_entity(entity: EntityRef) -> Self {
        Self::Eq {
            target: EntityTarget::entity(entity),
        }
    }

    #[must_use]
    pub const fn eq_slot() -> Self {
        Self::Eq {
            target: EntityTarget::slot(SlotId::Principal),
        }
    }

    #[must_use]
    pub const fn in_entity(entity: EntityRef) -> Self {
        Self::In {
            target: EntityTarget::entity(entity),
        }
    }

    #[must_use]
    pub const fn in_slot() -> Self {
        Self::In {
            target: EntityTarget::slot(SlotId::Principal),
        }
    }

    #[must_use]
    pub fn is_type<S: Into<String>>(entity_type: S) -> Self {
        Self::Is {
            entity_type: entity_type.into(),
            in_target: None,
        }
    }

    #[must_use]
    pub fn is_type_in_entity<S: Into<String>>(entity_type: S, entity: EntityRef) -> Self {
        Self::Is {
            entity_type: entity_type.into(),
            in_target: Some(InTarget::Entity { entity }),
        }
    }

    #[must_use]
    pub fn is_type_in_slot<S: Into<String>>(entity_type: S) -> Self {
        Self::Is {
            entity_type: entity_type.into(),
            in_target: Some(InTarget::Slot {
                slot: SlotId::Principal,
            }),
        }
    }

    #[must_use]
    pub const fn has_slot(&self) -> bool {
        match self {
            Self::All => false,
            Self::Eq { target } | Self::In { target } => target.is_slot(),
            Self::Is { in_target, .. } => matches!(in_target, Some(t) if t.is_slot()),
        }
    }
}

// ============================================================
// Resource Constraint
// ============================================================

/// Resource scope constraint.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "op"))]
pub enum ResourceConstraint {
    All,

    #[cfg_attr(feature = "serde", serde(rename = "=="))]
    Eq {
        #[cfg_attr(feature = "serde", serde(flatten))]
        target: EntityTarget,
    },

    #[cfg_attr(feature = "serde", serde(rename = "in"))]
    In {
        #[cfg_attr(feature = "serde", serde(flatten))]
        target: EntityTarget,
    },

    #[cfg_attr(feature = "serde", serde(rename = "is"))]
    Is {
        entity_type: String,
        #[cfg_attr(
            feature = "serde",
            serde(rename = "in", default, skip_serializing_if = "Option::is_none")
        )]
        in_target: Option<InTarget>,
    },
}

impl ResourceConstraint {
    #[must_use]
    pub const fn any() -> Self {
        Self::All
    }

    #[must_use]
    pub const fn eq_entity(entity: EntityRef) -> Self {
        Self::Eq {
            target: EntityTarget::entity(entity),
        }
    }

    #[must_use]
    pub const fn eq_slot() -> Self {
        Self::Eq {
            target: EntityTarget::slot(SlotId::Resource),
        }
    }

    #[must_use]
    pub const fn in_entity(entity: EntityRef) -> Self {
        Self::In {
            target: EntityTarget::entity(entity),
        }
    }

    #[must_use]
    pub const fn in_slot() -> Self {
        Self::In {
            target: EntityTarget::slot(SlotId::Resource),
        }
    }

    #[must_use]
    pub fn is_type<S: Into<String>>(entity_type: S) -> Self {
        Self::Is {
            entity_type: entity_type.into(),
            in_target: None,
        }
    }

    #[must_use]
    pub fn is_type_in_entity<S: Into<String>>(entity_type: S, entity: EntityRef) -> Self {
        Self::Is {
            entity_type: entity_type.into(),
            in_target: Some(InTarget::Entity { entity }),
        }
    }

    #[must_use]
    pub fn is_type_in_slot<S: Into<String>>(entity_type: S) -> Self {
        Self::Is {
            entity_type: entity_type.into(),
            in_target: Some(InTarget::Slot {
                slot: SlotId::Resource,
            }),
        }
    }

    #[must_use]
    pub const fn has_slot(&self) -> bool {
        match self {
            Self::All => false,
            Self::Eq { target } | Self::In { target } => target.is_slot(),
            Self::Is { in_target, .. } => matches!(in_target, Some(t) if t.is_slot()),
        }
    }
}

// ============================================================
// Action Constraint
// ============================================================

/// Target for action `in` constraint.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ActionInTarget {
    Single { entity: EntityRef },
    Multiple { entities: Vec<EntityRef> },
}

/// Action scope constraint.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "op"))]
pub enum ActionConstraint {
    /// Matches any action.
    All,

    /// Action must equal this specific action.
    #[cfg_attr(feature = "serde", serde(rename = "=="))]
    Eq { entity: EntityRef },

    /// Action must be in one of these action groups.
    #[cfg_attr(feature = "serde", serde(rename = "in"))]
    In {
        #[cfg_attr(feature = "serde", serde(flatten))]
        target: ActionInTarget,
    },
}

impl ActionConstraint {
    #[must_use]
    pub const fn any() -> Self {
        Self::All
    }

    #[must_use]
    pub const fn eq_entity(entity: EntityRef) -> Self {
        Self::Eq { entity }
    }

    #[must_use]
    pub const fn in_entity(entity: EntityRef) -> Self {
        Self::In {
            target: ActionInTarget::Single { entity },
        }
    }

    #[must_use]
    pub const fn in_entities(entities: Vec<EntityRef>) -> Self {
        Self::In {
            target: ActionInTarget::Multiple { entities },
        }
    }
}

// ============================================================
// Policy & PolicySet
// ============================================================

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Policy {
    pub effect: Effect,
    pub principal: PrincipalConstraint,
    pub action: ActionConstraint,
    pub resource: ResourceConstraint,
    pub conditions: Vec<Condition>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub annotations: BTreeMap<String, String>,
}

impl Policy {
    #[must_use]
    pub const fn new(
        effect: Effect,
        principal: PrincipalConstraint,
        action: ActionConstraint,
        resource: ResourceConstraint,
        conditions: Vec<Condition>,
        annotations: BTreeMap<String, String>,
    ) -> Self {
        Self {
            effect,
            principal,
            action,
            resource,
            conditions,
            annotations,
        }
    }

    #[must_use]
    pub const fn is_template(&self) -> bool {
        self.principal.has_slot() || self.resource.has_slot()
    }

    #[must_use]
    pub const fn is_static(&self) -> bool {
        !self.is_template()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TemplateLink {
    pub template_id: String,
    pub new_id: String,
    pub values: BTreeMap<String, EntityRef>,
}

impl TemplateLink {
    #[must_use]
    pub const fn new(
        template_id: String,
        new_id: String,
        values: BTreeMap<String, EntityRef>,
    ) -> Self {
        Self {
            template_id,
            new_id,
            values,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PolicySet {
    #[cfg_attr(feature = "serde", serde(default))]
    pub templates: BTreeMap<String, Policy>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub static_policies: BTreeMap<String, Policy>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub template_links: Vec<TemplateLink>,
}

impl PolicySet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_static_policy(&mut self, id: String, policy: Policy) {
        self.static_policies.insert(id, policy);
    }

    pub fn add_template(&mut self, id: String, template: Policy) {
        self.templates.insert(id, template);
    }

    pub fn add_template_link(&mut self, link: TemplateLink) {
        self.template_links.push(link);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.static_policies
            .len()
            .saturating_add(self.templates.len())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.static_policies.is_empty() && self.templates.is_empty()
    }
}
