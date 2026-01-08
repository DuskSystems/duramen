use syntree::{FlavorDefault, Node, Tree};

mod syntax;
pub use syntax::PolicyKind;

pub type SyntaxTree = Tree<PolicyKind, FlavorDefault>;
pub type SyntaxNode<'a> = Node<'a, PolicyKind, FlavorDefault>;
