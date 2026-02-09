use crate::{Annotation, CstNode, Node, Syntax};

mod and_expression;
pub use and_expression::AndExpression;

mod arguments;
pub use arguments::Arguments;

mod call;
pub use call::Call;

mod condition;
pub use condition::Condition;

mod condition_kind;
pub use condition_kind::ConditionKind;

mod effect;
pub use effect::Effect;

mod entity_reference;
pub use entity_reference::EntityReference;

mod expression;
pub use expression::Expression;

mod field;
pub use field::Field;

mod has_expression;
pub use has_expression::HasExpression;

mod if_expression;
pub use if_expression::IfExpression;

mod index;
pub use index::Index;

mod is_expression;
pub use is_expression::IsExpression;

mod like_expression;
pub use like_expression::LikeExpression;

mod list;
pub use list::List;

mod literal;
pub use literal::Literal;

mod literal_kind;
pub use literal_kind::LiteralKind;

mod member_access;
pub use member_access::MemberAccess;

mod member_expression;
pub use member_expression::MemberExpression;

mod or_expression;
pub use or_expression::OrExpression;

mod parenthesized;
pub use parenthesized::Parenthesized;

mod policies;
pub use policies::Policies;

mod product_expression;
pub use product_expression::ProductExpression;

mod product_operator;
pub use product_operator::ProductOperator;

mod record;
pub use record::Record;

mod record_entry;
pub use record_entry::RecordEntry;

mod relation_expression;
pub use relation_expression::RelationExpression;

mod relation_operator;
pub use relation_operator::RelationOperator;

mod slot;
pub use slot::Slot;

mod slot_kind;
pub use slot_kind::SlotKind;

mod sum_expression;
pub use sum_expression::SumExpression;

mod sum_operator;
pub use sum_operator::SumOperator;

mod unary_expression;
pub use unary_expression::UnaryExpression;

mod unary_operator;
pub use unary_operator::UnaryOperator;

mod variable;
pub use variable::Variable;

mod variable_definition;
pub use variable_definition::VariableDefinition;

#[derive(Clone, Copy, Debug)]
pub struct Policy<'a> {
    node: Node<'a>,
}

impl<'a> CstNode<'a> for Policy<'a> {
    fn cast(node: Node<'a>) -> Option<Self> {
        match node.kind() {
            Syntax::Policy => Some(Self { node }),
            _ => None,
        }
    }

    fn syntax(&self) -> Node<'a> {
        self.node
    }
}

impl<'a> Policy<'a> {
    /// Returns an iterator over the annotation children.
    pub fn annotations(&self) -> impl Iterator<Item = Annotation<'a>> {
        self.node.children().filter_map(Annotation::cast)
    }

    /// Returns the effect (`permit` or `forbid`).
    #[must_use]
    pub fn effect(&self) -> Option<Effect> {
        self.node.children().find_map(|child| match child.kind() {
            Syntax::PermitKeyword => Some(Effect::Permit),
            Syntax::ForbidKeyword => Some(Effect::Forbid),
            _ => None,
        })
    }

    /// Returns the effect keyword token.
    #[must_use]
    pub fn effect_token(&self) -> Option<Node<'a>> {
        self.node
            .children()
            .find(|child| matches!(child.kind(), Syntax::PermitKeyword | Syntax::ForbidKeyword))
    }

    /// Returns an iterator over the variable definition children.
    pub fn variable_definitions(&self) -> impl Iterator<Item = VariableDefinition<'a>> {
        self.node.children().filter_map(VariableDefinition::cast)
    }

    /// Returns an iterator over the condition children.
    pub fn conditions(&self) -> impl Iterator<Item = Condition<'a>> {
        self.node.children().filter_map(Condition::cast)
    }

    /// Returns the opening parenthesis token.
    #[must_use]
    pub fn open_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::OpenParenthesis)
    }

    /// Returns the closing parenthesis token.
    #[must_use]
    pub fn close_parenthesis(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::CloseParenthesis)
    }

    /// Returns the semicolon token.
    #[must_use]
    pub fn semicolon(&self) -> Option<Node<'a>> {
        self.node.child(Syntax::Semicolon)
    }
}
