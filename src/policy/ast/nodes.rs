use core::fmt::{self, Write};

use super::{
    AstNode, AstToken, BinaryOperator, ConditionKind, Effect, Expression, IdentifierToken,
    PolicyNode, StringToken, Variable, ast_node,
};
use crate::policy::PolicySyntax;

ast_node!(Policies, PolicySyntax::PolicySet);

impl<'a> Policies<'a> {
    pub fn policies(&self) -> impl Iterator<Item = Policy<'a>> + use<'a> {
        self.node.children().filter_map(Policy::cast)
    }
}

ast_node!(Policy, PolicySyntax::Policy);

impl<'a> Policy<'a> {
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> + use<'a> {
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

    pub fn variables(&self) -> impl Iterator<Item = VariableDefinition<'a>> + use<'a> {
        self.node.children().filter_map(VariableDefinition::cast)
    }

    pub fn conditions(&self) -> impl Iterator<Item = Condition<'a>> + use<'a> {
        self.node.children().filter_map(Condition::cast)
    }
}

ast_node!(Annotation, PolicySyntax::Annotation);

impl<'a> Annotation<'a> {
    #[must_use]
    pub fn name(&self) -> Option<IdentifierOrKeywordToken<'a>> {
        self.node
            .children()
            .find_map(IdentifierOrKeywordToken::cast)
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
pub struct IdentifierOrKeywordToken<'a> {
    node: PolicyNode<'a>,
}

impl<'a> AstToken<'a> for IdentifierOrKeywordToken<'a> {
    fn can_cast(kind: PolicySyntax) -> bool {
        kind == PolicySyntax::Identifier || kind.is_keyword()
    }

    fn cast(node: PolicyNode<'a>) -> Option<Self> {
        Self::can_cast(node.value()).then_some(Self { node })
    }

    fn syntax(&self) -> &PolicyNode<'a> {
        &self.node
    }
}

ast_node!(VariableDefinition, PolicySyntax::VariableDefinition);

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
    pub fn relation(&self) -> Option<BinaryOperator> {
        let node = self
            .node
            .children()
            .find(|node| matches!(node.value(), PolicySyntax::Equal2 | PolicySyntax::InKeyword))?;
        BinaryOperator::from_kind(node.value())
    }

    #[must_use]
    pub fn constraint(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }

    #[must_use]
    pub fn has_is_constraint(&self) -> bool {
        self.node
            .children()
            .any(|node| node.value() == PolicySyntax::IsKeyword)
    }

    #[must_use]
    pub fn is_type_name(&self) -> Option<Name<'a>> {
        if !self.has_is_constraint() {
            return None;
        }
        self.node.children().find_map(Name::cast)
    }

    #[must_use]
    pub fn has_is_in_constraint(&self) -> bool {
        let mut found_is = false;
        for child in self.node.children() {
            if child.value() == PolicySyntax::IsKeyword {
                found_is = true;
            } else if found_is && child.value() == PolicySyntax::InKeyword {
                return true;
            }
        }
        false
    }

    #[must_use]
    pub fn is_in_entity(&self) -> Option<Expression<'a>> {
        if !self.has_is_in_constraint() {
            return None;
        }
        self.node.children().find_map(Expression::cast)
    }
}

ast_node!(Condition, PolicySyntax::Condition);

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
    pub fn clause_name(&self) -> Option<IdentifierOrKeywordToken<'a>> {
        self.node
            .children()
            .find_map(IdentifierOrKeywordToken::cast)
    }

    #[must_use]
    pub fn expr(&self) -> Option<Expression<'a>> {
        self.node.children().find_map(Expression::cast)
    }
}

ast_node!(Name, PolicySyntax::Name);

impl<'a> Name<'a> {
    /// Returns an iterator over the identifier segments of this name.
    ///
    /// ```cedar
    /// permit(principal == Namespace::User::"alice", action, resource);
    /// //                  ^^^^^^^^^ ^^^^ segments: ["Namespace", "User"]
    /// ```
    pub fn segments(&self) -> impl Iterator<Item = IdentifierToken<'a>> + use<'a> {
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
