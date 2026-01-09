use super::syntax::SchemaSyntax;
use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaToken<'a> {
    syntax: SchemaSyntax,
    text: &'a str,
    position: usize,
}

impl<'a> SchemaToken<'a> {
    #[must_use]
    pub const fn syntax(&self) -> SchemaSyntax {
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

        let syntax = match first {
            Cursor::END => SchemaSyntax::Eof,
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                SchemaSyntax::Whitespace
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    SchemaSyntax::String
                } else {
                    SchemaSyntax::Unknown
                }
            }
            byte if Cursor::is_digit(byte) => {
                self.cursor.scan_integer();
                SchemaSyntax::Integer
            }
            byte if Cursor::is_ident_start(byte) => {
                let text = self.cursor.scan_ident();
                SchemaSyntax::from_keyword(text).unwrap_or(SchemaSyntax::Identifier)
            }
            b'(' => {
                self.cursor.bump();
                SchemaSyntax::OpenParenthesis
            }
            b')' => {
                self.cursor.bump();
                SchemaSyntax::CloseParenthesis
            }
            b'{' => {
                self.cursor.bump();
                SchemaSyntax::OpenBrace
            }
            b'}' => {
                self.cursor.bump();
                SchemaSyntax::CloseBrace
            }
            b'[' => {
                self.cursor.bump();
                SchemaSyntax::OpenBracket
            }
            b']' => {
                self.cursor.bump();
                SchemaSyntax::CloseBracket
            }
            b',' => {
                self.cursor.bump();
                SchemaSyntax::Comma
            }
            b';' => {
                self.cursor.bump();
                SchemaSyntax::Semicolon
            }
            b':' => {
                self.cursor.bump();
                if self.cursor.current() == b':' {
                    self.cursor.bump();
                    SchemaSyntax::Colon2
                } else {
                    SchemaSyntax::Colon
                }
            }
            b'@' => {
                self.cursor.bump();
                SchemaSyntax::At
            }
            b'.' => {
                self.cursor.bump();
                SchemaSyntax::Dot
            }
            b'?' => {
                self.cursor.bump();
                SchemaSyntax::Question
            }
            b'<' => {
                self.cursor.bump();
                SchemaSyntax::LessThan
            }
            b'>' => {
                self.cursor.bump();
                SchemaSyntax::GreaterThan
            }
            b'=' => {
                self.cursor.bump();
                SchemaSyntax::Equal
            }
            b'|' => {
                self.cursor.bump();
                SchemaSyntax::Pipe
            }
            b'/' => {
                self.cursor.bump();
                if self.cursor.current() == b'/' {
                    self.cursor.bump();
                    self.cursor.skip_line();
                    SchemaSyntax::Comment
                } else {
                    SchemaSyntax::Slash
                }
            }
            _ => {
                self.cursor.bump_char();
                SchemaSyntax::Unknown
            }
        };

        SchemaToken {
            syntax,
            text: self.cursor.slice(start),
            position: start,
        }
    }
}
