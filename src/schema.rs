use syntree::{FlavorDefault, Node, Tree};

mod lexer;
pub use lexer::{SchemaLexer, SchemaToken};

mod syntax;
pub use syntax::SchemaKind;

pub type SchemaTree = Tree<SchemaKind, FlavorDefault>;
pub type SchemaNode<'a> = Node<'a, SchemaKind, FlavorDefault>;
