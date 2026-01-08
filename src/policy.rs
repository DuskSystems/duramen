use syntree::{FlavorDefault, Node, Tree};

mod lexer;
pub use lexer::{PolicyLexer, PolicyToken};

mod syntax;
pub use syntax::PolicyKind;

pub type PolicyTree = Tree<PolicyKind, FlavorDefault>;
pub type PolicyNode<'a> = Node<'a, PolicyKind, FlavorDefault>;
