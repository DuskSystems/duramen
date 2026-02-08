mod addition_operator;
pub use addition_operator::AdditionOperator;

mod and_expression;
pub use and_expression::AndExpression;

mod argument_list;
pub use argument_list::ArgumentList;

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

mod field_access;
pub use field_access::FieldAccess;

mod has_expression;
pub use has_expression::HasExpression;

mod if_expression;
pub use if_expression::IfExpression;

mod index_access;
pub use index_access::IndexAccess;

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

mod method_call;
pub use method_call::MethodCall;

mod multiplication_operator;
pub use multiplication_operator::MultiplicationOperator;

mod or_expression;
pub use or_expression::OrExpression;

mod parenthesized;
pub use parenthesized::Parenthesized;

mod policies;
pub use policies::Policies;

mod product_expression;
pub use product_expression::ProductExpression;

mod record_entry;
pub use record_entry::RecordEntry;

mod record;
pub use record::Record;

mod relation_expression;
pub use relation_expression::RelationExpression;

mod relational_operator;
pub use relational_operator::RelationalOperator;

mod slot;
pub use slot::Slot;

mod slot_kind;
pub use slot_kind::SlotKind;

mod sum_expression;
pub use sum_expression::SumExpression;

mod unary_expression;
pub use unary_expression::UnaryExpression;

mod unary_operator;
pub use unary_operator::UnaryOperator;

mod variable;
pub use variable::Variable;

mod variable_definition;
pub use variable_definition::VariableDefinition;

use crate::{CstNode, Node, Syntax};

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
