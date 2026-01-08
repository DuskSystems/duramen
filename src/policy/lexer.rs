use super::syntax::PolicyKind;
use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyToken<'a> {
    kind: PolicyKind,
    text: &'a str,
    offset: usize,
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

    pub fn next_token(&mut self) -> PolicyToken<'a> {
        if self.cursor.is_eof() {
            return PolicyToken {
                kind: PolicyKind::EOF,
                text: "",
                offset: self.cursor.position(),
            };
        }

        let start = self.cursor.position();
        let Some(byte) = self.cursor.peek() else {
            return PolicyToken {
                kind: PolicyKind::EOF,
                text: "",
                offset: self.cursor.position(),
            };
        };

        let kind = match byte {
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                PolicyKind::WHITESPACE
            }
            b'/' if self.cursor.peek_next() == Some(b'/') => {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.skip_line();
                PolicyKind::COMMENT
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    PolicyKind::STRING
                } else {
                    PolicyKind::ERROR
                }
            }
            byte if Cursor::is_digit(byte) => {
                self.cursor.scan_integer();
                PolicyKind::INT
            }
            byte if Cursor::is_ident_start(byte) => {
                let text = self.cursor.scan_ident();
                PolicyKind::from_keyword(text).unwrap_or(PolicyKind::IDENT)
            }
            b'@' => {
                self.cursor.bump();
                PolicyKind::AT
            }
            b'(' => {
                self.cursor.bump();
                PolicyKind::L_PAREN
            }
            b')' => {
                self.cursor.bump();
                PolicyKind::R_PAREN
            }
            b'{' => {
                self.cursor.bump();
                PolicyKind::L_BRACE
            }
            b'}' => {
                self.cursor.bump();
                PolicyKind::R_BRACE
            }
            b'[' => {
                self.cursor.bump();
                PolicyKind::L_BRACKET
            }
            b']' => {
                self.cursor.bump();
                PolicyKind::R_BRACKET
            }
            b';' => {
                self.cursor.bump();
                PolicyKind::SEMI
            }
            b':' => {
                self.cursor.bump();
                if self.cursor.peek() == Some(b':') {
                    self.cursor.bump();
                    PolicyKind::COLON2
                } else {
                    PolicyKind::COLON
                }
            }
            b',' => {
                self.cursor.bump();
                PolicyKind::COMMA
            }
            b'.' => {
                self.cursor.bump();
                PolicyKind::DOT
            }
            b'?' => {
                self.cursor.bump();

                let position = self.cursor.position();
                let text = self.cursor.scan_ident();

                match text {
                    "principal" => PolicyKind::PRINCIPAL_SLOT,
                    "resource" => PolicyKind::RESOURCE_SLOT,
                    _ => {
                        self.cursor.seek(position);
                        PolicyKind::QUESTION
                    }
                }
            }
            b'=' => {
                self.cursor.bump();
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump();
                    PolicyKind::EQ2
                } else {
                    PolicyKind::ERROR
                }
            }
            b'!' => {
                self.cursor.bump();
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump();
                    PolicyKind::NEQ
                } else {
                    PolicyKind::BANG
                }
            }
            b'<' => {
                self.cursor.bump();
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump();
                    PolicyKind::LTEQ
                } else {
                    PolicyKind::LT
                }
            }
            b'>' => {
                self.cursor.bump();
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump();
                    PolicyKind::GTEQ
                } else {
                    PolicyKind::GT
                }
            }
            b'&' if self.cursor.peek_next() == Some(b'&') => {
                self.cursor.bump();
                self.cursor.bump();
                PolicyKind::AMP2
            }
            b'|' if self.cursor.peek_next() == Some(b'|') => {
                self.cursor.bump();
                self.cursor.bump();
                PolicyKind::PIPE2
            }
            b'+' => {
                self.cursor.bump();
                PolicyKind::PLUS
            }
            b'-' => {
                self.cursor.bump();
                PolicyKind::MINUS
            }
            b'*' => {
                self.cursor.bump();
                PolicyKind::STAR
            }
            b'/' => {
                self.cursor.bump();
                PolicyKind::SLASH
            }
            b'%' => {
                self.cursor.bump();
                PolicyKind::PERCENT
            }
            _ => {
                self.cursor.bump_char();
                PolicyKind::ERROR
            }
        };

        PolicyToken {
            kind,
            text: self.cursor.slice(start),
            offset: start,
        }
    }
}
