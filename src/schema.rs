use syntree::{FlavorDefault, Node, Tree};

mod syntax;
pub use syntax::SchemaKind;

pub type SyntaxTree = Tree<SchemaKind, FlavorDefault>;
pub type SyntaxNode<'a> = Node<'a, SchemaKind, FlavorDefault>;
