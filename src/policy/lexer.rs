use super::syntax::PolicyTokenKind;
use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyToken<'a> {
    kind: PolicyTokenKind,
    text: &'a str,
    position: usize,
}

impl<'a> PolicyToken<'a> {
    #[must_use]
    pub const fn kind(&self) -> PolicyTokenKind {
        self.kind
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
}

impl<'a> PolicyLexer<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }

    #[must_use]
    pub const fn position(&self) -> usize {
        self.cursor.position()
    }

    pub fn next_token(&mut self) -> PolicyToken<'a> {
        let start = self.cursor.position();
        let first = self.cursor.current();

        let kind = match first {
            Cursor::END => PolicyTokenKind::Eof,
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                PolicyTokenKind::Whitespace
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    PolicyTokenKind::String
                } else {
                    PolicyTokenKind::Unknown
                }
            }
            byte if Cursor::is_digit(byte) => {
                self.cursor.scan_integer();
                PolicyTokenKind::Integer
            }
            byte if Cursor::is_ident_start(byte) => {
                let text = self.cursor.scan_ident();
                PolicyTokenKind::from_keyword(text).unwrap_or(PolicyTokenKind::Identifier)
            }
            b'(' => {
                self.cursor.bump();
                PolicyTokenKind::OpenParenthesis
            }
            b')' => {
                self.cursor.bump();
                PolicyTokenKind::CloseParenthesis
            }
            b'{' => {
                self.cursor.bump();
                PolicyTokenKind::OpenBrace
            }
            b'}' => {
                self.cursor.bump();
                PolicyTokenKind::CloseBrace
            }
            b'[' => {
                self.cursor.bump();
                PolicyTokenKind::OpenBracket
            }
            b']' => {
                self.cursor.bump();
                PolicyTokenKind::CloseBracket
            }
            b',' => {
                self.cursor.bump();
                PolicyTokenKind::Comma
            }
            b';' => {
                self.cursor.bump();
                PolicyTokenKind::Semicolon
            }
            b':' => {
                self.cursor.bump();
                if self.cursor.current() == b':' {
                    self.cursor.bump();
                    PolicyTokenKind::Colon2
                } else {
                    PolicyTokenKind::Colon
                }
            }
            b'@' => {
                self.cursor.bump();
                PolicyTokenKind::At
            }
            b'.' => {
                self.cursor.bump();
                PolicyTokenKind::Dot
            }
            b'?' => {
                self.cursor.bump();
                PolicyTokenKind::Question
            }
            b'=' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicyTokenKind::Equal2
                } else {
                    PolicyTokenKind::Equal
                }
            }
            b'!' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicyTokenKind::NotEqual
                } else {
                    PolicyTokenKind::Not
                }
            }
            b'<' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicyTokenKind::LessEqual
                } else {
                    PolicyTokenKind::LessThan
                }
            }
            b'>' => {
                self.cursor.bump();
                if self.cursor.current() == b'=' {
                    self.cursor.bump();
                    PolicyTokenKind::GreaterEqual
                } else {
                    PolicyTokenKind::GreaterThan
                }
            }
            b'&' => {
                self.cursor.bump();
                if self.cursor.current() == b'&' {
                    self.cursor.bump();
                    PolicyTokenKind::Ampersand2
                } else {
                    PolicyTokenKind::Ampersand
                }
            }
            b'|' => {
                self.cursor.bump();
                if self.cursor.current() == b'|' {
                    self.cursor.bump();
                    PolicyTokenKind::Pipe2
                } else {
                    PolicyTokenKind::Pipe
                }
            }
            b'+' => {
                self.cursor.bump();
                PolicyTokenKind::Plus
            }
            b'-' => {
                self.cursor.bump();
                PolicyTokenKind::Minus
            }
            b'*' => {
                self.cursor.bump();
                PolicyTokenKind::Asterisk
            }
            b'/' => {
                self.cursor.bump();
                if self.cursor.current() == b'/' {
                    self.cursor.bump();
                    self.cursor.skip_line();
                    PolicyTokenKind::Comment
                } else {
                    PolicyTokenKind::Slash
                }
            }
            b'%' => {
                self.cursor.bump();
                PolicyTokenKind::Percent
            }
            _ => {
                self.cursor.bump_char();
                PolicyTokenKind::Unknown
            }
        };

        PolicyToken {
            kind,
            text: self.cursor.slice(start),
            position: start,
        }
    }
}
