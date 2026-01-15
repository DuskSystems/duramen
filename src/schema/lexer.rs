use alloc::vec::Vec;

use super::syntax::SchemaSyntax;
use crate::cursor::Cursor;
use crate::diagnostics::Diagnostic;

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
    diagnostics: Vec<Diagnostic>,
}

impl<'a> SchemaLexer<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
            diagnostics: Vec::new(),
        }
    }

    #[must_use]
    pub const fn position(&self) -> usize {
        self.cursor.position()
    }

    pub fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        core::mem::take(&mut self.diagnostics)
    }

    pub fn next_token(&mut self) -> SchemaToken<'a> {
        let start = self.cursor.position();
        let first = self.cursor.current();

        let syntax = match first {
            Cursor::END if self.cursor.position() >= self.cursor.source().len() => {
                SchemaSyntax::Eof
            }
            byte if Cursor::is_whitespace(byte) => {
                self.cursor.skip_whitespace();
                SchemaSyntax::Whitespace
            }
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    SchemaSyntax::String
                } else {
                    let end = self.cursor.position();
                    self.diagnostics.push(
                        Diagnostic::error("unterminated string")
                            .with_label(start..end, "string is not closed"),
                    );

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
                let len = self.cursor.unicode_whitespace_len();
                if len > 0 {
                    self.cursor.skip_whitespace();
                    SchemaSyntax::Whitespace
                } else {
                    self.cursor.bump_char();

                    let end = self.cursor.position();
                    self.diagnostics.push(
                        Diagnostic::error("unexpected character")
                            .with_label(start..end, "not recognized"),
                    );

                    SchemaSyntax::Unknown
                }
            }
        };

        SchemaToken {
            syntax,
            text: self.cursor.slice(start),
            position: start,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use super::*;

    fn render(source: &str, lexer: &mut SchemaLexer<'_>) -> String {
        let outputs: Vec<_> = lexer
            .take_diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.render("<test>", source))
            .collect();

        outputs
            .iter()
            .map(|output| anstream::adapter::strip_str(output).to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn unterminated_string() {
        let source = r#""hello"#;

        let mut lexer = SchemaLexer::new(source);
        while lexer.next_token().syntax() != SchemaSyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @r#"
        error: unterminated string
          ╭▸ <test>:1:1
          │
        1 │ "hello
          ╰╴━━━━━━ string is not closed
        "#);
    }

    #[test]
    fn unterminated_string_with_escape() {
        let source = r#""hello\""#;

        let mut lexer = SchemaLexer::new(source);
        while lexer.next_token().syntax() != SchemaSyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @r#"
        error: unterminated string
          ╭▸ <test>:1:1
          │
        1 │ "hello\"
          ╰╴━━━━━━━━ string is not closed
        "#);
    }

    #[test]
    fn unexpected_character() {
        let source = "entity # User";

        let mut lexer = SchemaLexer::new(source);
        while lexer.next_token().syntax() != SchemaSyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @"
        error: unexpected character
          ╭▸ <test>:1:8
          │
        1 │ entity # User
          ╰╴       ━ not recognized
        ");
    }

    #[test]
    fn unexpected_character_emoji() {
        let source = "entity 🦀 User";

        let mut lexer = SchemaLexer::new(source);
        while lexer.next_token().syntax() != SchemaSyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @"
        error: unexpected character
          ╭▸ <test>:1:8
          │
        1 │ entity 🦀 User
          ╰╴       ━━ not recognized
        ");
    }

    #[test]
    fn multiple_errors() {
        let source = r#"entity # "unterminated"#;

        let mut lexer = SchemaLexer::new(source);
        while lexer.next_token().syntax() != SchemaSyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @r#"
        error: unexpected character
          ╭▸ <test>:1:8
          │
        1 │ entity # "unterminated
          ╰╴       ━ not recognized
        error: unterminated string
          ╭▸ <test>:1:10
          │
        1 │ entity # "unterminated
          ╰╴         ━━━━━━━━━━━━━ string is not closed
        "#);
    }
}
