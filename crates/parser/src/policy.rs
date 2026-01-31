use alloc::string::String;

use duramen_cst::CstNode as _;
use duramen_cst::policy::{Policies, PolicyBuilder, PolicySyntax, PolicyTree};
use duramen_lexer::{Lexer, Token, TokenKind};

use crate::advance::Advance;

#[derive(Debug)]
pub struct PolicyParseResult {
    tree: PolicyTree,
}

impl PolicyParseResult {
    #[must_use]
    pub const fn tree(&self) -> &PolicyTree {
        &self.tree
    }

    #[must_use]
    pub fn policies(&self) -> Option<Policies<'_>> {
        self.tree.children().find_map(Policies::cast)
    }

    #[must_use]
    pub fn print(&self, source: &str) -> String {
        let mut output = String::with_capacity(source.len());

        for node in self.tree.children() {
            output.push_str(node.text(source));
        }

        output
    }
}

pub struct PolicyParser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    position: u32,
    builder: PolicyBuilder,
    advance: Advance,
}

impl<'a> PolicyParser<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            current: Token::new(TokenKind::Unknown, 0),
            position: 0,
            builder: PolicyBuilder::new((source.len() / 4) as u32),
            advance: Advance::new(),
        }
    }

    #[must_use]
    pub fn parse(mut self) -> PolicyParseResult {
        self.bump();
        self.policies();

        let tree = self.builder.build();
        PolicyParseResult { tree }
    }

    /// Consumes the current token and advances to the next non trivial token.
    #[inline(always)]
    fn bump(&mut self) {
        if self.current.len > 0 {
            self.builder
                .token(PolicySyntax::from(self.current.kind), self.current.len);

            self.position += self.current.len;
        }

        self.current = loop {
            match self.lexer.next() {
                Some(token) if token.kind.is_trivial() => {
                    self.builder
                        .token(PolicySyntax::from(token.kind), token.len);
                    self.position += token.len;
                }
                Some(token) => break token,
                None => break Token::new(TokenKind::Unknown, 0),
            }
        };
    }

    /// Parses a sequence of policies.
    ///
    /// ```cedar
    /// permit(principal, action, resource);
    /// forbid(principal == User::"tim", action, resource);
    /// ```
    fn policies(&mut self) {
        self.builder.open(PolicySyntax::Policies);

        while self.current.len > 0 {
            self.advance.push(self.position);
            self.policy();
            self.advance.pop(self.position, self.current.kind);
        }

        self.builder.close();
    }

    /// Parses a single policy.
    ///
    /// ```cedar
    /// @id("prototypes access policy")
    /// permit(principal, action == Action::"view", resource in Album::"device_prototypes")
    /// when { principal.department == "HardwareEngineering" && principal.jobLevel >= 5 };
    /// ```
    fn policy(&mut self) {
        self.builder.open(PolicySyntax::Policy);

        while self.current.len > 0
            && self.current.kind != TokenKind::At
            && self.current.kind != TokenKind::Permit
            && self.current.kind != TokenKind::Forbid
        {
            self.builder.open(PolicySyntax::Error);
            self.bump();
            self.builder.close();
        }

        while self.current.kind == TokenKind::At {
            self.advance.push(self.position);
            self.annotation();
            self.advance.pop(self.position, self.current.kind);
        }

        if self.current.kind == TokenKind::Permit || self.current.kind == TokenKind::Forbid {
            self.bump();
        }

        if self.current.kind == TokenKind::OpenParen {
            self.bump();

            self.variable_def();

            if self.current.kind == TokenKind::Comma {
                self.bump();
                self.variable_def();
            }

            if self.current.kind == TokenKind::Comma {
                self.bump();
                self.variable_def();
            }

            if self.current.kind == TokenKind::Comma {
                self.bump();
            }

            if self.current.kind == TokenKind::CloseParen {
                self.bump();
            }
        }

        while self.current.kind == TokenKind::When || self.current.kind == TokenKind::Unless {
            self.advance.push(self.position);
            self.condition();
            self.advance.pop(self.position, self.current.kind);
        }

        if self.current.kind == TokenKind::Semicolon {
            self.bump();
        }

        self.builder.close();
    }

    /// Parses an annotation.
    ///
    /// ```cedar
    /// @id("policy name")
    /// ```
    fn annotation(&mut self) {
        self.builder.open(PolicySyntax::Annotation);

        self.bump();

        if self.current.kind.is_identifier() {
            self.bump();
        }

        if self.current.kind == TokenKind::OpenParen {
            self.bump();

            if self.current.kind == TokenKind::String {
                self.bump();
            }

            if self.current.kind == TokenKind::CloseParen {
                self.bump();
            }
        }

        self.builder.close();
    }

    /// Parses a variable definition.
    ///
    /// ```cedar
    /// principal in UserGroup::"jane_friends"
    /// ```
    fn variable_def(&mut self) {
        self.builder.open(PolicySyntax::VariableDef);

        if self.current.kind == TokenKind::Question {
            self.slot();
            self.builder.close();
            return;
        }

        self.variable();

        if self.current.kind == TokenKind::Colon {
            self.bump();
            self.name();
        }

        if self.current.kind == TokenKind::Is {
            self.bump();
            self.name();

            if self.current.kind == TokenKind::In {
                self.bump();
                self.expr();
            }
        }

        if self.current.kind == TokenKind::Eq2 || self.current.kind == TokenKind::In {
            self.bump();
            self.expr();
        }

        self.builder.close();
    }

    /// Parses a variable.
    ///
    /// ```cedar
    /// principal
    /// ```
    fn variable(&mut self) {
        if self.current.kind.is_variable() {
            self.bump();
        }
    }

    /// Parses a template slot.
    ///
    /// ```cedar
    /// ?principal
    /// ```
    fn slot(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.bump();

        if self.current.kind.is_identifier() {
            self.bump();
        }

        self.builder.wrap(checkpoint, PolicySyntax::Slot);
    }

    /// Parses a condition.
    ///
    /// ```cedar
    /// when { resource.owner == principal }
    /// ```
    fn condition(&mut self) {
        self.builder.open(PolicySyntax::Condition);

        self.bump();

        if self.current.kind == TokenKind::OpenBrace {
            self.bump();

            if self.current.kind != TokenKind::CloseBrace {
                self.expr();
            }

            if self.current.kind == TokenKind::CloseBrace {
                self.bump();
            }
        }

        self.builder.close();
    }

    /// Parses an expression.
    fn expr(&mut self) {
        let checkpoint = self.builder.checkpoint();

        if self.current.kind == TokenKind::If {
            self.if_expr();
            self.builder.wrap(checkpoint, PolicySyntax::IfExpression);
        } else {
            self.or_expr();
        }
    }

    /// Parses an if expression.
    ///
    /// ```cedar
    /// if x then y else z
    /// ```
    fn if_expr(&mut self) {
        self.bump();
        self.expr();

        if self.current.kind == TokenKind::Then {
            self.bump();
            self.expr();
        }

        if self.current.kind == TokenKind::Else {
            self.bump();
            self.expr();
        }
    }

    /// Parses an or expression.
    ///
    /// ```cedar
    /// principal in resource.readers || principal in resource.editors
    /// ```
    fn or_expr(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.and_expr();

        if self.current.kind == TokenKind::Pipe2 {
            while self.current.kind == TokenKind::Pipe2 {
                self.advance.push(self.position);
                self.bump();
                self.and_expr();
                self.advance.pop(self.position, self.current.kind);
            }

            self.builder.wrap(checkpoint, PolicySyntax::OrExpression);
        }
    }

    /// Parses an and expression.
    ///
    /// ```cedar
    /// principal.department == "HardwareEngineering" && principal.jobLevel >= 5
    /// ```
    fn and_expr(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.relation();

        if self.current.kind == TokenKind::Amp2 {
            while self.current.kind == TokenKind::Amp2 {
                self.advance.push(self.position);
                self.bump();
                self.relation();
                self.advance.pop(self.position, self.current.kind);
            }

            self.builder.wrap(checkpoint, PolicySyntax::AndExpression);
        }
    }

    /// Parses a relation.
    ///
    /// ```cedar
    /// resource.path like "/home/*"
    /// ```
    fn relation(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.add_expr();

        if self.current.kind.is_comparison() {
            self.bump();
            self.add_expr();

            self.builder.wrap(checkpoint, PolicySyntax::Relation);
        } else if self.current.kind == TokenKind::Has {
            self.bump();
            if self.current.kind == TokenKind::String || self.current.kind.is_identifier() {
                self.bump();
            }

            self.builder.wrap(checkpoint, PolicySyntax::HasExpression);
        } else if self.current.kind == TokenKind::Like {
            self.bump();
            if self.current.kind == TokenKind::String {
                self.bump();
            }

            self.builder.wrap(checkpoint, PolicySyntax::LikeExpression);
        } else if self.current.kind == TokenKind::Is {
            self.bump();
            self.name();

            if self.current.kind == TokenKind::In {
                self.bump();
                self.expr();
            }

            self.builder.wrap(checkpoint, PolicySyntax::IsExpression);
        }
    }

    /// Parses an addition expression.
    ///
    /// ```cedar
    /// a + b - c
    /// ```
    fn add_expr(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.mult_expr();

        if self.current.kind == TokenKind::Plus || self.current.kind == TokenKind::Minus {
            while self.current.kind == TokenKind::Plus || self.current.kind == TokenKind::Minus {
                self.advance.push(self.position);
                self.bump();
                self.mult_expr();
                self.advance.pop(self.position, self.current.kind);
            }

            self.builder.wrap(checkpoint, PolicySyntax::Sum);
        }
    }

    /// Parses a multiplication expression.
    ///
    /// ```cedar
    /// a * b
    /// ```
    fn mult_expr(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.unary_expr();

        if self.current.kind == TokenKind::Star
            || self.current.kind == TokenKind::Slash
            || self.current.kind == TokenKind::Percent
        {
            while self.current.kind == TokenKind::Star
                || self.current.kind == TokenKind::Slash
                || self.current.kind == TokenKind::Percent
            {
                self.advance.push(self.position);
                self.bump();
                self.unary_expr();
                self.advance.pop(self.position, self.current.kind);
            }

            self.builder.wrap(checkpoint, PolicySyntax::Product);
        }
    }

    /// Parses a unary expression.
    ///
    /// ```cedar
    /// !resource.isPublic
    /// ```
    fn unary_expr(&mut self) {
        let checkpoint = self.builder.checkpoint();
        let mut has_unary = false;

        while self.current.kind == TokenKind::Bang || self.current.kind == TokenKind::Minus {
            self.advance.push(self.position);
            self.bump();
            has_unary = true;
            self.advance.pop(self.position, self.current.kind);
        }

        self.member_expr();

        if has_unary {
            self.builder.wrap(checkpoint, PolicySyntax::Unary);
        }
    }

    /// Parses member access.
    ///
    /// ```cedar
    /// context.now.datetime.offset(duration("-24h"))
    /// ```
    fn member_expr(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.primary();

        let mut has_access = false;

        loop {
            if self.current.kind == TokenKind::Dot {
                self.advance.push(self.position);

                let access_checkpoint = self.builder.checkpoint();
                self.bump();

                if self.current.kind.is_identifier() {
                    self.bump();
                }

                if self.current.kind == TokenKind::OpenParen {
                    self.bump();

                    if self.current.kind != TokenKind::CloseParen {
                        self.argument_list();
                    }

                    if self.current.kind == TokenKind::CloseParen {
                        self.bump();
                    }

                    self.builder
                        .wrap(access_checkpoint, PolicySyntax::MethodCall);
                } else {
                    self.builder
                        .wrap(access_checkpoint, PolicySyntax::FieldAccess);
                }

                has_access = true;
                self.advance.pop(self.position, self.current.kind);
            } else if self.current.kind == TokenKind::OpenParen {
                self.advance.push(self.position);

                let call_checkpoint = self.builder.checkpoint();
                self.bump();

                if self.current.kind != TokenKind::CloseParen {
                    self.argument_list();
                }

                if self.current.kind == TokenKind::CloseParen {
                    self.bump();
                }

                self.builder.wrap(call_checkpoint, PolicySyntax::MethodCall);

                has_access = true;
                self.advance.pop(self.position, self.current.kind);
            } else if self.current.kind == TokenKind::OpenBracket {
                self.advance.push(self.position);

                let index_checkpoint = self.builder.checkpoint();

                self.bump();
                self.expr();

                if self.current.kind == TokenKind::CloseBracket {
                    self.bump();
                }

                self.builder
                    .wrap(index_checkpoint, PolicySyntax::IndexAccess);

                has_access = true;
                self.advance.pop(self.position, self.current.kind);
            } else {
                break;
            }
        }

        if has_access {
            self.builder.wrap(checkpoint, PolicySyntax::Member);
        }
    }

    /// Parses a primary expression.
    ///
    /// ```cedar
    /// User::"alice"
    /// ```
    fn primary(&mut self) {
        let checkpoint = self.builder.checkpoint();

        if self.current.kind.is_literal() {
            self.bump();
            self.builder.wrap(checkpoint, PolicySyntax::Literal);
        } else if self.current.kind == TokenKind::Question {
            self.slot();
        } else if self.current.kind == TokenKind::OpenParen {
            self.bump();
            self.expr();

            if self.current.kind == TokenKind::CloseParen {
                self.bump();
            }

            self.builder.wrap(checkpoint, PolicySyntax::Parenthesized);
        } else if self.current.kind == TokenKind::OpenBracket {
            self.bump();

            if self.current.kind != TokenKind::CloseBracket {
                self.argument_list();
            }

            if self.current.kind == TokenKind::CloseBracket {
                self.bump();
            }

            self.builder.wrap(checkpoint, PolicySyntax::List);
        } else if self.current.kind == TokenKind::OpenBrace {
            self.bump();

            while self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
                self.advance.push(self.position);
                self.record_entry();

                if self.current.kind == TokenKind::Comma {
                    self.bump();
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }

                self.advance.pop(self.position, self.current.kind);
            }

            if self.current.kind == TokenKind::CloseBrace {
                self.bump();
            }

            self.builder.wrap(checkpoint, PolicySyntax::Record);
        } else if self.current.kind.is_identifier() {
            self.name();
        } else if self.current.len > 0 {
            self.builder.open(PolicySyntax::Error);
            self.bump();
            self.builder.close();
        }
    }

    /// Parses a record entry.
    ///
    /// ```cedar
    /// "field": value
    /// ```
    fn record_entry(&mut self) {
        self.builder.open(PolicySyntax::RecordEntry);

        if self.current.kind == TokenKind::String || self.current.kind.is_identifier() {
            self.bump();

            if self.current.kind == TokenKind::Colon {
                self.bump();
                self.expr();
            }
        } else if self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
            self.builder.open(PolicySyntax::Error);
            self.bump();
            self.builder.close();
        }

        self.builder.close();
    }

    /// Parses a comma-separated argument list.
    ///
    /// ```cedar
    /// ip("192.168.0.1"), ip("10.0.0.0/8")
    /// ```
    fn argument_list(&mut self) {
        self.builder.open(PolicySyntax::ArgumentList);

        self.expr();

        while self.current.kind == TokenKind::Comma {
            self.advance.push(self.position);
            self.bump();

            if self.current.kind == TokenKind::CloseParen
                || self.current.kind == TokenKind::CloseBracket
            {
                self.advance.pop(self.position, self.current.kind);
                break;
            }

            self.expr();
            self.advance.pop(self.position, self.current.kind);
        }

        self.builder.close();
    }

    /// Parses a name or entity reference.
    ///
    /// ```cedar
    /// User::"alice"
    /// ```
    fn name(&mut self) {
        let checkpoint = self.builder.checkpoint();

        self.builder.open(PolicySyntax::Name);

        if self.current.kind.is_identifier() {
            self.bump();

            while self.current.kind == TokenKind::Colon2 {
                if self.peek_entity_ref() {
                    self.builder.close();

                    self.advance.push(self.position);
                    self.bump();

                    if self.current.kind == TokenKind::String {
                        self.bump();

                        self.advance.pop(self.position, self.current.kind);
                        self.builder.wrap(checkpoint, PolicySyntax::EntityReference);

                        return;
                    } else if self.current.kind == TokenKind::OpenBrace {
                        self.bump();

                        while self.current.len > 0 && self.current.kind != TokenKind::CloseBrace {
                            self.advance.push(self.position);
                            self.record_entry();

                            if self.current.kind == TokenKind::Comma {
                                self.bump();
                            } else {
                                self.advance.pop(self.position, self.current.kind);
                                break;
                            }

                            self.advance.pop(self.position, self.current.kind);
                        }

                        if self.current.kind == TokenKind::CloseBrace {
                            self.bump();
                        }

                        self.advance.pop(self.position, self.current.kind);
                        self.builder.wrap(checkpoint, PolicySyntax::EntityReference);

                        return;
                    }

                    self.advance.pop(self.position, self.current.kind);
                    break;
                }

                self.advance.push(self.position);
                self.bump();

                if self.current.kind.is_identifier() {
                    self.bump();
                    self.advance.pop(self.position, self.current.kind);
                } else {
                    self.advance.pop(self.position, self.current.kind);
                    break;
                }
            }
        }

        self.builder.close();
    }

    fn peek_entity_ref(&mut self) -> bool {
        let saved = self.lexer.offset();

        let result = loop {
            match self.lexer.next() {
                Some(token) if token.kind.is_trivial() => {}
                Some(token) => {
                    break matches!(token.kind, TokenKind::String | TokenKind::OpenBrace);
                }
                None => break false,
            }
        };

        self.lexer.set_offset(saved);
        result
    }
}
