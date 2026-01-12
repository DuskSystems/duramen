use smallvec::SmallVec;

use super::syntax::PolicySyntax;
use crate::cursor::Cursor;
use crate::diagnostics::Diagnostic;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyToken<'a> {
    syntax: PolicySyntax,
    text: &'a str,
    position: usize,
}

impl<'a> PolicyToken<'a> {
    #[must_use]
    pub const fn syntax(&self) -> PolicySyntax {
        self.syntax
    }

    #[must_use]
    pub const fn text(&self) -> &'a str {
        self.text
    }

    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }
}

pub struct PolicyLexer<'a> {
    cursor: Cursor<'a>,
    diagnostics: SmallVec<[Diagnostic; 4]>,
}

impl<'a> PolicyLexer<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
            diagnostics: SmallVec::new_const(),
        }
    }

    #[must_use]
    pub const fn position(&self) -> usize {
        self.cursor.position()
    }

    pub fn take_diagnostics(&mut self) -> SmallVec<[Diagnostic; 4]> {
        core::mem::take(&mut self.diagnostics)
    }

    pub fn next_token(&mut self) -> PolicyToken<'a> {
        let start = self.cursor.position();
        let first = self.cursor.current();

        let syntax = match first {
            Cursor::END => PolicySyntax::Eof,
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                PolicySyntax::Whitespace
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    PolicySyntax::String
                } else {
                    let end = self.cursor.position();
                    self.diagnostics.push(
                        Diagnostic::error("unterminated string")
                            .with_label(start..end, "string is not closed"),
                    );

                    PolicySyntax::Unknown
                }
            }
            byte if Cursor::is_digit(byte) => {
                self.cursor.scan_integer();
                PolicySyntax::Integer
            }
            byte if Cursor::is_ident_start(byte) => {
                let text = self.cursor.scan_ident();
                PolicySyntax::from_keyword(text).unwrap_or(PolicySyntax::Identifier)
            }
            b'(' => {
                self.cursor.bump();
                PolicySyntax::OpenParenthesis
            }
            b')' => {
                self.cursor.bump();
                PolicySyntax::CloseParenthesis
            }
            b'{' => {
                self.cursor.bump();
                PolicySyntax::OpenBrace
            }
            b'}' => {
                self.cursor.bump();
                PolicySyntax::CloseBrace
            }
            b'[' => {
                self.cursor.bump();
                PolicySyntax::OpenBracket
            }
            b']' => {
                self.cursor.bump();
                PolicySyntax::CloseBracket
            }
            b',' => {
                self.cursor.bump();
                PolicySyntax::Comma
            }
            b';' => {
                self.cursor.bump();
                PolicySyntax::Semicolon
            }
            b':' => {
                self.cursor.bump();
                if self.cursor.current() == b':' {
                    self.cursor.bump();
                    PolicySyntax::Colon2
                } else {
                    PolicySyntax::Colon
                }
            }
            b'@' => {
                self.cursor.bump();
                PolicySyntax::At
            }
            b'.' => {
                self.cursor.bump();
                PolicySyntax::Dot
            }
            b'?' => {
                self.cursor.bump();
                PolicySyntax::Question
            }
            b'=' => {
                self.cursor.bump();
                match self.cursor.current() {
                    b'=' => {
                        self.cursor.bump();
                        PolicySyntax::Equal2
                    }
                    b'>' => {
                        self.cursor.bump();
                        PolicySyntax::FatArrow
                    }
                    _ => PolicySyntax::Equal,
                }
            }
            b'!' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicySyntax::NotEqual
                } else {
                    PolicySyntax::Not
                }
            }
            b'<' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicySyntax::LessEqual
                } else {
                    PolicySyntax::LessThan
                }
            }
            b'>' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicySyntax::GreaterEqual
                } else {
                    PolicySyntax::GreaterThan
                }
            }
            b'&' => {
                self.cursor.bump();
                if self.cursor.current() == b'&' {
                    self.cursor.bump();
                    PolicySyntax::Ampersand2
                } else {
                    PolicySyntax::Ampersand
                }
            }
            b'|' => {
                self.cursor.bump();
                if self.cursor.current() == b'|' {
                    self.cursor.bump();
                    PolicySyntax::Pipe2
                } else {
                    PolicySyntax::Pipe
                }
            }
            b'+' => {
                self.cursor.bump();
                PolicySyntax::Plus
            }
            b'-' => {
                self.cursor.bump();
                PolicySyntax::Minus
            }
            b'*' => {
                self.cursor.bump();
                PolicySyntax::Asterisk
            }
            b'/' => {
                self.cursor.bump();
                if self.cursor.current() == b'/' {
                    self.cursor.bump();
                    self.cursor.skip_line();
                    PolicySyntax::Comment
                } else {
                    PolicySyntax::Slash
                }
            }
            b'%' => {
                self.cursor.bump();
                PolicySyntax::Percent
            }
            _ => {
                self.cursor.bump_char();
                let end = self.cursor.position();
                self.diagnostics.push(
                    Diagnostic::error("unexpected character")
                        .with_label(start..end, "not recognized"),
                );

                PolicySyntax::Unknown
            }
        };

        PolicyToken {
            syntax,
            text: self.cursor.slice(start),
            position: start,
        }
    }
}
