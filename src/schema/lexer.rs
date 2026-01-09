use super::syntax::SchemaTokenKind;
use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaToken<'a> {
    kind: SchemaTokenKind,
    text: &'a str,
    position: usize,
}

impl<'a> SchemaToken<'a> {
    #[must_use]
    pub const fn kind(&self) -> SchemaTokenKind {
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

pub struct SchemaLexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> SchemaLexer<'a> {
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

    pub fn next_token(&mut self) -> SchemaToken<'a> {
        let start = self.cursor.position();
        let first = self.cursor.current();

        let kind = match first {
            Cursor::END => SchemaTokenKind::Eof,
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                SchemaTokenKind::Whitespace
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    SchemaTokenKind::String
                } else {
                    SchemaTokenKind::Unknown
                }
            }
            byte if Cursor::is_ident_start(byte) => {
                let text = self.cursor.scan_ident();
                SchemaTokenKind::from_keyword(text).unwrap_or(SchemaTokenKind::Identifier)
            }
            b'(' => {
                self.cursor.bump();
                SchemaTokenKind::OpenParenthesis
            }
            b')' => {
                self.cursor.bump();
                SchemaTokenKind::CloseParenthesis
            }
            b'{' => {
                self.cursor.bump();
                SchemaTokenKind::OpenBrace
            }
            b'}' => {
                self.cursor.bump();
                SchemaTokenKind::CloseBrace
            }
            b'[' => {
                self.cursor.bump();
                SchemaTokenKind::OpenBracket
            }
            b']' => {
                self.cursor.bump();
                SchemaTokenKind::CloseBracket
            }
            b',' => {
                self.cursor.bump();
                SchemaTokenKind::Comma
            }
            b';' => {
                self.cursor.bump();
                SchemaTokenKind::Semicolon
            }
            b':' => {
                self.cursor.bump();
                if self.cursor.current() == b':' {
                    self.cursor.bump();
                    SchemaTokenKind::Colon2
                } else {
                    SchemaTokenKind::Colon
                }
            }
            b'@' => {
                self.cursor.bump();
                SchemaTokenKind::At
            }
            b'.' => {
                self.cursor.bump();
                SchemaTokenKind::Dot
            }
            b'?' => {
                self.cursor.bump();
                SchemaTokenKind::Question
            }
            b'<' => {
                self.cursor.bump();
                SchemaTokenKind::LessThan
            }
            b'>' => {
                self.cursor.bump();
                SchemaTokenKind::GreaterThan
            }
            b'=' => {
                self.cursor.bump();
                SchemaTokenKind::Equal
            }
            b'|' => {
                self.cursor.bump();
                SchemaTokenKind::Pipe
            }
            b'/' => {
                self.cursor.bump();
                if self.cursor.current() == b'/' {
                    self.cursor.bump();
                    self.cursor.skip_line();
                    SchemaTokenKind::Comment
                } else {
                    SchemaTokenKind::Slash
                }
            }
            _ => {
                self.cursor.bump_char();
                SchemaTokenKind::Unknown
            }
        };

        SchemaToken {
            kind,
            text: self.cursor.slice(start),
            position: start,
        }
    }
}
