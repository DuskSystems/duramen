use duramen_syntax::{Node, Syntax};

use crate::{CstNode, Expression, Name, Slot, Variable};

#[derive(Clone, Copy, Debug)]
pub struct VariableDefinition<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for VariableDefinition<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::VariableDefinition => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> VariableDefinition<'a> {
    /// Returns the template slot, if this is a slot variable (`?principal`).
    #[must_use]
    pub fn slot(&self) -> Option<Slot<'a>> {
        self.node.children().find_map(Slot::cast)
    }

    /// Returns the scope variable (`principal`, `action`, `resource`, `context`).
    #[must_use]
    pub fn variable(&self) -> Option<Variable> {
        let token = self.variable_token()?;

        match token.kind() {
            Syntax::PrincipalKeyword => Some(Variable::Principal),
            Syntax::ActionKeyword => Some(Variable::Action),
            Syntax::ResourceKeyword => Some(Variable::Resource),
            Syntax::ContextKeyword => Some(Variable::Context),
            _ => None,
        }
    }

    /// Returns the variable keyword token.
    #[must_use]
    pub fn variable_token(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| child.kind().is_identifier())
    }

    /// Returns the entity kind name (after `:` or `is`).
    #[must_use]
    pub fn kind(&self) -> Option<Name<'a>> {
        self.node.children().find_map(Name::cast)
    }

    /// Returns the constraint expression.
    ///
    /// For `principal == Entity::"id"`, returns the entity reference.
    /// For `principal is Type in Entity::"id"`, returns the entity reference after `in`.
    #[must_use]
    pub fn expression(&self) -> Option<Expression<'a>> {
        // Prefer expression after `in` (handles `is Type in expr`)
        let after_in = self
            .node
            .after(Syntax::InKeyword)
            .find_map(Expression::cast);

        if after_in.is_some() {
            return after_in;
        }

        // Fall back to expression after a comparison operator
        for child in self.node.children() {
            if matches!(
                child.kind(),
                Syntax::Equal
                    | Syntax::NotEqual
                    | Syntax::Less
                    | Syntax::LessEqual
                    | Syntax::Greater
                    | Syntax::GreaterEqual
                    | Syntax::Assign
            ) {
                return self.node.after(child.kind()).find_map(Expression::cast);
            }
        }

        None
    }

    /// Returns the `is` keyword token.
    #[must_use]
    pub fn is_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::IsKeyword)
    }

    /// Returns the `in` keyword token.
    #[must_use]
    pub fn in_token(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::InKeyword)
    }

    /// Returns the colon token.
    #[must_use]
    pub fn colon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Colon)
    }

    /// Returns the comparison operator token (`==`, `!=`, `<`, etc.).
    #[must_use]
    pub fn operator_token(&self) -> Option<Node<'a>> {
        self.node.children().find(|child| {
            matches!(
                child.kind(),
                Syntax::Equal
                    | Syntax::NotEqual
                    | Syntax::Less
                    | Syntax::LessEqual
                    | Syntax::Greater
                    | Syntax::GreaterEqual
                    | Syntax::InKeyword
                    | Syntax::Assign
            )
        })
    }
}
