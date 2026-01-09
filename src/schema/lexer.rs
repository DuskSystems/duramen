use super::syntax::SchemaKind;
use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaToken<'a> {
    kind: SchemaKind,
    text: &'a str,
    offset: usize,
}

impl<'a> SchemaToken<'a> {
    #[must_use]
    pub const fn kind(&self) -> SchemaKind {
        self.kind
    }

    #[must_use]
    pub const fn text(&self) -> &'a str {
        self.text
    }

    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
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

    pub fn next_token(&mut self) -> SchemaToken<'a> {
        if self.cursor.is_eof() {
            return SchemaToken {
                kind: SchemaKind::EOF,
                text: "",
                offset: self.cursor.position(),
            };
        }

        let start = self.cursor.position();
        let Some(byte) = self.cursor.peek() else {
            return SchemaToken {
                kind: SchemaKind::EOF,
                text: "",
                offset: self.cursor.position(),
            };
        };

        let kind = match byte {
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                SchemaKind::WHITESPACE
            }
            b'/' if self.cursor.peek_next() == Some(b'/') => {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.skip_line();
                SchemaKind::COMMENT
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    SchemaKind::STRING
                } else {
                    SchemaKind::ERROR
                }
            }
            byte if Cursor::is_ident_start(byte) => {
                let text = self.cursor.scan_ident();
                SchemaKind::from_keyword(text).unwrap_or(SchemaKind::IDENT)
            }
            b'@' => {
                self.cursor.bump();
                SchemaKind::AT
            }
            b'(' => {
                self.cursor.bump();
                SchemaKind::L_PAREN
            }
            b')' => {
                self.cursor.bump();
                SchemaKind::R_PAREN
            }
            b'{' => {
                self.cursor.bump();
                SchemaKind::L_BRACE
            }
            b'}' => {
                self.cursor.bump();
                SchemaKind::R_BRACE
            }
            b'[' => {
                self.cursor.bump();
                SchemaKind::L_BRACKET
            }
            b']' => {
                self.cursor.bump();
                SchemaKind::R_BRACKET
            }
            b';' => {
                self.cursor.bump();
                SchemaKind::SEMI
            }
            b':' => {
                self.cursor.bump();
                if self.cursor.peek() == Some(b':') {
                    self.cursor.bump();
                    SchemaKind::COLON2
                } else {
                    SchemaKind::COLON
                }
            }
            b',' => {
                self.cursor.bump();
                SchemaKind::COMMA
            }
            b'.' => {
                self.cursor.bump();
                SchemaKind::DOT
            }
            b'?' => {
                self.cursor.bump();
                SchemaKind::QUESTION
            }
            b'=' => {
                self.cursor.bump();
                SchemaKind::EQ
            }
            b'<' => {
                self.cursor.bump();
                SchemaKind::LT
            }
            b'>' => {
                self.cursor.bump();
                SchemaKind::GT
            }
            _ => {
                self.cursor.bump_char();
                SchemaKind::ERROR
            }
        };

        SchemaToken {
            kind,
            text: self.cursor.slice(start),
            offset: start,
        }
    }
}
