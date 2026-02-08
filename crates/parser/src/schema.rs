use duramen_diagnostic::Diagnostics;
use duramen_lexer::TokenKind;
use duramen_syntax::{Syntax, Tree};

use crate::common::Parser;
use crate::error::ParseError;

/// Parses Cedar schema source text into a concrete syntax tree.
pub struct SchemaParser<'a> {
    parser: Parser<'a>,
}

impl<'a> SchemaParser<'a> {
    /// Creates a new schema parser.
    #[must_use]
    pub const fn new(source: &'a str, diagnostics: &'a mut Diagnostics) -> Self {
        Self {
            parser: Parser::new(source, diagnostics),
        }
    }

    /// Parses the source text and returns the concrete syntax tree.
    #[must_use]
    pub fn parse(mut self) -> Tree {
        self.schema();
        self.parser.builder.build()
    }

    /// Parses a schema file.
    ///
    /// ```cedarschema
    /// namespace Acme { entity User; }
    /// ```
    fn schema(&mut self) {
        let branch = self.parser.builder.open(Syntax::Schema);
        self.parser.next();

        while !self.parser.at(&[TokenKind::Eof]) {
            self.parser.advance.push(self.parser.position);
            self.namespace();
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
        }

        self.parser.builder.close(&branch);
    }

    /// Parses a namespace.
    ///
    /// ```cedarschema
    /// namespace Foo { entity Bar; }
    /// ```
    fn namespace(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.at(&[TokenKind::CloseBrace]) {
            let err = self.parser.builder.open(Syntax::Error);
            self.parser.next();
            self.parser.builder.close(&err);

            return;
        }

        if self.parser.at(&[TokenKind::NamespaceKeyword]) {
            self.parser.next();
            self.name();
            self.namespace_body();
            self.parser
                .builder
                .commit(&checkpoint, Syntax::NamespaceDeclaration);

            return;
        }

        self.declaration();
    }

    /// Parses a nested namespace declaration.
    ///
    /// ```cedarschema
    /// namespace Inner { entity Foo; }
    /// ```
    fn nested_namespace(&mut self) {
        let branch = self.parser.builder.open(Syntax::NamespaceDeclaration);

        self.nested_namespace_body();

        self.parser.builder.close(&branch);
    }

    /// Parses the body of a nested namespace after the `namespace` keyword.
    fn nested_namespace_body(&mut self) {
        self.parser.next();
        self.name();
        self.namespace_body();
    }

