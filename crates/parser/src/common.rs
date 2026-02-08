use core::ops::Range;

use duramen_diagnostic::Diagnostics;
use duramen_lexer::{Lexer, Token, TokenKind};
use duramen_syntax::{Builder, Syntax};

use crate::advance::Advance;
use crate::error::ParseError;

/// Shared parser infrastructure for policy and schema parsers.
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub builder: Builder,
    pub advance: Advance,
    pub position: usize,
    pub current: Token,
    pub diagnostics: &'a mut Diagnostics,
}

impl<'a> Parser<'a> {
    /// Creates a new parser.
    #[must_use]
    pub const fn new(source: &'a str, diagnostics: &'a mut Diagnostics) -> Self {
        Self {
            lexer: Lexer::new(source),
            builder: Builder::new(),
            advance: Advance::new(),
            position: 0,
            current: Token {
                kind: TokenKind::Eof,
                len: 0,
            },
            diagnostics,
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

    /// Consumes the current token if it matches the given kind, or pushes an error diagnostic.
    pub fn expect(&mut self, kind: TokenKind) {
        if !self.eat(kind) {
            self.diagnostics.push(ParseError::Expected {
                span: self.span(),
                expected: kind,
            });
        }
    }

    /// Consumes the current token and moves to the next non-trivial token.
    pub fn next(&mut self) {
        if self.current.kind != TokenKind::Eof {
            match self.current.kind {
                TokenKind::StringUnterminated => {
                    self.diagnostics
                        .push(ParseError::UnterminatedString { span: self.span() });
                }
                TokenKind::Unknown => {
                    self.diagnostics
                        .push(ParseError::UnknownCharacter { span: self.span() });
                }
                _ => {}
            }

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

    /// Parses an annotation.
    ///
    /// ```cedar
    /// @id("policy name")
    /// ```
    pub fn annotation(&mut self) {
        let branch = self.builder.open(Syntax::Annotation);

        self.next();
        if self.kind().is_identifier() {
            self.next();
        }

        if self.eat(TokenKind::OpenParenthesis) {
            self.eat(TokenKind::String);
            self.expect(TokenKind::CloseParenthesis);
        }

        self.builder.close(&branch);
    }
}
