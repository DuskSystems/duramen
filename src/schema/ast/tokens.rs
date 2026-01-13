use super::{AstToken, SchemaNode, SchemaSyntax, ast_token};

ast_token!(StringToken, SchemaSyntax::String);
ast_token!(IntegerToken, SchemaSyntax::Integer);
ast_token!(IdentifierToken, SchemaSyntax::Identifier);
