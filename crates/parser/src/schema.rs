use alloc::string::String;

use duramen_cst::{SchemaBuilder, SchemaSyntax, SchemaTree};
use duramen_lexer::{Lexer, Token, TokenKind};

use crate::advance::Advance;

#[derive(Debug)]
pub struct SchemaParseResult {
    tree: SchemaTree,
}

impl SchemaParseResult {
    #[must_use]
    pub fn print(&self, source: &str) -> String {
        let mut output = String::with_capacity(self.tree.capacity());

        for node in self.tree.children() {
            let range = node.range();
            if let Some(text) = source.get(range) {
                output.push_str(text);
            }
        }

        output
    }
}

pub struct SchemaParser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    position: usize,
    builder: SchemaBuilder,
    advance: Advance,
}

impl<'a> SchemaParser<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            current: Token::new(TokenKind::Unknown, 0),
            position: 0,
            builder: SchemaBuilder::new(),
            advance: Advance::new(),
        }
    }

    /// # Panics
    ///
    /// Panics on syntree error.
    #[must_use]
    #[expect(clippy::expect_used, reason = "TODO")]
    pub fn parse(mut self) -> SchemaParseResult {
        self.bump().expect("tree construction failed");
        self.schema().expect("tree construction failed");

        let tree = self.builder.build().expect("tree construction failed");
        SchemaParseResult { tree }
    }

    /// Consumes the current token and advances to the next non trivial token.
    fn bump(&mut self) -> Result<(), duramen_cst::Error> {
        if self.current.len > 0 {
            self.builder
                .token(SchemaSyntax::from(self.current.kind), self.current.len)?;

            self.position = self
                .position
                .checked_add(self.current.len)
                .ok_or(duramen_cst::Error::Overflow)?;
        }

        self.current = loop {
            match self.lexer.next() {
                Some(token) if token.kind.is_trivial() => {
                    self.builder
                        .token(SchemaSyntax::from(token.kind), token.len)?;
                    self.position = self
                        .position
                        .checked_add(token.len)
                        .ok_or(duramen_cst::Error::Overflow)?;
                }
                Some(token) => break token,
                None => break Token::new(TokenKind::Unknown, 0),
            }
        };

        Ok(())
    }

    /// Parses a schema file.
    fn schema(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::Schema)?;

        while self.current.len > 0 {
            self.advance.push(self.position);
            self.namespace()?;
            self.advance.pop(self.position, self.current.kind);
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses a namespace.
    ///
    /// ```cedarschema
    /// namespace Foo { entity Bar; }
    /// ```
    fn namespace(&mut self) -> Result<(), duramen_cst::Error> {
        let checkpoint = self.builder.checkpoint()?;

        if self.current.kind == TokenKind::CloseBrace {
            self.builder.open(SchemaSyntax::Error)?;
            self.bump()?;
            self.builder.close()?;
            return Ok(());
        }

        if self.current.kind == TokenKind::Namespace {
            self.bump()?;
            self.name()?;

            if self.current.kind == TokenKind::OpenBrace {
                self.bump()?;

                while self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
                    self.advance.push(self.position);
                    self.declaration()?;
                    self.advance.pop(self.position, self.current.kind);
                }

                if self.current.kind == TokenKind::CloseBrace {
                    self.bump()?;
                }
            }

            self.builder
                .close_at(&checkpoint, SchemaSyntax::NamespaceDeclaration)?;
            Ok(())
        } else {
            self.declaration()
        }
    }

    /// Parses a declaration: entity, action, or type.
    fn declaration(&mut self) -> Result<(), duramen_cst::Error> {
        while self.current.kind == TokenKind::At {
            self.advance.push(self.position);
            self.annotation()?;
            self.advance.pop(self.position, self.current.kind);
        }

        if self.current.kind == TokenKind::Entity {
            self.entity_declaration()
        } else if self.current.kind == TokenKind::Action {
            self.action_declaration()
        } else if self.current.kind == TokenKind::Type {
            self.type_declaration()
        } else {
            self.builder.open(SchemaSyntax::Error)?;
            while self.current.len > 0 {
                if self.current.kind == TokenKind::Entity
                    || self.current.kind == TokenKind::Action
                    || self.current.kind == TokenKind::Type
                    || self.current.kind == TokenKind::At
                    || self.current.kind == TokenKind::CloseBrace
                {
                    break;
                }

                self.advance.push(self.position);
                self.bump()?;
                self.advance.pop(self.position, self.current.kind);
            }

            self.builder.close()?;
            Ok(())
        }
    }

    /// Parses an annotation.
    ///
    /// ```cedarschema
    /// @doc("description")
    /// ```
    fn annotation(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::Annotation)?;

        self.bump()?;

        if self.current.kind.is_identifier() {
            self.bump()?;
        }

        if self.current.kind == TokenKind::OpenParen {
            self.bump()?;

            if self.current.kind == TokenKind::String {
                self.bump()?;
            }

            if self.current.kind == TokenKind::CloseParen {
                self.bump()?;
            }
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses an entity declaration.
    ///
    /// ```cedarschema
    /// entity User in [UserGroup] { department: String, jobLevel: Long };
    /// ```
    fn entity_declaration(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::EntityDeclaration)?;

        self.bump()?;

        self.name_list()?;

        if self.current.kind == TokenKind::In {
            self.entity_parents()?;
        }

        if self.current.kind == TokenKind::Enum {
            self.bump()?;

            if self.current.kind == TokenKind::OpenBracket {
                self.builder.open(SchemaSyntax::EnumType)?;
                self.bump()?;

                loop {
                    if self.current.kind == TokenKind::CloseBracket || self.current.len == 0 {
                        break;
                    }

                    if self.current.kind != TokenKind::String {
                        break;
                    }

                    self.advance.push(self.position);
                    self.builder.open(SchemaSyntax::EnumVariant)?;
                    self.bump()?;
                    self.builder.close()?;

                    if self.current.kind == TokenKind::Comma {
                        self.bump()?;
                        self.advance.pop(self.position, self.current.kind);
                    } else {
                        self.advance.pop(self.position, self.current.kind);
                        break;
                    }
                }

                if self.current.kind == TokenKind::CloseBracket {
                    self.bump()?;
                }

                self.builder.close()?;
            }
        } else {
            if self.current.kind == TokenKind::Eq {
                self.bump()?;
            }

            if self.current.kind == TokenKind::OpenBrace {
                self.entity_attributes()?;
            }

            if self.current.kind == TokenKind::Tags {
                self.entity_tags()?;
            }
        }

        if self.current.kind == TokenKind::Semicolon {
            self.bump()?;
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses entity parents.
    ///
    /// ```cedarschema
    /// in [UserGroup, Team]
    /// ```
    fn entity_parents(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::EntityParents)?;

        self.bump()?;
        self.type_list()?;

        self.builder.close()?;
        Ok(())
    }

    /// Parses entity attributes.
    ///
    /// ```cedarschema
    /// { department: String, jobLevel: Long }
    /// ```
    fn entity_attributes(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::EntityAttributes)?;

        self.bump()?;

        while self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
            self.advance.push(self.position);
            self.attribute_declaration()?;

            if self.current.kind == TokenKind::Comma {
                self.bump()?;
            } else {
                self.advance.pop(self.position, self.current.kind);
                break;
            }
            self.advance.pop(self.position, self.current.kind);
        }

        if self.current.kind == TokenKind::CloseBrace {
            self.bump()?;
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses entity tags.
    ///
    /// ```cedarschema
    /// tags String
    /// ```
    fn entity_tags(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::EntityTags)?;

        self.bump()?;
        self.type_expr()?;

        self.builder.close()?;
        Ok(())
    }

    /// Parses an action declaration.
    ///
    /// ```cedarschema
    /// action view appliesTo { principal: [User], resource: [Photo] };
    /// ```
    fn action_declaration(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::ActionDeclaration)?;

        self.bump()?;

        self.action_name_list()?;

        if self.current.kind == TokenKind::In {
            self.action_parents()?;
        }

        if self.current.kind == TokenKind::AppliesTo {
            self.applies_to_clause()?;
        }

        if self.current.kind == TokenKind::Attributes {
            self.action_attributes()?;
        }

        if self.current.kind == TokenKind::Semicolon {
            self.bump()?;
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses action parents.
    ///
    /// ```cedarschema
    /// in [Action::"read", Action::"write"]
    /// ```
    fn action_parents(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::ActionParents)?;

        self.bump()?;

        if self.current.kind == TokenKind::OpenBracket {
            self.bump()?;

            loop {
                if self.current.kind == TokenKind::CloseBracket || self.current.len == 0 {
                    break;
                }

                self.advance.push(self.position);
                self.qualified_name()?;

                if self.current.kind == TokenKind::Comma {
                    self.bump()?;
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }
                self.advance.pop(self.position, self.current.kind);
            }

            if self.current.kind == TokenKind::CloseBracket {
                self.bump()?;
            }
        } else {
            self.qualified_name()?;
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses an appliesTo clause.
    ///
    /// ```cedarschema
    /// appliesTo { principal: [User], resource: [Photo] }
    /// ```
    fn applies_to_clause(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::AppliesToClause)?;

        self.bump()?;

        if self.current.kind == TokenKind::OpenBrace {
            self.bump()?;

            while self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
                self.advance.push(self.position);
                self.applies_to_entry()?;

                if self.current.kind == TokenKind::Comma {
                    self.bump()?;
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }

                self.advance.pop(self.position, self.current.kind);
            }

            if self.current.kind == TokenKind::CloseBrace {
                self.bump()?;
            }
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses principal/resource/context entry in appliesTo.
    fn applies_to_entry(&mut self) -> Result<(), duramen_cst::Error> {
        if self.current.kind == TokenKind::Principal {
            self.builder.open(SchemaSyntax::PrincipalTypes)?;

            self.bump()?;

            if self.current.kind == TokenKind::Colon {
                self.bump()?;
                self.type_list()?;
            }

            self.builder.close()?;
        } else if self.current.kind == TokenKind::Resource {
            self.builder.open(SchemaSyntax::ResourceTypes)?;

            self.bump()?;

            if self.current.kind == TokenKind::Colon {
                self.bump()?;
                self.type_list()?;
            }

            self.builder.close()?;
        } else if self.current.kind == TokenKind::Context {
            self.builder.open(SchemaSyntax::ContextType)?;

            self.bump()?;

            if self.current.kind == TokenKind::Colon {
                self.bump()?;
                self.type_expr()?;
            }

            self.builder.close()?;
        } else {
            self.bump()?;
        }

        Ok(())
    }

    /// Parses action attributes.
    ///
    /// ```cedarschema
    /// attributes { }
    /// ```
    fn action_attributes(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::ActionAttributes)?;

        self.bump()?;

        if self.current.kind == TokenKind::OpenBrace {
            self.bump()?;

            if self.current.kind == TokenKind::CloseBrace {
                self.bump()?;
            }
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses a type declaration.
    ///
    /// ```cedarschema
    /// type Email = String;
    /// ```
    fn type_declaration(&mut self) -> Result<(), duramen_cst::Error> {
        self.builder.open(SchemaSyntax::TypeDeclaration)?;

        self.bump()?;

        if self.current.kind.is_identifier() {
            self.name()?;
        }

        if self.current.kind == TokenKind::Eq {
            self.bump()?;
        }

        self.type_expr()?;

        if self.current.kind == TokenKind::Semicolon {
            self.bump()?;
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses a type expression.
    ///
    /// ```cedarschema
    /// Set<User>
    /// ```
    fn type_expr(&mut self) -> Result<(), duramen_cst::Error> {
        let checkpoint = self.builder.checkpoint()?;

        if self.current.kind == TokenKind::Set {
            self.bump()?;

            if self.current.kind == TokenKind::Lt {
                self.bump()?;
                self.type_expr()?;

                if self.current.kind == TokenKind::Gt {
                    self.bump()?;
                }
            }

            self.builder.close_at(&checkpoint, SchemaSyntax::SetType)?;
        } else if self.current.kind == TokenKind::OpenBrace {
            self.bump()?;

            while self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
                self.advance.push(self.position);
                self.attribute_declaration()?;

                if self.current.kind == TokenKind::Comma {
                    self.bump()?;
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }

                self.advance.pop(self.position, self.current.kind);
            }

            if self.current.kind == TokenKind::CloseBrace {
                self.bump()?;
            }

            self.builder
                .close_at(&checkpoint, SchemaSyntax::RecordType)?;
        } else {
            self.name()?;
            self.builder
                .close_at(&checkpoint, SchemaSyntax::EntityType)?;
        }

        Ok(())
    }

    /// Parses an attribute declaration.
    ///
    /// ```cedarschema
    /// owner?: User
    /// ```
    fn attribute_declaration(&mut self) -> Result<(), duramen_cst::Error> {
        while self.current.kind == TokenKind::At {
            self.advance.push(self.position);
            self.annotation()?;
            self.advance.pop(self.position, self.current.kind);
        }

        self.builder.open(SchemaSyntax::AttributeDeclaration)?;

        if self.current.kind == TokenKind::String || self.current.kind.is_identifier() {
            self.bump()?;

            if self.current.kind == TokenKind::Question {
                self.bump()?;
            }

            if self.current.kind == TokenKind::Colon {
                self.bump()?;
                self.type_expr()?;
            }
        } else if self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
            self.builder.open(SchemaSyntax::Error)?;
            self.bump()?;
            self.builder.close()?;
        }

        self.builder.close()?;
        Ok(())
    }

    /// Parses a type list.
    ///
    /// ```cedarschema
    /// [User, Admin]
    /// ```
    fn type_list(&mut self) -> Result<(), duramen_cst::Error> {
        if self.current.kind == TokenKind::OpenBracket {
            self.builder.open(SchemaSyntax::TypeList)?;
            self.bump()?;

            loop {
                if self.current.kind == TokenKind::CloseBracket || self.current.len == 0 {
                    break;
                }

                self.advance.push(self.position);
                self.name()?;

                if self.current.kind == TokenKind::Comma {
                    self.bump()?;
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }
                self.advance.pop(self.position, self.current.kind);
            }

            if self.current.kind == TokenKind::CloseBracket {
                self.bump()?;
            }

            self.builder.close()?;
            Ok(())
        } else {
            self.name()
        }
    }

    /// Parses a qualified name.
    ///
    /// ```cedarschema
    /// Action::"view"
    /// ```
    fn qualified_name(&mut self) -> Result<(), duramen_cst::Error> {
        self.name()?;

        if self.current.kind == TokenKind::Colon2 {
            self.bump()?;

            if self.current.kind == TokenKind::String {
                self.bump()?;
            }
        } else if self.current.kind == TokenKind::String {
            self.bump()?;
        }

        Ok(())
    }

    /// Parses a name.
    ///
    /// ```cedarschema
    /// Foo::Bar::Baz
    /// ```
    fn name(&mut self) -> Result<(), duramen_cst::Error> {
        let checkpoint = self.builder.checkpoint()?;

        if self.current.kind.is_identifier() {
            self.bump()?;

            while self.current.kind == TokenKind::Colon2 {
                self.advance.push(self.position);
                self.bump()?;

                if self.current.kind.is_identifier() {
                    self.bump()?;
                    self.advance.pop(self.position, self.current.kind);
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }
            }

            self.builder.close_at(&checkpoint, SchemaSyntax::Name)?;
        } else if self.current.len > 0 {
            self.builder.open(SchemaSyntax::Error)?;
            self.bump()?;
            self.builder.close()?;
        }

        Ok(())
    }

    /// Parses comma-separated names.
    fn name_list(&mut self) -> Result<(), duramen_cst::Error> {
        self.name()?;

        while self.current.kind == TokenKind::Comma {
            self.advance.push(self.position);
            self.bump()?;
            self.name()?;
            self.advance.pop(self.position, self.current.kind);
        }

        Ok(())
    }

    /// Parses comma-separated action names (identifiers or strings).
    fn action_name_list(&mut self) -> Result<(), duramen_cst::Error> {
        self.action_name()?;

        while self.current.kind == TokenKind::Comma {
            self.advance.push(self.position);
            self.bump()?;
            self.action_name()?;
            self.advance.pop(self.position, self.current.kind);
        }

        Ok(())
    }

    /// Parses an action name (identifier or string).
    fn action_name(&mut self) -> Result<(), duramen_cst::Error> {
        let checkpoint = self.builder.checkpoint()?;

        if self.current.kind == TokenKind::String || self.current.kind.is_identifier() {
            self.bump()?;
            self.builder.close_at(&checkpoint, SchemaSyntax::Name)?;
        } else if self.current.len > 0 {
            self.builder.open(SchemaSyntax::Error)?;
            self.bump()?;
            self.builder.close()?;
        }

        Ok(())
    }
}
