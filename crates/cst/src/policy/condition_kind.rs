#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum ConditionKind {
    When,
    Unless,
}
