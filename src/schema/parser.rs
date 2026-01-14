use alloc::string::ToString as _;

use smallvec::SmallVec;
use syntree::{Builder, Tree};

use super::Schema;
use super::lexer::{SchemaLexer, SchemaToken};
use super::syntax::SchemaSyntax;
use crate::diagnostics::Diagnostic;

pub struct SchemaParser<'a> {
    source: &'a str,
    lexer: SchemaLexer<'a>,
    current: SchemaToken<'a>,
    builder: Builder<SchemaSyntax>,
    diagnostics: SmallVec<[Diagnostic; 4]>,
    #[cfg(debug_assertions)]
    advances: SmallVec<[usize; 4]>,
}

impl<'a> SchemaParser<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        let mut lexer = SchemaLexer::new(source);
        let current = lexer.next_token();

        Self {
            source,
            lexer,
            current,
            builder: Builder::new(),
            diagnostics: SmallVec::new_const(),
            #[cfg(debug_assertions)]
            advances: SmallVec::new_const(),
        }
    }

    #[must_use]
    pub fn parse(mut self) -> Schema<'a> {
        if let Err(err) = self.schema() {
            self.diagnostics.push(Diagnostic::error(err.to_string()));
        }

        let mut diagnostics = self.lexer.take_diagnostics();
        diagnostics.extend(self.diagnostics);

        let tree = match self.builder.build() {
            Ok(tree) => tree,
            Err(err) => {
                diagnostics.push(Diagnostic::error(err.to_string()));
                Tree::default()
            }
        };

        Schema::new(self.source, tree, diagnostics)
    }

    const fn current(&self) -> SchemaSyntax {
        self.current.syntax()
    }

    fn at(&self, kind: SchemaSyntax) -> bool {
        self.current.syntax() == kind
    }

    fn at_any(&self, kinds: &[SchemaSyntax]) -> bool {
        kinds.contains(&self.current.syntax())
    }

    fn bump(&mut self) -> Result<(), syntree::Error> {
        let token = self.current;
        self.builder.token(token.syntax(), token.text().len())?;
        self.current = self.lexer.next_token();

        Ok(())
    }

    fn skip_trivia(&mut self) -> Result<(), syntree::Error> {
        while self.current().is_trivial() {
            self.bump()?;
        }

        Ok(())
    }

    fn expect(&mut self, kind: SchemaSyntax) -> Result<(), syntree::Error> {
        self.skip_trivia()?;
        if self.at(kind) { self.bump() } else { Ok(()) }
    }

    fn schema(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::Schema)?;

        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            if self.at(SchemaSyntax::NamespaceKeyword) {
                self.namespace()?;
            } else if self.at(SchemaSyntax::At) {
                self.annotated_decl()?;
            } else if self.at_decl_start() {
                self.decl()?;
            } else {
                self.builder.open(SchemaSyntax::Error)?;
                self.bump()?;
                self.builder.close()?;
            }
            self.advance_pop();
        }

        self.builder.close()?;
        Ok(())
    }

    fn namespace(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::Namespace)?;
        self.expect(SchemaSyntax::NamespaceKeyword)?;

        self.skip_trivia()?;

        self.path()?;
        self.expect(SchemaSyntax::OpenBrace)?;

        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::CloseBrace) || self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            if self.at(SchemaSyntax::At) {
                self.annotated_decl()?;
            } else if self.at_decl_start() {
                self.decl()?;
            } else {
                self.builder.open(SchemaSyntax::Error)?;
                self.bump()?;
                self.builder.close()?;
            }

            self.advance_pop();
        }

        self.expect(SchemaSyntax::CloseBrace)?;
        self.builder.close()?;

        Ok(())
    }

    fn annotated_decl(&mut self) -> Result<(), syntree::Error> {
        while self.at(SchemaSyntax::At) {
            self.advance_push();
            self.annotation()?;
            self.skip_trivia()?;
            self.advance_pop();
        }

        if self.at_decl_start() {
            self.decl()?;
        }

        Ok(())
    }

    fn annotation(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::Annotation)?;
        self.expect(SchemaSyntax::At)?;

        self.skip_trivia()?;
        if self.at(SchemaSyntax::Identifier) || self.current().is_keyword() {
            self.bump()?;
        }

        self.skip_trivia()?;
        if self.at(SchemaSyntax::OpenParenthesis) {
            self.bump()?;
            self.skip_trivia()?;

            if self.at(SchemaSyntax::String) {
                self.bump()?;
            }

            self.expect(SchemaSyntax::CloseParenthesis)?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn decl(&mut self) -> Result<(), syntree::Error> {
        match self.current() {
            SchemaSyntax::EntityKeyword => self.entity_decl(),
            SchemaSyntax::ActionKeyword => self.action_decl(),
            SchemaSyntax::TypeKeyword => self.type_decl(),
            _ => Ok(()),
        }
    }

    fn entity_decl(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::EntityDeclaration)?;

        self.expect(SchemaSyntax::EntityKeyword)?;
        self.skip_trivia()?;

        self.idents()?;
        self.skip_trivia()?;

        if self.at(SchemaSyntax::EnumKeyword) {
            self.bump()?;
            self.skip_trivia()?;

            self.builder.open(SchemaSyntax::EnumType)?;
            self.expect(SchemaSyntax::OpenBracket)?;

            self.string_list()?;
            self.expect(SchemaSyntax::CloseBracket)?;

            self.builder.close()?;
        } else {
            if self.at(SchemaSyntax::InKeyword) {
                self.entity_parents()?;
                self.skip_trivia()?;
            }

            if self.at(SchemaSyntax::Equal) {
                self.bump()?;
                self.skip_trivia()?;
            }

            if self.at(SchemaSyntax::OpenBrace) {
                self.builder.open(SchemaSyntax::EntityAttributes)?;
                self.record_type()?;
                self.builder.close()?;
                self.skip_trivia()?;
            }

            if self.at(SchemaSyntax::TagsKeyword) {
                self.entity_tags()?;
                self.skip_trivia()?;
            }
        }

        self.expect(SchemaSyntax::Semicolon)?;
        self.builder.close()?;

        Ok(())
    }

    fn entity_parents(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::EntityParents)?;
        self.expect(SchemaSyntax::InKeyword)?;

        self.skip_trivia()?;
        self.type_list()?;

        self.builder.close()?;
        Ok(())
    }

    fn entity_tags(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::EntityTags)?;
        self.expect(SchemaSyntax::TagsKeyword)?;

        self.skip_trivia()?;
        self.type_expr()?;

        self.builder.close()?;
        Ok(())
    }

    fn action_decl(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::ActionDeclaration)?;

        self.expect(SchemaSyntax::ActionKeyword)?;
        self.skip_trivia()?;

        self.names()?;
        self.skip_trivia()?;

        if self.at(SchemaSyntax::InKeyword) {
            self.action_parents()?;
            self.skip_trivia()?;
        }

        if self.at(SchemaSyntax::AppliesToKeyword) {
            self.applies_to()?;
            self.skip_trivia()?;
        }

        if self.at(SchemaSyntax::AttributesKeyword) {
            self.action_attributes()?;
            self.skip_trivia()?;
        }

        self.expect(SchemaSyntax::Semicolon)?;
        self.builder.close()?;

        Ok(())
    }

    fn action_parents(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::ActionParents)?;
        self.expect(SchemaSyntax::InKeyword)?;

        self.skip_trivia()?;

        if self.at(SchemaSyntax::OpenBracket) {
            self.bump()?;
            self.qual_names()?;
            self.expect(SchemaSyntax::CloseBracket)?;
        } else {
            self.path()?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn applies_to(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::AppliesTo)?;
        self.expect(SchemaSyntax::AppliesToKeyword)?;
        self.expect(SchemaSyntax::OpenBrace)?;

        self.app_decls()?;
        self.expect(SchemaSyntax::CloseBrace)?;

        self.builder.close()?;
        Ok(())
    }

    fn app_decls(&mut self) -> Result<(), syntree::Error> {
        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::CloseBrace) || self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            match self.current() {
                SchemaSyntax::PrincipalKeyword => {
                    self.principal_types()?;
                }
                SchemaSyntax::ResourceKeyword => {
                    self.resource_types()?;
                }
                SchemaSyntax::ContextKeyword => {
                    self.context_type()?;
                }
                _ => {
                    self.builder.open(SchemaSyntax::Error)?;
                    self.bump()?;
                    self.builder.close()?;
                }
            }

            self.skip_trivia()?;
            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
            }

            self.advance_pop();
        }

        Ok(())
    }

    fn principal_types(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::PrincipalTypes)?;
        self.expect(SchemaSyntax::PrincipalKeyword)?;
        self.expect(SchemaSyntax::Colon)?;

        self.skip_trivia()?;
        self.type_list()?;

        self.builder.close()?;
        Ok(())
    }

    fn resource_types(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::ResourceTypes)?;
        self.expect(SchemaSyntax::ResourceKeyword)?;
        self.expect(SchemaSyntax::Colon)?;

        self.skip_trivia()?;
        self.type_list()?;

        self.builder.close()?;
        Ok(())
    }

    fn context_type(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::ContextType)?;
        self.expect(SchemaSyntax::ContextKeyword)?;
        self.expect(SchemaSyntax::Colon)?;

        self.skip_trivia()?;
        self.type_expr()?;

        self.builder.close()?;
        Ok(())
    }

    fn action_attributes(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::ActionAttributes)?;
        self.expect(SchemaSyntax::AttributesKeyword)?;
        self.expect(SchemaSyntax::OpenBrace)?;

        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::CloseBrace) || self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            if self.at(SchemaSyntax::Identifier) {
                self.attribute_entry()?;
            } else {
                self.builder.open(SchemaSyntax::Error)?;
                self.bump()?;
                self.builder.close()?;
            }

            self.skip_trivia()?;
            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
            }

            self.advance_pop();
        }

        self.expect(SchemaSyntax::CloseBrace)?;
        self.builder.close()?;

        Ok(())
    }

    fn attribute_entry(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::AttributeEntry)?;

        if self.at(SchemaSyntax::Identifier) {
            self.bump()?;
        }

        self.expect(SchemaSyntax::Colon)?;
        self.skip_trivia()?;

        if self.current().is_literal() {
            self.bump()?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn type_decl(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::CommonTypeDeclaration)?;
        self.expect(SchemaSyntax::TypeKeyword)?;

        self.skip_trivia()?;
        if self.at(SchemaSyntax::Identifier) {
            self.bump()?;
        }

        self.expect(SchemaSyntax::Equal)?;
        self.skip_trivia()?;

        self.type_expr()?;
        self.expect(SchemaSyntax::Semicolon)?;

        self.builder.close()?;
        Ok(())
    }

    fn type_expr(&mut self) -> Result<(), syntree::Error> {
        match self.current() {
            SchemaSyntax::SetKeyword => {
                self.set_type()?;
            }
            SchemaSyntax::OpenBrace => {
                self.record_type()?;
            }
            SchemaSyntax::Identifier
            | SchemaSyntax::BoolKeyword
            | SchemaSyntax::LongKeyword
            | SchemaSyntax::StringKeyword => {
                self.path()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn set_type(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::SetType)?;
        self.expect(SchemaSyntax::SetKeyword)?;
        self.expect(SchemaSyntax::LessThan)?;

        self.skip_trivia()?;
        self.type_expr()?;

        self.expect(SchemaSyntax::GreaterThan)?;
        self.builder.close()?;

        Ok(())
    }

    fn record_type(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::RecordType)?;
        self.expect(SchemaSyntax::OpenBrace)?;

        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::CloseBrace) || self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            while self.at(SchemaSyntax::At) {
                self.advance_push();
                self.annotation()?;
                self.skip_trivia()?;
                self.advance_pop();
            }

            if self.at(SchemaSyntax::Identifier)
                || self.at(SchemaSyntax::String)
                || self.current().is_keyword()
            {
                self.attr_decl()?;
            } else {
                self.builder.open(SchemaSyntax::Error)?;
                self.bump()?;
                self.builder.close()?;
            }

            self.skip_trivia()?;
            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
            }

            self.advance_pop();
        }

        self.expect(SchemaSyntax::CloseBrace)?;
        self.builder.close()?;

        Ok(())
    }

    fn attr_decl(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::AttributeDeclaration)?;

        if self.at(SchemaSyntax::Identifier)
            || self.at(SchemaSyntax::String)
            || self.current().is_keyword()
        {
            self.bump()?;
        }

        self.skip_trivia()?;
        if self.at(SchemaSyntax::Question) {
            self.bump()?;
        }

        self.expect(SchemaSyntax::Colon)?;
        self.skip_trivia()?;

        self.type_expr()?;
        self.builder.close()?;

        Ok(())
    }

    fn path(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::Name)?;

        if self.at_any(&[
            SchemaSyntax::Identifier,
            SchemaSyntax::BoolKeyword,
            SchemaSyntax::LongKeyword,
            SchemaSyntax::StringKeyword,
        ]) {
            self.bump()?;
        }

        loop {
            self.advance_push();
            self.skip_trivia()?;
            if self.at(SchemaSyntax::Colon2) {
                self.bump()?;
                self.skip_trivia()?;
                if self.at_any(&[
                    SchemaSyntax::Identifier,
                    SchemaSyntax::BoolKeyword,
                    SchemaSyntax::LongKeyword,
                    SchemaSyntax::StringKeyword,
                ]) {
                    self.bump()?;
                } else if self.at(SchemaSyntax::String) {
                    self.bump()?;
                    self.advance_pop();
                    break;
                }
                self.advance_pop();
            } else {
                self.advance_drop();
                break;
            }
        }

        self.builder.close()?;
        Ok(())
    }

    fn type_list(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(SchemaSyntax::TypeList)?;

        if self.at(SchemaSyntax::OpenBracket) {
            self.bump()?;

            loop {
                self.advance_push();
                self.skip_trivia()?;

                if self.at(SchemaSyntax::CloseBracket) || self.at(SchemaSyntax::Eof) {
                    self.advance_drop();
                    break;
                }

                self.path()?;
                self.skip_trivia()?;

                if self.at(SchemaSyntax::Comma) {
                    self.bump()?;
                }

                self.advance_pop();
            }

            self.expect(SchemaSyntax::CloseBracket)?;
        } else {
            self.path()?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn idents(&mut self) -> Result<(), syntree::Error> {
        if self.at(SchemaSyntax::Identifier) {
            self.bump()?;
        }

        loop {
            self.advance_push();
            self.skip_trivia()?;
            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
                self.skip_trivia()?;
                if self.at(SchemaSyntax::Identifier) {
                    self.bump()?;
                }
                self.advance_pop();
            } else {
                self.advance_drop();
                break;
            }
        }

        Ok(())
    }

    fn names(&mut self) -> Result<(), syntree::Error> {
        if self.at(SchemaSyntax::Identifier) || self.at(SchemaSyntax::String) {
            self.bump()?;
        }

        loop {
            self.advance_push();
            self.skip_trivia()?;
            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
                self.skip_trivia()?;
                if self.at(SchemaSyntax::Identifier) || self.at(SchemaSyntax::String) {
                    self.bump()?;
                }
                self.advance_pop();
            } else {
                self.advance_drop();
                break;
            }
        }

        Ok(())
    }

    fn qual_names(&mut self) -> Result<(), syntree::Error> {
        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::CloseBracket) || self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            if self.at(SchemaSyntax::String) {
                self.builder.open(SchemaSyntax::Name)?;
                self.bump()?;
                self.builder.close()?;
            } else {
                self.path()?;
            }
            self.skip_trivia()?;

            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
            }

            self.advance_pop();
        }

        Ok(())
    }

    fn string_list(&mut self) -> Result<(), syntree::Error> {
        loop {
            self.advance_push();
            self.skip_trivia()?;

            if self.at(SchemaSyntax::CloseBracket) || self.at(SchemaSyntax::Eof) {
                self.advance_drop();
                break;
            }

            if self.at(SchemaSyntax::String) {
                self.bump()?;
            } else {
                self.builder.open(SchemaSyntax::Error)?;
                self.bump()?;
                self.builder.close()?;
            }

            self.skip_trivia()?;
            if self.at(SchemaSyntax::Comma) {
                self.bump()?;
            }

            self.advance_pop();
        }

        Ok(())
    }

    fn at_decl_start(&self) -> bool {
        self.at_any(&[
            SchemaSyntax::EntityKeyword,
            SchemaSyntax::ActionKeyword,
            SchemaSyntax::TypeKeyword,
        ])
    }

    #[cfg(debug_assertions)]
    fn advance_push(&mut self) {
        self.advances.push(self.current.position());
    }

    #[cfg(not(debug_assertions))]
    fn advance_push(&mut self) {}

    #[expect(clippy::panic, reason = "Debug only")]
    #[cfg(debug_assertions)]
    fn advance_pop(&mut self) {
        let Some(start) = self.advances.pop() else {
            panic!("`advance_pop` called without prior `advance_push`");
        };

        assert!(
            self.current.position() > start,
            "schema parser did not advance: stuck at position {start} (token {:?})",
            self.current.syntax()
        );
    }

    #[cfg(not(debug_assertions))]
    fn advance_pop(&mut self) {}

    #[expect(clippy::panic, reason = "Debug only")]
    #[cfg(debug_assertions)]
    fn advance_drop(&mut self) {
        let Some(_) = self.advances.pop() else {
            panic!("`advance_drop` called without prior `advance_push`");
        };
    }

    #[cfg(not(debug_assertions))]
    fn advance_drop(&mut self) {}
}
