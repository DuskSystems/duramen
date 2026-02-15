use duramen_diagnostic::Diagnostics;
use duramen_lexer::TokenKind;
use duramen_syntax::{Group, Tree};

use crate::common::Parser;
use crate::error::ParseError;

/// Parses Cedar schema source text into a concrete syntax tree.
pub struct SchemaParser<'src> {
    parser: Parser<'src>,
}

impl<'src> SchemaParser<'src> {
    /// Parses the source text and returns the tree and diagnostics.
    #[must_use]
    pub fn parse(source: &'src str) -> (Tree<'src>, Diagnostics) {
        let mut this = Self {
            parser: Parser::new(source),
        };

        this.schema();

        let tree = this.parser.builder.build(this.parser.source);
        let diagnostics = this.parser.diagnostics;
        (tree, diagnostics)
    }

    /// Parses a schema file.
    ///
    /// ```cedarschema
    /// namespace Acme { entity User; }
    /// ```
    fn schema(&mut self) {
        let branch = self.parser.builder.open(Group::Schema);
        self.parser.next();

        while !self.parser.at(&[TokenKind::Eof]) {
            self.parser.advance_push();
            self.namespace();
            self.parser.advance_pop();
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
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            self.parser.next();
            self.parser.builder.close(&err);
            self.parser.diagnostics.push(ParseError::Unexpected {
                span: start..self.parser.position,
            });

            return;
        }

        if self.parser.at(&[TokenKind::NamespaceKeyword]) {
            self.parser.next();
            self.name();
            self.namespace_body();
            self.parser
                .builder
                .commit(&checkpoint, Group::NamespaceDeclaration);

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
        let branch = self.parser.builder.open(Group::NamespaceDeclaration);

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
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            self.parser.next();
            self.parser.builder.close(&err);
            self.parser.diagnostics.push(ParseError::Unexpected {
                span: start..self.parser.position,
            });

            return;
        }

        self.parser.next();
        while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBrace]) {
            self.parser.advance_push();
            self.declaration();
            self.parser.advance_pop();
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
            let start = self.parser.position;
            let checkpoint = self.parser.builder.checkpoint();

            while self.parser.at(&[TokenKind::At]) {
                self.parser.advance_push();
                self.parser.annotation();
                self.parser.advance_pop();
            }

            match self.parser.kind() {
                TokenKind::EntityKeyword => {
                    self.entity_declaration_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Group::EntityDeclaration);
                }
                TokenKind::ActionKeyword => {
                    self.action_declaration_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Group::ActionDeclaration);
                }
                TokenKind::TypeKeyword => {
                    self.type_declaration_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Group::TypeDeclaration);
                }
                TokenKind::NamespaceKeyword => {
                    self.nested_namespace_body();
                    self.parser
                        .builder
                        .commit(&checkpoint, Group::NamespaceDeclaration);
                }
                _ => {
                    self.parser.builder.commit(&checkpoint, Group::Error);
                    self.parser.diagnostics.push(ParseError::Unexpected {
                        span: start..self.parser.position,
                    });
                }
            }

            return;
        }

        let start = self.parser.position;
        let err = self.parser.builder.open(Group::Error);
        while !self.parser.at(&[
            TokenKind::Eof,
            TokenKind::EntityKeyword,
            TokenKind::ActionKeyword,
            TokenKind::TypeKeyword,
            TokenKind::NamespaceKeyword,
            TokenKind::At,
            TokenKind::CloseBrace,
        ]) {
            self.parser.advance_push();
            self.parser.next();
            self.parser.advance_pop();
        }

        self.parser.builder.close(&err);
        self.parser.diagnostics.push(ParseError::Unexpected {
            span: start..self.parser.position,
        });
    }

    /// Parses an entity declaration.
    ///
    /// ```cedarschema
    /// entity User in [UserGroup] { department: String, jobLevel: Long };
    /// ```
    fn entity_declaration(&mut self) {
        let branch = self.parser.builder.open(Group::EntityDeclaration);

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
                let branch = self.parser.builder.open(Group::EnumType);

                self.parser.next();
                self.enum_variants();
                self.parser.expect(TokenKind::CloseBracket);

                self.parser.builder.close(&branch);
            }
        } else {
            self.parser.expect(TokenKind::Equals);
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
        let branch = self.parser.builder.open(Group::EntityParents);

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
        let branch = self.parser.builder.open(Group::EntityAttributes);

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
            self.parser.advance_push();
            self.attribute_declaration();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser.advance_pop();
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
        let branch = self.parser.builder.open(Group::EntityTags);

        self.parser.next();
        self.type_expression();

        self.parser.builder.close(&branch);
    }

    /// Parses an action declaration.
    ///
    /// ```cedarschema
    /// action view appliesTo { principal: [User], resource: [Photo] };
    /// ```
    fn action_declaration(&mut self) {
        let branch = self.parser.builder.open(Group::ActionDeclaration);

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
        let branch = self.parser.builder.open(Group::ActionParents);

        self.parser.next();
        if !self.parser.at(&[TokenKind::OpenBracket]) {
            self.qualified_name();
            self.parser.builder.close(&branch);

            return;
        }

        self.parser.next();
        while !self.parser.at(&[TokenKind::CloseBracket, TokenKind::Eof]) {
            self.parser.advance_push();
            self.qualified_name();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser.advance_pop();
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
        let branch = self.parser.builder.open(Group::AppliesToClause);
        self.parser.next();

        if self.parser.eat(TokenKind::OpenBrace) {
            while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBrace]) {
                self.parser.advance_push();
                self.applies_to_entry();
                let comma = self.parser.eat(TokenKind::Comma);
                self.parser.advance_pop();
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
            TokenKind::PrincipalKeyword => (Group::PrincipalTypes, false),
            TokenKind::ResourceKeyword => (Group::ResourceTypes, false),
            TokenKind::ContextKeyword => (Group::ContextType, true),
            _ => {
                let start = self.parser.position;
                let err = self.parser.builder.open(Group::Error);
                self.parser.next();
                self.parser.builder.close(&err);
                self.parser.diagnostics.push(ParseError::Unexpected {
                    span: start..self.parser.position,
                });

                return;
            }
        };

        let branch = self.parser.builder.open(syntax);

        self.parser.next();
        if self.parser.eat(TokenKind::Colon) {
            if is_context {
                self.type_expression();
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
        let branch = self.parser.builder.open(Group::ActionAttributes);

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
        let branch = self.parser.builder.open(Group::TypeDeclaration);
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
        self.type_expression();
        self.parser.expect(TokenKind::Semicolon);
    }

    /// Parses a type expression.
    ///
    /// ```cedarschema
    /// Set<User>
    /// ```
    fn type_expression(&mut self) {
        if !self.parser.depth_push() {
            return;
        }

        self.type_expression_inner();
        self.parser.depth_pop();
    }

    /// Parses the inner type expression after the depth guard.
    fn type_expression_inner(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.at(&[TokenKind::SetKeyword])
            && self.parser.lexer.peek_kind() == Some(TokenKind::LessThan)
        {
            self.parser.next();
            self.parser.next();
            self.type_expression();
            self.parser.expect(TokenKind::GreaterThan);
            self.parser.builder.commit(&checkpoint, Group::SetType);

            return;
        }

        if self.parser.at(&[TokenKind::EnumKeyword])
            && self.parser.lexer.peek_kind() == Some(TokenKind::OpenBracket)
        {
            self.parser.next();
            self.parser.next();
            self.enum_variants();
            self.parser.expect(TokenKind::CloseBracket);
            self.parser.builder.commit(&checkpoint, Group::EnumType);

            return;
        }

        if self.parser.at(&[TokenKind::OpenBrace]) {
            self.parser.next();
            self.attribute_entries();
            self.parser.expect(TokenKind::CloseBrace);
            self.parser.builder.commit(&checkpoint, Group::RecordType);

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
            self.parser.advance_push();
            self.parser.annotation();
            self.parser.advance_pop();
        }

        if self.parser.at(&[TokenKind::String]) || self.parser.kind().is_identifier() {
            self.parser.next();
            self.parser.eat(TokenKind::QuestionMark);
            if self.parser.eat(TokenKind::Colon) {
                self.type_expression();
            }
        }

        if !self
            .parser
            .at(&[TokenKind::Eof, TokenKind::CloseBrace, TokenKind::Comma])
        {
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            while !self
                .parser
                .at(&[TokenKind::Eof, TokenKind::CloseBrace, TokenKind::Comma])
            {
                self.parser.advance_push();
                self.parser.next();
                self.parser.advance_pop();
            }

            self.parser.builder.close(&err);
            self.parser.diagnostics.push(ParseError::Unexpected {
                span: start..self.parser.position,
            });
        }

        self.parser
            .builder
            .commit(&checkpoint, Group::AttributeDeclaration);
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

        let branch = self.parser.builder.open(Group::Types);

        self.parser.next();
        while !self.parser.at(&[TokenKind::CloseBracket, TokenKind::Eof]) {
            self.parser.advance_push();
            self.name();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser.advance_pop();

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

            self.parser.advance_push();
            self.parser.next();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser.advance_pop();

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
                .commit(&checkpoint, Group::EntityReference);
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

                self.parser.advance_push();
                self.parser.next();
                let ident = self.parser.kind().is_identifier();
                if ident {
                    self.parser.next();
                }

                self.parser.advance_pop();
                if !ident {
                    break;
                }
            }

            self.parser.builder.commit(&checkpoint, Group::Name);
            return;
        }

        if !self.parser.at(&[TokenKind::Eof]) {
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            self.parser.next();
            self.parser.builder.close(&err);
            self.parser.diagnostics.push(ParseError::Unexpected {
                span: start..self.parser.position,
            });
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
            self.parser.advance_push();
            self.parser.next();
            self.name();
            self.parser.advance_pop();
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
            self.parser.advance_push();
            self.parser.next();
            self.action_name();
            self.parser.advance_pop();
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
            self.parser.builder.commit(&checkpoint, Group::Name);

            return;
        }

        if !self.parser.at(&[TokenKind::Eof]) {
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            self.parser.next();
            self.parser.builder.close(&err);
            self.parser.diagnostics.push(ParseError::Unexpected {
                span: start..self.parser.position,
            });
        }
    }
}
