#[cfg(debug_assertions)]
use alloc::vec::Vec;
use core::ops::Range;

use duramen_diagnostic::Diagnostics;
use duramen_lexer::{Lexer, Token, TokenKind};
use duramen_syntax::{Builder, Syntax};

use crate::error::ParseError;

/// Maximum nesting depth.
const DEPTH_LIMIT: usize = 16;

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
    /// Parses an annotation.
    ///
    /// ```cedar
    /// @id("policy name")
    /// ```
    /// Recovery tokens that signal the end of an annotation.
    const ANNOTATION_RECOVERY: &'static [TokenKind] = &[
        TokenKind::OpenParenthesis,
        TokenKind::At,
        TokenKind::PermitKeyword,
        TokenKind::ForbidKeyword,
        TokenKind::EntityKeyword,
        TokenKind::ActionKeyword,
        TokenKind::TypeKeyword,
    ];

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

    pub fn annotation(&mut self) {
        let branch = self.builder.open(Syntax::Annotation);

        self.next();
        if self.kind().is_identifier() {
            self.next();
        }

        // Recover from malformed annotation names (e.g. `@bad-annotation`
        // where `-` splits the identifier into multiple tokens).
        self.recover_to(Self::ANNOTATION_RECOVERY);

        if self.eat(TokenKind::OpenParenthesis) {
            self.eat(TokenKind::String);
            self.recover_to(&[
                TokenKind::CloseParenthesis,
                TokenKind::At,
                TokenKind::PermitKeyword,
                TokenKind::ForbidKeyword,
            ]);
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

    /// Recovers to a closing delimiter, wrapping unexpected content in an Error node.
    ///
    /// Returns `true` if the closing delimiter was found and consumed.
    pub fn recover_to_close(&mut self, close: TokenKind) -> bool {
        if self.eat(close) {
            return true;
        }

        if self.at(&[TokenKind::Eof]) {
            return false;
        }

        let open = match close {
            TokenKind::CloseParenthesis => TokenKind::OpenParenthesis,
            TokenKind::CloseBracket => TokenKind::OpenBracket,
            TokenKind::CloseBrace => TokenKind::OpenBrace,
            _ => return false,
        };

        let err = self.builder.open(Syntax::Error);
        let mut depth = 0_usize;

        while !self.at(&[TokenKind::Eof]) {
            if self.kind() == open {
                depth += 1;
            } else if self.kind() == close {
                if depth == 0 {
                    break;
                }

                depth -= 1;
            }

            self.advance_push();
            self.next();
            self.advance_pop();
        }

        self.builder.close(&err);
        self.eat(close)
    }

    /// Recovers to a token in the recovery set, wrapping consumed tokens in an Error node.
    ///
    /// Tracks nesting depth across all three delimiter pairs so that matched
    /// delimiters inside the error are consumed as a group. Stops at unmatched
    /// close delimiters as a safety net (prevents escaping the enclosing scope).
    ///
    /// Returns `true` if any tokens were consumed.
    pub fn recover_to(&mut self, recovery: &[TokenKind]) -> bool {
        if self.at(recovery) || self.at(&[TokenKind::Eof]) {
            return false;
        }

        let err = self.builder.open(Syntax::Error);
        let mut parens = 0_usize;
        let mut braces = 0_usize;
        let mut brackets = 0_usize;

        while !self.at(&[TokenKind::Eof]) {
            if parens == 0 && braces == 0 && brackets == 0 && self.at(recovery) {
                break;
            }

            match self.kind() {
                TokenKind::OpenParenthesis => parens += 1,
                TokenKind::OpenBrace => braces += 1,
                TokenKind::OpenBracket => brackets += 1,
                TokenKind::CloseParenthesis if parens > 0 => parens -= 1,
                TokenKind::CloseBrace if braces > 0 => braces -= 1,
                TokenKind::CloseBracket if brackets > 0 => brackets -= 1,
                // Unmatched close delimiter — stop to avoid escaping enclosing scope
                TokenKind::CloseParenthesis | TokenKind::CloseBrace | TokenKind::CloseBracket => {
                    break;
                }
                _ => {}
            }

            self.advance_push();
            self.next();
            self.advance_pop();
        }

        self.builder.close(&err);
        true
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
