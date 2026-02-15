#[cfg(debug_assertions)]
use alloc::vec::Vec;
use core::ops::Range;

use duramen_diagnostic::Diagnostics;
use duramen_lexer::{Lexer, Token, TokenKind};
use duramen_syntax::{Builder, Group};

use crate::error::ParseError;

/// Maximum nesting depth.
const DEPTH_LIMIT: usize = 16;

/// Shared parser infrastructure for policy and schema parsers.
pub struct Parser<'src> {
    pub source: &'src str,
    pub lexer: Lexer<'src>,
    pub builder: Builder,
    pub diagnostics: Diagnostics,

    pub position: usize,
    pub current: Token,

    depth: usize,
    #[cfg(debug_assertions)]
    advances: Vec<usize>,
}

impl<'src> Parser<'src> {
    /// Creates a new parser.
    #[must_use]
    pub const fn new(source: &'src str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            builder: Builder::new(),
            diagnostics: Diagnostics::new(),

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

    /// Consumes the current token and moves to the next non-trivial token.
    pub fn next(&mut self) {
        if self.current.kind != TokenKind::Eof {
            self.builder.token(self.current.kind, self.current.len);
            self.position += self.current.len;
        }

        loop {
            let token = self.lexer.next().unwrap_or(Token {
                kind: TokenKind::Eof,
                len: 0,
            });

            if token.kind.is_trivial() {
                self.builder.token(token.kind, token.len);
                self.position += token.len;
            } else {
                self.current = token;
                break;
            }
        }
    }

    /// Parses an annotation.
    ///
    /// ```cedar
    /// @id("policy name")
    /// ```
    pub fn annotation(&mut self) {
        let branch = self.builder.open(Group::Annotation);

        self.next();
        if self.kind().is_identifier() {
            self.next();
        }

        if self.eat(TokenKind::OpenParenthesis) {
            self.eat(TokenKind::String);
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
