#[cfg(debug_assertions)]
use alloc::vec::Vec;
use core::ops::Range;

use duramen_diagnostic::Diagnostics;
use duramen_lexer::{Lexer, Token, TokenKind};
use duramen_syntax::{Builder, Syntax};

use crate::error::ParseError;

/// Maximum nesting depth.
const DEPTH_LIMIT: usize = 16;

pub const SCOPE_RECOVERY: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::CloseParenthesis,
    TokenKind::WhenKeyword,
    TokenKind::UnlessKeyword,
    TokenKind::Semicolon,
];

#[rustfmt::skip]
pub const CONDITION_RECOVERY: &[TokenKind] = &[
    TokenKind::CloseBrace
];

pub const EXPRESSION_RECOVERY: &[TokenKind] = &[
    TokenKind::CloseParenthesis,
    TokenKind::CloseBracket,
    TokenKind::CloseBrace,
    TokenKind::Comma,
    TokenKind::Semicolon,
];

#[rustfmt::skip]
pub const RECORD_RECOVERY: &[TokenKind] = &[
    TokenKind::Comma,
    TokenKind::CloseBrace
];

pub const ANNOTATION_RECOVERY: &[TokenKind] = &[
    TokenKind::CloseParenthesis,
    TokenKind::At,
    TokenKind::PermitKeyword,
    TokenKind::ForbidKeyword,
];

/// Shared parser infrastructure for policy and schema parsers.
pub struct Parser<'src, 'diag> {
    pub source: &'src str,
    pub lexer: Lexer<'src>,
    pub builder: Builder,
    pub diagnostics: &'diag mut Diagnostics,

    pub position: usize,
    pub current: Token,

    depth: usize,
    #[cfg(debug_assertions)]
    advances: Vec<usize>,
}

impl<'src, 'diag> Parser<'src, 'diag> {
    /// Creates a new parser.
    #[must_use]
    pub const fn new(source: &'src str, diagnostics: &'diag mut Diagnostics) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            builder: Builder::new(),
            diagnostics,

            position: 0,
            current: Token {
                kind: TokenKind::Eof,
                len: 0,
            },

