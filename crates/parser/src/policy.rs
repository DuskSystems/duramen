use duramen_diagnostic::Diagnostics;
use duramen_lexer::TokenKind;
use duramen_syntax::{Group, Tree};

use crate::common::Parser;
use crate::error::ParseError;

/// Binding power and syntax kind for an infix operator.
struct InfixOperator {
    left: u8,
    right: u8,
    kind: Group,
}

/// Parses Cedar policy source text into a concrete syntax tree.
pub struct PolicyParser<'src> {
    parser: Parser<'src>,
}

impl<'src> PolicyParser<'src> {
    /// Parses the source text and returns the tree and diagnostics.
    #[must_use]
    pub fn parse(source: &'src str) -> (Tree<'src>, Diagnostics) {
        let mut this = Self {
            parser: Parser::new(source),
        };

        this.policies();

        let tree = this.parser.builder.build(this.parser.source);
        let diagnostics = this.parser.diagnostics;
        (tree, diagnostics)
    }

    /// Parses a sequence of policies.
    ///
    /// ```cedar
    /// permit(principal, action, resource);
    /// forbid(principal == User::"tim", action, resource);
    /// ```
    fn policies(&mut self) {
        let branch = self.parser.builder.open(Group::Policies);
        self.parser.next();

        while !self.parser.at(&[TokenKind::Eof]) {
            self.parser.advance_push();
            self.policy();
            self.parser.advance_pop();
        }

        self.parser.builder.close(&branch);
    }