    /// Parses the body of a namespace.
    ///
    /// ```cedarschema
    /// { entity User; action view; }
    /// ```
    fn namespace_body(&mut self) {
        if !self.parser.at(&[TokenKind::OpenBrace]) {
            let err = self.parser.builder.open(Syntax::Error);
            self.parser.next();
            self.parser.builder.close(&err);

            return;
        }

        self.parser.next();
        while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBrace]) {
            self.parser.advance.push(self.parser.position);
            self.declaration();
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
        }

        self.parser.expect(TokenKind::CloseBrace);
    }

    /// Parses a declaration (entity, action, type, or nested namespace).
    ///
    /// ```cedarschema
    /// entity User;
    /// ```
    fn declaration(&mut self) {
        if self.parser.at(&[TokenKind::EntityKeyword]) {
            self.entity_declaration();
            return;
        }

        if self.parser.at(&[TokenKind::ActionKeyword]) {
            self.action_declaration();
            return;
        }

        if self.parser.at(&[TokenKind::TypeKeyword]) {
            self.type_declaration();
            return;
        }

        if self.parser.at(&[TokenKind::NamespaceKeyword]) {
            self.nested_namespace();
            return;
        }

        if self.parser.at(&[TokenKind::At]) {
            let checkpoint = self.parser.builder.checkpoint();

            while self.parser.at(&[TokenKind::At]) {
                self.parser.advance.push(self.parser.position);
                self.parser.annotation();
                self.parser
                    .advance
                    .pop(self.parser.position, self.parser.current.kind);
            }

            match self.parser.kind() {
                TokenKind::EntityKeyword => {
                    self.entity_declaration_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Syntax::EntityDeclaration);
                }
                TokenKind::ActionKeyword => {
                    self.action_declaration_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Syntax::ActionDeclaration);
                }
                TokenKind::TypeKeyword => {
                    self.type_declaration_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Syntax::TypeDeclaration);
                }
                TokenKind::NamespaceKeyword => {
                    self.nested_namespace_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Syntax::NamespaceDeclaration);
                }
                _ => {
                    self.parser.builder.commit(&checkpoint, Syntax::Error);
                }
            }

            return;
        }

        let err = self.parser.builder.open(Syntax::Error);
        while !self.parser.at(&[
            TokenKind::Eof,
            TokenKind::EntityKeyword,
            TokenKind::ActionKeyword,
            TokenKind::TypeKeyword,
            TokenKind::NamespaceKeyword,
            TokenKind::At,
            TokenKind::CloseBrace,
        ]) {
            self.parser.advance.push(self.parser.position);
            self.parser.next();
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
        }

        self.parser.builder.close(&err);
    }

    /// Parses an entity declaration.
    ///
    /// ```cedarschema
    /// entity User in [UserGroup] { department: String, jobLevel: Long };
    /// ```
    fn entity_declaration(&mut self) {
        let branch = self.parser.builder.open(Syntax::EntityDeclaration);

        self.entity_declaration_body();

        self.parser.builder.close(&branch);
    }

    /// Parses the body of an entity declaration after the `entity` keyword.
    fn entity_declaration_body(&mut self) {
        self.parser.next();
        self.name_list();
        if self.parser.at(&[TokenKind::InKeyword]) {
            self.entity_parents();
        }

        if self.parser.eat(TokenKind::EnumKeyword) {
            if self.parser.at(&[TokenKind::OpenBracket]) {
                let branch = self.parser.builder.open(Syntax::EnumType);

                self.parser.next();
                self.enum_variants();
                self.parser.expect(TokenKind::CloseBracket);

                self.parser.builder.close(&branch);
            }
        } else {
            self.parser.eat(TokenKind::Equals);
            if self.parser.at(&[TokenKind::OpenBrace]) {
                self.entity_attributes();
            }

            if self.parser.at(&[TokenKind::TagsKeyword]) {
                self.entity_tags();
            }
        }

        self.parser.expect(TokenKind::Semicolon);
    }

    /// Parses entity parents.
    ///
    /// ```cedarschema
    /// in [UserGroup, Team]
    /// ```
    fn entity_parents(&mut self) {
        let branch = self.parser.builder.open(Syntax::EntityParents);

        self.parser.next();
        self.type_list();

        self.parser.builder.close(&branch);
    }

    /// Parses entity attributes.
    ///
    /// ```cedarschema
    /// { department: String, jobLevel: Long }
    /// ```
    fn entity_attributes(&mut self) {
        let branch = self.parser.builder.open(Syntax::EntityAttributes);

        self.parser.next();
        self.attribute_entries();
        self.parser.expect(TokenKind::CloseBrace);

        self.parser.builder.close(&branch);
    }

    /// Parses comma-separated attribute declarations inside braces.
    ///
    /// ```cedarschema
    /// name: String, age: Long
    /// ```
    fn attribute_entries(&mut self) {
        while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBrace]) {
            self.parser.advance.push(self.parser.position);
            self.attribute_declaration();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
            if !comma {
                break;
            }
        }
    }

    /// Parses entity tags.
    ///
    /// ```cedarschema
    /// tags String
    /// ```
    fn entity_tags(&mut self) {
        let branch = self.parser.builder.open(Syntax::EntityTags);

        self.parser.next();
        self.type_expr();

        self.parser.builder.close(&branch);
    }

    /// Parses an action declaration.
    ///
    /// ```cedarschema
    /// action view appliesTo { principal: [User], resource: [Photo] };
    /// ```
    fn action_declaration(&mut self) {
        let branch = self.parser.builder.open(Syntax::ActionDeclaration);

        self.action_declaration_body();

        self.parser.builder.close(&branch);
    }

    /// Parses the body of an action declaration after the `action` keyword.
    fn action_declaration_body(&mut self) {
        self.parser.next();
        self.action_name_list();

        if self.parser.at(&[TokenKind::InKeyword]) {
            self.action_parents();
        }

        if self.parser.at(&[TokenKind::AppliesToKeyword]) {
            self.applies_to_clause();
        }

        if self.parser.at(&[TokenKind::AttributesKeyword]) {
            self.action_attributes();
        }

        self.parser.expect(TokenKind::Semicolon);
    }

    /// Parses action parents.
    ///
    /// ```cedarschema
    /// in [Action::"read", Action::"write"]
    /// ```
    fn action_parents(&mut self) {
        let branch = self.parser.builder.open(Syntax::ActionParents);

        self.parser.next();
        if !self.parser.at(&[TokenKind::OpenBracket]) {
            self.qualified_name();
            self.parser.builder.close(&branch);

            return;
        }

        self.parser.next();
        while !self.parser.at(&[TokenKind::CloseBracket, TokenKind::Eof]) {
            self.parser.advance.push(self.parser.position);
            self.qualified_name();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
            if !comma {
                break;
            }
        }

        self.parser.expect(TokenKind::CloseBracket);
        self.parser.builder.close(&branch);
    }

    /// Parses an appliesTo clause.
    ///
    /// ```cedarschema
    /// appliesTo { principal: [User], resource: [Photo] }
    /// ```
    fn applies_to_clause(&mut self) {
        let branch = self.parser.builder.open(Syntax::AppliesToClause);
        self.parser.next();

        if self.parser.eat(TokenKind::OpenBrace) {
            while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBrace]) {
                self.parser.advance.push(self.parser.position);
                self.applies_to_entry();
                let comma = self.parser.eat(TokenKind::Comma);
                self.parser
                    .advance
                    .pop(self.parser.position, self.parser.current.kind);
                if !comma {
                    break;
                }
            }

            self.parser.expect(TokenKind::CloseBrace);
        }

        self.parser.builder.close(&branch);
    }

    /// Parses principal/resource/context entry in appliesTo.
    ///
    /// ```cedarschema
    /// principal: [User]
    /// ```
    fn applies_to_entry(&mut self) {
        let (syntax, is_context) = match self.parser.kind() {
            TokenKind::PrincipalKeyword => (Syntax::PrincipalTypes, false),
            TokenKind::ResourceKeyword => (Syntax::ResourceTypes, false),
            TokenKind::ContextKeyword => (Syntax::ContextType, true),
            _ => {
                let err = self.parser.builder.open(Syntax::Error);
                self.parser.next();
                self.parser.builder.close(&err);

                return;
            }
        };

        let branch = self.parser.builder.open(syntax);

        self.parser.next();
        if self.parser.eat(TokenKind::Colon) {
            if is_context {
                self.type_expr();
            } else {
                self.type_list();
            }
        }

        self.parser.builder.close(&branch);
    }

    /// Parses action attributes.
    ///
    /// ```cedarschema
    /// attributes { name: String }
    /// ```
    fn action_attributes(&mut self) {
        let branch = self.parser.builder.open(Syntax::ActionAttributes);

        self.parser.next();
        if self.parser.eat(TokenKind::OpenBrace) {
            self.attribute_entries();
            self.parser.expect(TokenKind::CloseBrace);
        }

        self.parser.builder.close(&branch);
    }

    /// Parses a type declaration.
    ///
    /// ```cedarschema
    /// type Email = String;
    /// ```
    fn type_declaration(&mut self) {
        let branch = self.parser.builder.open(Syntax::TypeDeclaration);
        self.type_declaration_body();
        self.parser.builder.close(&branch);
    }

    /// Parses the body of a type declaration after the `type` keyword.
    fn type_declaration_body(&mut self) {
        self.parser.next();

        if self.parser.kind().is_identifier() {
            self.name();
        }

        self.parser.expect(TokenKind::Equals);
        self.type_expr();
        self.parser.expect(TokenKind::Semicolon);
    }

    /// Parses a type expression.
    ///
    /// ```cedarschema
    /// Set<User>
    /// ```
    fn type_expr(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.at(&[TokenKind::SetKeyword])
            && self.parser.lexer.peek_kind() == Some(TokenKind::LessThan)
        {
            self.parser.next();
            self.parser.next();
            self.type_expr();
            self.parser.expect(TokenKind::GreaterThan);
            self.parser.builder.commit(&checkpoint, Syntax::SetType);

            return;
        }

        if self.parser.at(&[TokenKind::EnumKeyword])
            && self.parser.lexer.peek_kind() == Some(TokenKind::OpenBracket)
        {
            self.parser.next();
            self.parser.next();
            self.enum_variants();
            self.parser.expect(TokenKind::CloseBracket);
            self.parser.builder.commit(&checkpoint, Syntax::EnumType);

            return;
        }

        if self.parser.at(&[TokenKind::OpenBrace]) {
            self.parser.next();
            self.attribute_entries();
            self.parser.expect(TokenKind::CloseBrace);
            self.parser.builder.commit(&checkpoint, Syntax::RecordType);

            return;
        }

        self.name();
    }

    /// Parses an attribute declaration.
    ///
    /// ```cedarschema
    /// owner?: User
    /// ```
    fn attribute_declaration(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        while self.parser.at(&[TokenKind::At]) {
            self.parser.advance.push(self.parser.position);
            self.parser.annotation();
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
        }

        if self.parser.at(&[TokenKind::String]) || self.parser.kind().is_identifier() {
            self.parser.next();
            self.parser.eat(TokenKind::QuestionMark);
            if self.parser.eat(TokenKind::Colon) {
                self.type_expr();
            }
        }

        if !self
            .parser
            .at(&[TokenKind::Eof, TokenKind::CloseBrace, TokenKind::Comma])
        {
            let err = self.parser.builder.open(Syntax::Error);
            while !self
                .parser
                .at(&[TokenKind::Eof, TokenKind::CloseBrace, TokenKind::Comma])
            {
                self.parser.advance.push(self.parser.position);
                self.parser.next();
                self.parser
                    .advance
                    .pop(self.parser.position, self.parser.current.kind);
            }

            self.parser.builder.close(&err);
        }

        self.parser
            .builder
            .commit(&checkpoint, Syntax::AttributeDeclaration);
    }

    /// Parses a type list.
    ///
    /// ```cedarschema
    /// [User, Admin]
    /// ```
    fn type_list(&mut self) {
        if !self.parser.at(&[TokenKind::OpenBracket]) {
            self.name();
            return;
        }

        let branch = self.parser.builder.open(Syntax::Types);

        self.parser.next();
        while !self.parser.at(&[TokenKind::CloseBracket, TokenKind::Eof]) {
            self.parser.advance.push(self.parser.position);
            self.name();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
            if !comma {
                break;
            }
        }

        self.parser.expect(TokenKind::CloseBracket);
        self.parser.builder.close(&branch);
    }

    /// Parses enum variants inside brackets.
    ///
    /// ```cedarschema
    /// "active", "inactive"
    /// ```
    fn enum_variants(&mut self) {
        while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBracket]) {
            if !self.parser.at(&[TokenKind::String]) {
                break;
            }

            self.parser.advance.push(self.parser.position);
            self.parser.next();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
            if !comma {
                break;
            }
        }
    }

    /// Parses a qualified name.
    ///
    /// ```cedarschema
    /// Action::"view"
    /// ```
    fn qualified_name(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.eat(TokenKind::String) {
            return;
        }

        self.name();
        if self.parser.at(&[TokenKind::Colon2]) {
            self.parser.next();
            self.parser.eat(TokenKind::String);
            self.parser
                .builder
                .commit(&checkpoint, Syntax::EntityReference);
        }
    }

    /// Parses a name.
    ///
    /// ```cedarschema
    /// Foo::Bar::Baz
    /// ```
    fn name(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.kind().is_identifier() {
            self.parser.next();
            while self.parser.at(&[TokenKind::Colon2]) {
                if self.parser.lexer.peek_kind() == Some(TokenKind::String) {
                    break;
                }

                self.parser.advance.push(self.parser.position);
                self.parser.next();
                let ident = self.parser.kind().is_identifier();
                if ident {
                    self.parser.next();
                }

                self.parser
                    .advance
                    .pop(self.parser.position, self.parser.current.kind);
                if !ident {
                    break;
                }
            }

            self.parser.builder.commit(&checkpoint, Syntax::Name);
            return;
        }

        if !self.parser.at(&[TokenKind::Eof]) {
            self.parser.diagnostics.push(ParseError::UnexpectedToken {
                span: self.parser.span(),
            });

            let err = self.parser.builder.open(Syntax::Error);
            self.parser.next();
            self.parser.builder.close(&err);
        }
    }

    /// Parses comma-separated names.
    ///
    /// ```cedarschema
    /// User, Admin, Guest
    /// ```
    fn name_list(&mut self) {
        self.name();
        while self.parser.at(&[TokenKind::Comma]) {
            self.parser.advance.push(self.parser.position);
            self.parser.next();
            self.name();
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
        }
    }

    /// Parses comma-separated action names (identifiers or strings).
    ///
    /// ```cedarschema
    /// view, "edit", delete
    /// ```
    fn action_name_list(&mut self) {
        self.action_name();
        while self.parser.at(&[TokenKind::Comma]) {
            self.parser.advance.push(self.parser.position);
            self.parser.next();
            self.action_name();
            self.parser
                .advance
                .pop(self.parser.position, self.parser.current.kind);
        }
    }

    /// Parses an action name (identifier or string).
    ///
    /// ```cedarschema
    /// view
    /// ```
    fn action_name(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.at(&[TokenKind::String]) || self.parser.kind().is_identifier() {
            self.parser.next();
            self.parser.builder.commit(&checkpoint, Syntax::Name);

            return;
        }

        if !self.parser.at(&[TokenKind::Eof]) {
            self.parser.diagnostics.push(ParseError::UnexpectedToken {
                span: self.parser.span(),
            });

            let err = self.parser.builder.open(Syntax::Error);
            self.parser.next();
            self.parser.builder.close(&err);
        }
    }
}
