use core::fmt::{self, Write};

use super::{
    AstNode, AstToken as _, ConditionKind, Effect, Expression, IdentifierToken, PolicyNode,
    StringToken, Variable,
};
use crate::policy::PolicySyntax;

#[derive(Debug, Clone, Copy)]
pub struct Policies<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for Policies<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::PolicySet
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> Policies<'a> {
    pub fn policies(&self) -> impl Iterator<Item = Policy<'a>> + 'a {
        self.node.children().filter_map(Policy::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Policy<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for Policy<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Policy
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> Policy<'a> {
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + 'a {
        self.node.children().filter_map(Annotation::cast)
    }

    #[must_use]
    pub fn effect(&self) -> Option<Effect> {
        let token = self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::PermitKeyword | PolicySyntax::ForbidKeyword
            )
        })?;
        Effect::from_kind(token.value())
    }

    #[must_use]
    pub fn effect_token(&self) -> Option<PolicyNode<'a>> {
        self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::PermitKeyword | PolicySyntax::ForbidKeyword
            )
        })
    }

    pub fn variables(&self) -> impl Iterator<Item = VariableDefinition<'a>> + 'a {
        self.node.children().filter_map(VariableDefinition::cast)
    }

    pub fn conditions(&self) -> impl Iterator<Item = Condition<'a>> + 'a {
        self.node.children().filter_map(Condition::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Annotation<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for Annotation<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Annotation
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> Annotation<'a> {
    #[must_use]
    pub fn name(&self) -> Option<IdentifierToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::Identifier)
            .and_then(IdentifierToken::cast)
    }

    #[must_use]
    pub fn value(&self) -> Option<StringToken<'a>> {
        self.node
            .children()
            .find(|node| node.value() == PolicySyntax::String)
            .and_then(StringToken::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VariableDefinition<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for VariableDefinition<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::VariableDefinition
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> VariableDefinition<'a> {
    #[must_use]
    pub fn variable(&self) -> Option<Variable> {
        let variable = Variable::from_kind(self.variable_token()?.value())?;
        match variable {
            Variable::Context => None,
            _ => Some(variable),
        }
    }

    #[must_use]
    pub fn variable_token(&self) -> Option<PolicyNode<'a>> {
        self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::PrincipalKeyword
                    | PolicySyntax::ActionKeyword
                    | PolicySyntax::ResourceKeyword
                    | PolicySyntax::ContextKeyword
            )
        })
    }

    #[must_use]
    pub fn constraint(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Condition<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for Condition<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Condition
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> Condition<'a> {
    #[must_use]
    pub fn kind(&self) -> Option<ConditionKind> {
        let token = self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::WhenKeyword | PolicySyntax::UnlessKeyword
            )
        })?;
        ConditionKind::from_kind(token.value())
    }

    #[must_use]
    pub fn kind_token(&self) -> Option<PolicyNode<'a>> {
        self.node.children().find(|node| {
            matches!(
                node.value(),
                PolicySyntax::WhenKeyword | PolicySyntax::UnlessKeyword
            )
        })
    }

    #[must_use]
    pub fn expr(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

/// Qualified name consisting of one or more `::` separated segments.
///
/// ```cedar
/// permit(principal == Namespace::User::"alice", action, resource);
/// //                  ^^^^^^^^^^^^^^^ Name with segments [Namespace, User]
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Name<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstNode<'a> for Name<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Name
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

impl<'a> Name<'a> {
    /// Returns an iterator over the identifier segments of this name.
    ///
    /// ```cedar
    /// permit(principal == Namespace::User::"alice", action, resource);
    /// //                  ^^^^^^^^^ ^^^^ segments: ["Namespace", "User"]
    /// ```
    pub fn segments(&self) -> impl Iterator<Item = IdentifierToken<'a>> + 'a {
        self.node
            .children()
            .filter(|node| node.value() == PolicySyntax::Identifier)
            .filter_map(IdentifierToken::cast)
    }

    /// Returns `true` if this is a simple (unqualified) name with a single segment.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.active };
    /// //     ^^^^^^^^^ is_simple returns true
    /// //     Namespace::Type is_simple returns false
    /// ```
    #[must_use]
    pub fn is_simple(&self) -> bool {
        let mut segments = self.segments();
        segments.next().is_some() && segments.next().is_none()
    }

    /// Returns the first segment if this is a simple (unqualified) name.
    ///
    /// Returns `None` if the name has multiple segments.
    ///
    /// ```cedar
    /// permit(principal, action, resource)
    /// when { principal.active };
    /// //     ^^^^^^^^^ as_simple returns Some(IdentifierToken)
    /// ```
    #[must_use]
    pub fn as_simple(&self) -> Option<IdentifierToken<'a>> {
        let mut segments = self.segments();
        let first = segments.next()?;
        if segments.next().is_some() {
            return None;
        }
        Some(first)
    }

    /// Writes the full qualified name to the given writer.
    ///
    /// Segments are joined with `::`.
    pub fn write_to<W: Write>(&self, source: &str, writer: &mut W) -> fmt::Result {
        let mut first = true;
        for segment in self.segments() {
            if !first {
                writer.write_str("::")?;
            }

            first = false;
            writer.write_str(segment.text(source))?;
        }

        Ok(())
    }
}
