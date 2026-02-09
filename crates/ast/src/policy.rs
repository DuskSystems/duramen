use alloc::vec::Vec;

use crate::common::Annotations;

mod action_constraint;
pub use action_constraint::ActionConstraint;

mod action_list;
pub use action_list::ActionList;

mod binary_operator;
pub use binary_operator::BinaryOperator;

mod condition;
pub use condition::{Condition, ConditionKind};

mod effect;
pub use effect::Effect;

mod entity_or_slot;
pub use entity_or_slot::EntityOrSlot;

mod entity_reference;
pub use entity_reference::EntityReference;

mod expression;
pub use expression::{Expression, ExpressionKind};

mod literal;
pub use literal::Literal;

mod pattern;
pub use pattern::Pattern;

mod pattern_element;
pub use pattern_element::PatternElement;

mod policies;
pub use policies::Policies;

mod principal_constraint;
pub use principal_constraint::PrincipalConstraint;

mod record_expression;
pub use record_expression::RecordExpression;

mod resource_constraint;
pub use resource_constraint::ResourceConstraint;

mod scope_constraint;
pub use scope_constraint::ScopeConstraint;

mod slot_kind;
pub use slot_kind::SlotKind;

mod unary_operator;
pub use unary_operator::UnaryOperator;

mod variable;
pub use variable::Variable;

/// A Cedar policy.
#[derive(Clone, Debug)]
pub struct Policy<'a> {
    annotations: Annotations<'a>,
    effect: Effect,
    principal: PrincipalConstraint<'a>,
    action: ActionConstraint<'a>,
    resource: ResourceConstraint<'a>,
    conditions: Vec<Condition<'a>>,
}

impl<'a> Policy<'a> {
    /// Creates a new policy.
    #[must_use]
    pub const fn new(
        annotations: Annotations<'a>,
        effect: Effect,
        principal: PrincipalConstraint<'a>,
        action: ActionConstraint<'a>,
        resource: ResourceConstraint<'a>,
        conditions: Vec<Condition<'a>>,
    ) -> Self {
        Self {
            annotations,
            effect,
            principal,
            action,
            resource,
            conditions,
        }
    }

    /// Returns the policy annotations.
    #[must_use]
    pub const fn annotations(&self) -> &Annotations<'a> {
        &self.annotations
    }

    /// Returns the policy effect (permit or forbid).
    #[must_use]
    pub const fn effect(&self) -> Effect {
        self.effect
    }

    /// Returns the principal constraint.
    #[must_use]
    pub const fn principal(&self) -> &PrincipalConstraint<'a> {
        &self.principal
    }

    /// Returns the action constraint.
    #[must_use]
    pub const fn action(&self) -> &ActionConstraint<'a> {
        &self.action
    }

    /// Returns the resource constraint.
    #[must_use]
    pub const fn resource(&self) -> &ResourceConstraint<'a> {
        &self.resource
    }

    /// Returns the policy conditions.
    #[must_use]
    pub fn conditions(&self) -> &[Condition<'a>] {
        &self.conditions
    }
}