    /// Parses a single policy.
    ///
    /// ```cedar
    /// @id("prototypes access policy")
    /// permit(principal, action == Action::"view", resource in Album::"device_prototypes")
    /// when { principal.department == "HardwareEngineering" && principal.jobLevel >= 5 };
    /// ```
    fn policy(&mut self) {
        let branch = self.parser.builder.open(Group::Policy);

        if !self.parser.at(&[
            TokenKind::Eof,
            TokenKind::At,
            TokenKind::PermitKeyword,
            TokenKind::ForbidKeyword,
        ]) {
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            while !self.parser.at(&[
                TokenKind::Eof,
                TokenKind::At,
                TokenKind::PermitKeyword,
                TokenKind::ForbidKeyword,
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

        while self.parser.at(&[TokenKind::At]) {
            self.parser.advance_push();
            self.parser.annotation();
            self.parser.advance_pop();
        }

        if self
            .parser
            .at(&[TokenKind::PermitKeyword, TokenKind::ForbidKeyword])
        {
            self.parser.next();
        }

        let checkpoint = self.parser.builder.checkpoint();
        if self.parser.eat(TokenKind::OpenParenthesis) {
            self.variable_declaration();
            if self.parser.eat(TokenKind::Comma) {
                self.variable_declaration();
            }

            if self.parser.eat(TokenKind::Comma) {
                self.variable_declaration();
            }

            self.parser.eat(TokenKind::Comma);
            self.parser.expect(TokenKind::CloseParenthesis);
            self.parser.builder.commit(&checkpoint, Group::Scope);
        }

        while self
            .parser
            .at(&[TokenKind::WhenKeyword, TokenKind::UnlessKeyword])
        {
            self.parser.advance_push();
            self.condition();
            self.parser.advance_pop();
        }

        self.parser.expect(TokenKind::Semicolon);
        self.parser.builder.close(&branch);
    }

    /// Parses a variable declaration.
    ///
    /// ```cedar
    /// principal in UserGroup::"jane_friends"
    /// ```
    fn variable_declaration(&mut self) {
        let branch = self.parser.builder.open(Group::VariableDefinition);

        if self.parser.at(&[TokenKind::QuestionMark]) {
            self.slot();
            self.parser.builder.close(&branch);

            return;
        }

        if self.parser.kind().is_identifier() {
            self.parser.next();
        }

        if self.parser.eat(TokenKind::Colon) {
            self.name();
        }

        if self.parser.eat(TokenKind::IsKeyword) {
            self.pratt_expression(7);
            if self.parser.eat(TokenKind::InKeyword) {
                self.pratt_expression(7);
            }
        }

        if self.parser.kind().is_comparison() || self.parser.at(&[TokenKind::Equals]) {
            self.parser.next();
            self.expression();
        }

        self.parser.builder.close(&branch);
    }

    /// Parses a template slot.
    ///
    /// ```cedar
    /// ?principal
    /// ```
    fn slot(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        self.parser.next();
        if self.parser.kind().is_identifier() {
            self.parser.next();
        }

        self.parser.builder.commit(&checkpoint, Group::Slot);
    }

    /// Parses a condition.
    ///
    /// ```cedar
    /// when { resource.owner == principal }
    /// ```
    fn condition(&mut self) {
        let branch = self.parser.builder.open(Group::Condition);

        self.parser.next();
        if self.parser.eat(TokenKind::OpenBrace) {
            if !self.parser.at(&[TokenKind::CloseBrace]) {
                self.expression();
            }

            self.parser.expect(TokenKind::CloseBrace);
        }

        self.parser.builder.close(&branch);
    }

    /// Parses an expression.
    ///
    /// ```cedar
    /// principal.department == "Engineering"
    /// ```
    fn expression(&mut self) {
        if !self.parser.depth_push() {
            return;
        }

        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.at(&[TokenKind::IfKeyword]) {
            self.parser.next();
            self.expression();
            if self.parser.expect(TokenKind::ThenKeyword) {
                self.expression();
            }

            if self.parser.expect(TokenKind::ElseKeyword) {
                self.expression();
            }

            self.parser.builder.commit(&checkpoint, Group::IfExpression);
        } else {
            self.pratt_expression(0);
        }

        self.parser.depth_pop();
    }

    /// Returns the binding power and syntax node kind for the current infix operator.
    const fn infix_operator(&self) -> Option<InfixOperator> {
        match self.parser.kind() {
            TokenKind::Pipe2 => Some(InfixOperator {
                left: 1,
                right: 2,
                kind: Group::OrExpression,
            }),
            TokenKind::Ampersand2 => Some(InfixOperator {
                left: 3,
                right: 4,
                kind: Group::AndExpression,
            }),
            TokenKind::LessThan
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEquals
            | TokenKind::Equals2
            | TokenKind::BangEquals
            | TokenKind::InKeyword
            | TokenKind::Equals => Some(InfixOperator {
                left: 5,
                right: 6,
                kind: Group::RelationExpression,
            }),
            TokenKind::HasKeyword => Some(InfixOperator {
                left: 5,
                right: 6,
                kind: Group::HasExpression,
            }),
            TokenKind::LikeKeyword => Some(InfixOperator {
                left: 5,
                right: 6,
                kind: Group::LikeExpression,
            }),
            TokenKind::IsKeyword => Some(InfixOperator {
                left: 5,
                right: 6,
                kind: Group::IsExpression,
            }),
            TokenKind::Plus | TokenKind::Minus => Some(InfixOperator {
                left: 7,
                right: 8,
                kind: Group::SumExpression,
            }),
            TokenKind::Asterisk => Some(InfixOperator {
                left: 9,
                right: 10,
                kind: Group::ProductExpression,
            }),
            _ => None,
        }
    }

    /// Parses an expression using Pratt parsing.
    fn pratt_expression(&mut self, min_bp: u8) {
        let checkpoint = self.parser.builder.checkpoint();

        self.unary_expression();

        while let Some(op) = self.infix_operator() {
            if op.left < min_bp {
                break;
            }

            self.parser.advance_push();

            if self.parser.kind() == TokenKind::IsKeyword {
                self.parser.next();
                self.name();
                if self.parser.eat(TokenKind::InKeyword) {
                    self.pratt_expression(op.right);
                }
            } else {
                self.parser.next();
                self.pratt_expression(op.right);
            }

            self.parser.advance_pop();

            self.parser.builder.commit(&checkpoint, op.kind);
        }
    }

    /// Parses a unary expression.
    ///
    /// ```cedar
    /// !resource.isPublic
    /// ```
    fn unary_expression(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        let mut unary = false;
        while self.parser.at(&[TokenKind::Bang, TokenKind::Minus]) {
            self.parser.advance_push();
            self.parser.next();
            unary = true;
            self.parser.advance_pop();
        }

        self.member_expression();
        if unary {
            self.parser
                .builder
                .commit(&checkpoint, Group::UnaryExpression);
        }
    }

    /// Parses member access.
    ///
    /// ```cedar
    /// context.now.datetime.offset(duration("-24h"))
    /// ```
    fn member_expression(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        self.primary();
        let mut access = false;
        while self.parser.at(&[
            TokenKind::Dot,
            TokenKind::OpenParenthesis,
            TokenKind::OpenBracket,
        ]) {
            self.parser.advance_push();
            match self.parser.kind() {
                TokenKind::Dot => {
                    let inner = self.parser.builder.checkpoint();

                    self.parser.next();
                    if self.parser.kind().is_identifier() {
                        self.parser.next();
                    }

                    if self.parser.at(&[TokenKind::OpenParenthesis]) {
                        self.parser.next();
                        if !self.parser.at(&[TokenKind::CloseParenthesis]) {
                            self.argument_list();
                        }

                        self.parser.expect(TokenKind::CloseParenthesis);
                        self.parser.builder.commit(&inner, Group::Call);
                    } else {
                        self.parser.builder.commit(&inner, Group::Field);
                    }
                }
                TokenKind::OpenParenthesis => {
                    let inner = self.parser.builder.checkpoint();

                    self.parser.next();
                    if !self.parser.at(&[TokenKind::CloseParenthesis]) {
                        self.argument_list();
                    }

                    self.parser.expect(TokenKind::CloseParenthesis);
                    self.parser.builder.commit(&inner, Group::Call);
                }
                _ => {
                    let inner = self.parser.builder.checkpoint();

                    self.parser.next();
                    self.expression();
                    self.parser.expect(TokenKind::CloseBracket);

                    self.parser.builder.commit(&inner, Group::Index);
                }
            }
            access = true;
            self.parser.advance_pop();
        }

        if access {
            self.parser
                .builder
                .commit(&checkpoint, Group::MemberExpression);
        }
    }

    /// Parses a primary expression.
    ///
    /// ```cedar
    /// User::"alice"
    /// ```
    fn primary(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();

        if self.parser.kind().is_literal() {
            self.parser.next();
            self.parser.builder.commit(&checkpoint, Group::Literal);

            return;
        }

        if self.parser.at(&[TokenKind::QuestionMark]) {
            self.slot();
            return;
        }

        if self.parser.at(&[TokenKind::OpenParenthesis]) {
            self.parser.next();
            self.expression();
            self.parser.expect(TokenKind::CloseParenthesis);
            self.parser
                .builder
                .commit(&checkpoint, Group::Parenthesized);

            return;
        }

        if self.parser.at(&[TokenKind::OpenBracket]) {
            self.parser.next();
            if !self.parser.at(&[TokenKind::CloseBracket]) {
                self.argument_list();
            }

            self.parser.expect(TokenKind::CloseBracket);
            self.parser.builder.commit(&checkpoint, Group::List);

            return;
        }

        if self.parser.at(&[TokenKind::OpenBrace]) {
            self.parser.next();
            self.record_entries();
            self.parser.expect(TokenKind::CloseBrace);
            self.parser.builder.commit(&checkpoint, Group::Record);

            return;
        }

        if self.parser.kind().is_identifier() {
            self.name();
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

    /// Parses a record entry.
    ///
    /// ```cedar
    /// "field": value
    /// ```
    fn record_entry(&mut self) {
        let branch = self.parser.builder.open(Group::RecordEntry);

        if self.parser.at(&[TokenKind::String]) || self.parser.kind().is_identifier() {
            self.parser.next();
            if self.parser.eat(TokenKind::Colon) {
                self.expression();
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

        self.parser.builder.close(&branch);
    }

    /// Parses comma-separated record entries inside braces.
    ///
    /// ```cedar
    /// "name": "alice", "age": 30
    /// ```
    fn record_entries(&mut self) {
        while !self.parser.at(&[TokenKind::Eof, TokenKind::CloseBrace]) {
            self.parser.advance_push();
            self.record_entry();
            let comma = self.parser.eat(TokenKind::Comma);
            self.parser.advance_pop();
            if !comma {
                break;
            }
        }
    }

    /// Parses a comma-separated argument list.
    ///
    /// ```cedar
    /// ip("192.168.0.1"), ip("10.0.0.0/8")
    /// ```
    fn argument_list(&mut self) {
        let branch = self.parser.builder.open(Group::Arguments);
        self.expression();

        while self.parser.at(&[TokenKind::Comma]) {
            self.parser.advance_push();
            self.parser.next();
            if self
                .parser
                .at(&[TokenKind::CloseParenthesis, TokenKind::CloseBracket])
            {
                self.parser.advance_pop();
                break;
            }

            self.expression();
            self.parser.advance_pop();
        }

        self.parser.builder.close(&branch);
    }

    /// Parses a name or entity reference.
    ///
    /// ```cedar
    /// User::"alice"
    /// ```
    fn name(&mut self) {
        let checkpoint = self.parser.builder.checkpoint();
        let branch = self.parser.builder.open(Group::Name);

        if self.parser.kind().is_identifier() {
            self.parser.next();
            while self.parser.at(&[TokenKind::Colon2]) {
                if matches!(
                    self.parser.lexer.peek_kind(),
                    Some(TokenKind::String | TokenKind::OpenBrace)
                ) {
                    self.parser.builder.close(&branch);
                    self.parser.advance_push();
                    self.parser.next();
                    if self.parser.at(&[TokenKind::String]) {
                        self.parser.next();
                        self.parser.advance_pop();
                        self.parser
                            .builder
                            .commit(&checkpoint, Group::EntityReference);

                        return;
                    }

                    if self.parser.at(&[TokenKind::OpenBrace]) {
                        self.parser.next();
                        self.record_entries();
                        self.parser.eat(TokenKind::CloseBrace);
                        self.parser.advance_pop();
                        self.parser
                            .builder
                            .commit(&checkpoint, Group::EntityReference);

                        return;
                    }

                    self.parser.advance_pop();
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
        } else if !self.parser.at(&[TokenKind::Eof]) {
            self.parser.builder.close(&branch);
            let start = self.parser.position;
            let err = self.parser.builder.open(Group::Error);
            self.parser.next();
            self.parser.builder.close(&err);
            self.parser.diagnostics.push(ParseError::Unexpected {
                span: start..self.parser.position,
            });

            return;
        }

        self.parser.builder.close(&branch);
    }
}
