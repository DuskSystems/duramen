use alloc::string::ToString as _;

use smallvec::SmallVec;
use syntree::{Builder, Checkpoint, Flavor, FlavorDefault, Tree};

use super::PolicySet;
use super::lexer::{PolicyLexer, PolicyToken};
use super::syntax::PolicySyntax;
use crate::diagnostics::Diagnostic;

type PolicyCheckpoint = Checkpoint<<FlavorDefault as Flavor>::Pointer>;

pub struct PolicyParser<'a> {
    source: &'a str,
    lexer: PolicyLexer<'a>,
    current: PolicyToken<'a>,
    builder: Builder<PolicySyntax>,
    diagnostics: SmallVec<[Diagnostic; 4]>,
}

impl<'a> PolicyParser<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        let mut lexer = PolicyLexer::new(source);
        let current = lexer.next_token();

        Self {
            source,
            lexer,
            current,
            builder: Builder::new(),
            diagnostics: SmallVec::new_const(),
        }
    }

    #[must_use]
    pub fn parse(mut self) -> PolicySet<'a> {
        if let Err(err) = self.policy_set() {
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

        PolicySet::new(self.source, tree, diagnostics)
    }

    const fn current(&self) -> PolicySyntax {
        self.current.syntax()
    }

    fn at(&self, kind: PolicySyntax) -> bool {
        self.current.syntax() == kind
    }

    fn at_any(&self, kinds: &[PolicySyntax]) -> bool {
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

    fn expect(&mut self, kind: PolicySyntax) -> Result<(), syntree::Error> {
        self.skip_trivia()?;
        if self.at(kind) { self.bump() } else { Ok(()) }
    }

    fn checkpoint(&mut self) -> Result<PolicyCheckpoint, syntree::Error> {
        self.builder.checkpoint()
    }

    fn wrap_at(
        &mut self,
        checkpoint: &PolicyCheckpoint,
        kind: PolicySyntax,
    ) -> Result<(), syntree::Error> {
        self.builder.close_at(checkpoint, kind)?;
        Ok(())
    }

    fn policy_set(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::PolicySet)?;

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::Eof) {
                break;
            }

            self.policy()?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn policy(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::Policy)?;

        while self.at(PolicySyntax::At) {
            self.annotation()?;
            self.skip_trivia()?;
        }

        if self.at_any(&[PolicySyntax::PermitKeyword, PolicySyntax::ForbidKeyword]) {
            self.bump()?;
        }

        self.expect(PolicySyntax::OpenParenthesis)?;
        self.scope()?;
        self.expect(PolicySyntax::CloseParenthesis)?;

        self.skip_trivia()?;

        loop {
            if self.at_condition_start() {
                self.condition()?;
            } else if self.at(PolicySyntax::Identifier) {
                self.extension_clause()?;
            } else {
                break;
            }

            self.skip_trivia()?;
        }

        self.expect(PolicySyntax::Semicolon)?;
        self.builder.close()?;

        Ok(())
    }

    fn annotation(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::Annotation)?;
        self.expect(PolicySyntax::At)?;

        self.skip_trivia()?;
        if self.at_ident_or_keyword() {
            self.bump()?;
        }

        self.skip_trivia()?;
        if self.at(PolicySyntax::OpenParenthesis) {
            self.bump()?;
            self.skip_trivia()?;
            if self.at(PolicySyntax::String) {
                self.bump()?;
            }
            self.expect(PolicySyntax::CloseParenthesis)?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn extension_clause(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::Condition)?;

        if self.at(PolicySyntax::Identifier) {
            self.bump()?;
        }

        self.expect(PolicySyntax::OpenBrace)?;

        self.skip_trivia()?;
        if !self.at(PolicySyntax::CloseBrace) {
            if self.at(PolicySyntax::String) {
                self.bump()?;
            } else {
                self.expression()?;
            }
        }

        self.expect(PolicySyntax::CloseBrace)?;
        self.builder.close()?;

        Ok(())
    }

    fn scope(&mut self) -> Result<(), syntree::Error> {
        self.skip_trivia()?;
        self.variable_definition()?;

        self.expect(PolicySyntax::Comma)?;

        self.skip_trivia()?;
        self.variable_definition()?;

        self.expect(PolicySyntax::Comma)?;

        self.skip_trivia()?;
        self.variable_definition()?;

        Ok(())
    }

    fn variable_definition(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::VariableDefinition)?;

        if self.at_any(&[
            PolicySyntax::PrincipalKeyword,
            PolicySyntax::ActionKeyword,
            PolicySyntax::ResourceKeyword,
        ]) {
            self.bump()?;
        }

        self.skip_trivia()?;

        match self.current() {
            PolicySyntax::Equal2 => {
                self.bump()?;
                self.skip_trivia()?;
                self.entity_or_slot()?;
            }
            PolicySyntax::InKeyword => {
                self.bump()?;
                self.skip_trivia()?;

                if self.at(PolicySyntax::OpenBracket) {
                    self.entity_list()?;
                } else {
                    self.entity_or_slot()?;
                }
            }
            PolicySyntax::IsKeyword | PolicySyntax::Colon => {
                self.bump()?;
                self.skip_trivia()?;

                self.path()?;
                self.skip_trivia()?;

                if self.at(PolicySyntax::InKeyword) {
                    self.bump()?;
                    self.skip_trivia()?;
                    self.entity_or_slot()?;
                }
            }
            _ => {}
        }

        self.builder.close()?;
        Ok(())
    }

    fn entity_or_slot(&mut self) -> Result<(), syntree::Error> {
        if self.at(PolicySyntax::Question) {
            self.slot()?;
        } else {
            self.entity_ref()?;
        }

        Ok(())
    }

    fn slot(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::SlotExpression)?;
        self.expect(PolicySyntax::Question)?;

        self.skip_trivia()?;
        if self.at(PolicySyntax::Identifier) {
            self.bump()?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn entity_list(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::ListExpression)?;
        self.expect(PolicySyntax::OpenBracket)?;

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::CloseBracket) || self.at(PolicySyntax::Eof) {
                break;
            }

            self.entity_ref()?;
            self.skip_trivia()?;

            if self.at(PolicySyntax::Comma) {
                self.bump()?;
            }
        }

        self.expect(PolicySyntax::CloseBracket)?;
        self.builder.close()?;

        Ok(())
    }

    fn entity_ref(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::EntityReference)?;
        self.path()?;

        self.skip_trivia()?;
        if self.at(PolicySyntax::Colon2) {
            self.bump()?;
            self.skip_trivia()?;
        }

        if self.at(PolicySyntax::String) {
            self.bump()?;
        } else if self.at(PolicySyntax::OpenBrace) {
            self.expr_record()?;
        }

        self.builder.close()?;
        Ok(())
    }

    fn path(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::Name)?;

        if self.at(PolicySyntax::Identifier) {
            self.bump()?;
        }

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::Colon2) {
                self.bump()?;
                self.skip_trivia()?;

                if self.at(PolicySyntax::Identifier) {
                    self.bump()?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.builder.close()?;
        Ok(())
    }

    fn condition(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::Condition)?;

        if self.at_any(&[PolicySyntax::WhenKeyword, PolicySyntax::UnlessKeyword]) {
            self.bump()?;
        }

        self.expect(PolicySyntax::OpenBrace)?;
        self.skip_trivia()?;

        if !self.at(PolicySyntax::CloseBrace) {
            self.expression()?;
        }

        self.expect(PolicySyntax::CloseBrace)?;
        self.builder.close()?;

        Ok(())
    }

    fn expression(&mut self) -> Result<(), syntree::Error> {
        self.expr_if()
    }

    fn expr_if(&mut self) -> Result<(), syntree::Error> {
        if self.at(PolicySyntax::IfKeyword) {
            self.builder.open(PolicySyntax::IfExpression)?;

            self.bump()?;

            self.skip_trivia()?;
            self.expression()?;

            self.expect(PolicySyntax::ThenKeyword)?;

            self.skip_trivia()?;
            self.expression()?;

            self.expect(PolicySyntax::ElseKeyword)?;

            self.skip_trivia()?;
            self.expression()?;

            self.builder.close()?;
        } else {
            self.expr_or()?;
        }

        Ok(())
    }

    fn expr_or(&mut self) -> Result<(), syntree::Error> {
        let checkpoint = self.checkpoint()?;
        self.expr_and()?;

        loop {
            self.skip_trivia()?;
            if self.at(PolicySyntax::Pipe2) {
                self.bump()?;
                self.skip_trivia()?;
                self.expr_and()?;
                self.wrap_at(&checkpoint, PolicySyntax::BinaryExpression)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn expr_and(&mut self) -> Result<(), syntree::Error> {
        let checkpoint = self.checkpoint()?;
        self.expr_relation()?;

        loop {
            self.skip_trivia()?;
            if self.at(PolicySyntax::Ampersand2) {
                self.bump()?;
                self.skip_trivia()?;
                self.expr_relation()?;
                self.wrap_at(&checkpoint, PolicySyntax::BinaryExpression)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn expr_relation(&mut self) -> Result<(), syntree::Error> {
        let checkpoint = self.checkpoint()?;

        self.expr_add()?;
        self.skip_trivia()?;

        match self.current() {
            PolicySyntax::Equal2
            | PolicySyntax::NotEqual
            | PolicySyntax::LessThan
            | PolicySyntax::LessEqual
            | PolicySyntax::GreaterThan
            | PolicySyntax::GreaterEqual
            | PolicySyntax::InKeyword => {
                self.bump()?;
                self.skip_trivia()?;
                self.expr_add()?;
                self.wrap_at(&checkpoint, PolicySyntax::BinaryExpression)?;
            }
            PolicySyntax::HasKeyword => {
                self.bump()?;
                self.skip_trivia()?;

                if self.at(PolicySyntax::Identifier) || self.at(PolicySyntax::String) {
                    self.bump()?;
                }

                self.wrap_at(&checkpoint, PolicySyntax::HasExpression)?;
            }
            PolicySyntax::LikeKeyword => {
                self.bump()?;
                self.skip_trivia()?;

                if self.at(PolicySyntax::String) {
                    self.bump()?;
                }

                self.wrap_at(&checkpoint, PolicySyntax::LikeExpression)?;
            }
            PolicySyntax::IsKeyword => {
                self.bump()?;
                self.skip_trivia()?;

                self.path()?;
                self.skip_trivia()?;

                if self.at(PolicySyntax::InKeyword) {
                    self.bump()?;
                    self.skip_trivia()?;
                    self.entity_ref()?;
                }

                self.wrap_at(&checkpoint, PolicySyntax::IsExpression)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn expr_add(&mut self) -> Result<(), syntree::Error> {
        let checkpoint = self.checkpoint()?;
        self.expr_mul()?;

        loop {
            self.skip_trivia()?;

            if self.at_any(&[PolicySyntax::Plus, PolicySyntax::Minus]) {
                self.bump()?;
                self.skip_trivia()?;
                self.expr_mul()?;
                self.wrap_at(&checkpoint, PolicySyntax::BinaryExpression)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn expr_mul(&mut self) -> Result<(), syntree::Error> {
        let checkpoint = self.checkpoint()?;
        self.expr_unary()?;

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::Asterisk) {
                self.bump()?;
                self.skip_trivia()?;
                self.expr_unary()?;
                self.wrap_at(&checkpoint, PolicySyntax::BinaryExpression)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn expr_unary(&mut self) -> Result<(), syntree::Error> {
        if self.at_any(&[PolicySyntax::Not, PolicySyntax::Minus]) {
            self.builder.open(PolicySyntax::UnaryExpression)?;

            self.bump()?;

            self.skip_trivia()?;
            self.expr_unary()?;

            self.builder.close()?;
        } else {
            self.expr_member()?;
        }

        Ok(())
    }

    fn expr_member(&mut self) -> Result<(), syntree::Error> {
        let checkpoint = self.checkpoint()?;
        self.expr_primary()?;

        loop {
            self.skip_trivia()?;

            match self.current() {
                PolicySyntax::Dot => {
                    self.bump()?;
                    self.skip_trivia()?;

                    if self.at(PolicySyntax::Identifier) {
                        self.bump()?;
                    }

                    self.skip_trivia()?;
                    if self.at(PolicySyntax::OpenParenthesis) {
                        self.argument_list()?;
                    }

                    self.wrap_at(&checkpoint, PolicySyntax::MemberExpression)?;
                }
                PolicySyntax::OpenBracket => {
                    self.bump()?;
                    self.skip_trivia()?;

                    if self.at(PolicySyntax::String) {
                        self.bump()?;
                    }

                    self.expect(PolicySyntax::CloseBracket)?;
                    self.wrap_at(&checkpoint, PolicySyntax::MemberExpression)?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn argument_list(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::ArgumentList)?;
        self.expect(PolicySyntax::OpenParenthesis)?;

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::CloseParenthesis) || self.at(PolicySyntax::Eof) {
                break;
            }

            self.expression()?;

            self.skip_trivia()?;
            if self.at(PolicySyntax::Comma) {
                self.bump()?;
            }
        }

        self.expect(PolicySyntax::CloseParenthesis)?;
        self.builder.close()?;

        Ok(())
    }

    fn expr_primary(&mut self) -> Result<(), syntree::Error> {
        match self.current() {
            syntax if syntax.is_literal() => {
                self.builder.open(PolicySyntax::LiteralExpression)?;
                self.bump()?;
                self.builder.close()?;
            }
            PolicySyntax::Identifier
            | PolicySyntax::PrincipalKeyword
            | PolicySyntax::ActionKeyword
            | PolicySyntax::ResourceKeyword
            | PolicySyntax::ContextKeyword => {
                self.expr_name_or_entity()?;
            }
            PolicySyntax::Question => {
                self.slot()?;
            }
            PolicySyntax::OpenParenthesis => {
                self.expr_paren()?;
            }
            PolicySyntax::OpenBracket => {
                self.expr_list()?;
            }
            PolicySyntax::OpenBrace => {
                self.expr_record()?;
            }
            _ => {
                if !self.at(PolicySyntax::Eof)
                    && !self.at(PolicySyntax::CloseBrace)
                    && !self.at(PolicySyntax::CloseParenthesis)
                    && !self.at(PolicySyntax::CloseBracket)
                {
                    self.builder.open(PolicySyntax::Error)?;
                    self.bump()?;
                    self.builder.close()?;
                }
            }
        }

        Ok(())
    }

    fn expr_name_or_entity(&mut self) -> Result<(), syntree::Error> {
        if self.at_any(&[
            PolicySyntax::PrincipalKeyword,
            PolicySyntax::ActionKeyword,
            PolicySyntax::ResourceKeyword,
            PolicySyntax::ContextKeyword,
        ]) {
            self.builder.open(PolicySyntax::PathExpression)?;
            self.bump()?;
            self.builder.close()?;
            return Ok(());
        }

        let checkpoint = self.checkpoint()?;

        self.path()?;
        self.skip_trivia()?;

        if self.at(PolicySyntax::Colon2) {
            self.bump()?;
            self.skip_trivia()?;
        }

        if self.at(PolicySyntax::String) {
            self.bump()?;
            self.wrap_at(&checkpoint, PolicySyntax::EntityReference)?;
        } else if self.at(PolicySyntax::OpenParenthesis) {
            self.argument_list()?;
            self.wrap_at(&checkpoint, PolicySyntax::MemberExpression)?;
        } else {
            self.wrap_at(&checkpoint, PolicySyntax::PathExpression)?;
        }

        Ok(())
    }

    fn expr_paren(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::ParenExpression)?;
        self.expect(PolicySyntax::OpenParenthesis)?;

        self.skip_trivia()?;
        self.expression()?;

        self.expect(PolicySyntax::CloseParenthesis)?;
        self.builder.close()?;

        Ok(())
    }

    fn expr_list(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::ListExpression)?;
        self.expect(PolicySyntax::OpenBracket)?;

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::CloseBracket) || self.at(PolicySyntax::Eof) {
                break;
            }

            self.expression()?;

            self.skip_trivia()?;
            if self.at(PolicySyntax::Comma) {
                self.bump()?;
            }
        }

        self.expect(PolicySyntax::CloseBracket)?;
        self.builder.close()?;

        Ok(())
    }

    fn expr_record(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::RecordExpression)?;
        self.expect(PolicySyntax::OpenBrace)?;

        loop {
            self.skip_trivia()?;

            if self.at(PolicySyntax::CloseBrace) || self.at(PolicySyntax::Eof) {
                break;
            }

            self.record_entry()?;

            self.skip_trivia()?;
            if self.at(PolicySyntax::Comma) {
                self.bump()?;
            }
        }

        self.expect(PolicySyntax::CloseBrace)?;
        self.builder.close()?;

        Ok(())
    }

    fn record_entry(&mut self) -> Result<(), syntree::Error> {
        self.builder.open(PolicySyntax::RecordEntry)?;

        if self.at(PolicySyntax::Identifier) || self.at(PolicySyntax::String) {
            self.bump()?;
        }

        self.expect(PolicySyntax::Colon)?;
        self.skip_trivia()?;
        self.expression()?;
        self.builder.close()?;

        Ok(())
    }

    fn at_ident_or_keyword(&self) -> bool {
        self.at(PolicySyntax::Identifier) || self.current().is_keyword()
    }

    fn at_condition_start(&self) -> bool {
        self.at_any(&[PolicySyntax::WhenKeyword, PolicySyntax::UnlessKeyword])
    }
}