            depth: 0,
            #[cfg(debug_assertions)]
            advances: Vec::new(),
        }
    }

    /// Returns the current token's kind.
    pub const fn kind(&self) -> TokenKind {
        self.current.kind
    }

    /// Returns the current token's span.
    pub const fn span(&self) -> Range<usize> {
        self.position..self.position + self.current.len
    }

    /// Checks if the current token matches any of the given kinds.
    pub fn at(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.current.kind)
    }

    /// Consumes the current token if it matches the given kind.
    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.current.kind == kind {
            self.next();
            true
        } else {
            false
        }
    }

    /// Consumes the current token if it matches the given kind, emitting a
    /// diagnostic if it does not.
    pub fn expect(&mut self, kind: TokenKind, expected: &'static str) {
        if !self.eat(kind) {
            self.diagnostics.push(ParseError::ExpectedToken {
                span: self.span(),
                expected,
            });
        }
    }

    /// Consumes the current token and moves to the next non-trivial token.
    pub fn next(&mut self) {
        if self.current.kind != TokenKind::Eof {
            self.builder
                .token(Syntax::from(self.current.kind), self.current.len);

            self.position += self.current.len;
        }

        loop {
            let token = self.lexer.next().unwrap_or(Token {
                kind: TokenKind::Eof,
                len: 0,
            });

            if token.kind.is_trivial() {
                self.builder.token(Syntax::from(token.kind), token.len);
                self.position += token.len;
            } else {
                self.current = token;
                break;
            }
        }
    }

    /// Consumes tokens until a `recovery` is found, wrapping them in an Error node.
    ///
    /// Returns `true` if any tokens were consumed.
    pub fn recover(&mut self, recovery: &[TokenKind]) -> bool {
        if self.at(recovery) || self.kind() == TokenKind::Eof {
            return false;
        }

        let err = self.builder.open(Syntax::Error);
        let mut unexpected_start = None;
        let mut unexpected_end = self.position;
        let mut depth = 0_usize;

        while self.kind() != TokenKind::Eof {
            if depth == 0 && self.at(recovery) {
                break;
            }

            match self.kind() {
                TokenKind::OpenParenthesis | TokenKind::OpenBracket | TokenKind::OpenBrace => {
                    depth += 1;
                }
                TokenKind::CloseParenthesis | TokenKind::CloseBracket | TokenKind::CloseBrace => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }

            if !self.diagnose() {
                unexpected_start.get_or_insert(self.position);
                unexpected_end = self.position + self.current.len;
            }

            self.advance_push();
            self.next();
            self.advance_pop();
        }

        if let Some(start) = unexpected_start {
            self.diagnostics.push(ParseError::UnexpectedToken {
                span: start..unexpected_end,
            });
        }

        self.builder.close(&err);
        true
    }

    /// Wraps the current token in an Error node and emits a diagnostic.
    pub fn error(&mut self) {
        let err = self.builder.open(Syntax::Error);
        if !self.diagnose() {
            self.diagnostics
                .push(ParseError::UnexpectedToken { span: self.span() });
        }

        self.advance_push();
        self.next();
        self.advance_pop();

        self.builder.close(&err);
    }

    /// Emits a diagnostic if the current token is a known error token.
    ///
    /// Returns `true` if a diagnostic was emitted.
    fn diagnose(&mut self) -> bool {
        match self.kind() {
            TokenKind::StringUnterminated => {
                self.diagnostics
                    .push(ParseError::UnterminatedString { span: self.span() });

                true
            }
            TokenKind::StringSingleQuoted => {
                self.diagnostics
                    .push(ParseError::StringSingleQuoted { span: self.span() });

                true
            }
            TokenKind::CommentBlock => {
                let start = self.position;
                let end = start + self.current.len;

                let open = start..start + 2;
                let close = self.source[start..end]
                    .ends_with("*/")
                    .then(|| end - 2..end);

                self.diagnostics
                    .push(ParseError::BlockComment { open, close });

                true
            }
            _ => false,
        }
    }

    /// Parses an annotation.
    ///
    /// ```cedar
    /// @id("policy name")
    /// ```
    pub fn annotation(&mut self) {
        let branch = self.builder.open(Syntax::Annotation);

        self.next();
        if self.kind().is_identifier() {
            let name_start = self.position;
            let mut ident_end = self.position + self.current.len;
            self.next();

            // Greedily consume adjacent non-whitespace tokens that look like
            // part of an intended name (e.g. `@view-permission`). Stop at `(`,
            // recovery tokens, or when there is a trivia gap.
            if self.position == ident_end
                && !self.at(ANNOTATION_RECOVERY)
                && self.kind() != TokenKind::OpenParenthesis
                && self.kind() != TokenKind::Eof
            {
                let err = self.builder.open(Syntax::Error);

                while self.position == ident_end
                    && !self.at(ANNOTATION_RECOVERY)
                    && self.kind() != TokenKind::OpenParenthesis
                    && self.kind() != TokenKind::Eof
                {
                    let next_end = self.position + self.current.len;
                    self.advance_push();
                    self.next();
                    self.advance_pop();

                    // If the next token is adjacent too, keep going. Otherwise,
                    // `ident_end` stays stale and the while condition breaks.
                    ident_end = next_end;
                }

                self.diagnostics.push(ParseError::InvalidAnnotationName {
                    span: name_start..ident_end,
                });
                self.builder.close(&err);
            }
        }

        if self.eat(TokenKind::OpenParenthesis) {
            if !self.eat(TokenKind::String)
                && self.kind() != TokenKind::CloseParenthesis
                && self.kind() != TokenKind::Eof
            {
                let err = self.builder.open(Syntax::Error);
                self.diagnostics.push(ParseError::ExpectedToken {
                    span: self.span(),
                    expected: "a string literal",
                });
                self.advance_push();
                self.next();
                self.advance_pop();
                self.builder.close(&err);
            }

            self.recover(ANNOTATION_RECOVERY);
            self.eat(TokenKind::CloseParenthesis);
        }

        self.builder.close(&branch);
    }

    /// Enters a nested expression, returning `false` if too deep.
    #[must_use]
    pub fn depth_push(&mut self) -> bool {
        if self.depth >= DEPTH_LIMIT {
            self.diagnostics
                .push(ParseError::NestingTooDeep { span: self.span() });
            return false;
        }

        self.depth += 1;
        true
    }

    /// Exits a nested expression.
    pub const fn depth_pop(&mut self) {
        self.depth -= 1;
    }

    #[cfg(debug_assertions)]
    pub fn advance_push(&mut self) {
        self.advances.push(self.position);
    }

    #[cfg(not(debug_assertions))]
    pub fn advance_push(&mut self) {}

    #[cfg(debug_assertions)]
    #[expect(clippy::panic, reason = "Debug assertion")]
    pub fn advance_pop(&mut self) {
        let Some(start) = self.advances.pop() else {
            panic!("`advance_pop` called without prior `advance_push`");
        };

        assert!(
            self.position > start,
            "parser did not advance: stuck at position {start} (token {:?})",
            self.current.kind
        );
    }

    #[cfg(not(debug_assertions))]
    pub fn advance_pop(&mut self) {}
}
