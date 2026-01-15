use alloc::vec::Vec;

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
    diagnostics: Vec<Diagnostic>,
}

impl<'a> PolicyLexer<'a> {
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

    pub fn next_token(&mut self) -> PolicyToken<'a> {
        let start = self.cursor.position();
        let first = self.cursor.current();

        let syntax = match first {
            Cursor::END if self.cursor.position() >= self.cursor.source().len() => {
                PolicySyntax::Eof
            }
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
                let len = self.cursor.unicode_whitespace_len();
                if len > 0 {
                    self.cursor.skip_whitespace();
                    PolicySyntax::Whitespace
                } else {
                    self.cursor.bump_char();

                    let end = self.cursor.position();
                    self.diagnostics.push(
                        Diagnostic::error("unexpected character")
                            .with_label(start..end, "not recognized"),
                    );

                    PolicySyntax::Unknown
                }
            }
        };

        PolicyToken {
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

    fn render(source: &str, lexer: &mut PolicyLexer<'_>) -> String {
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

        let mut lexer = PolicyLexer::new(source);
        while lexer.next_token().syntax() != PolicySyntax::Eof {}

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

        let mut lexer = PolicyLexer::new(source);
        while lexer.next_token().syntax() != PolicySyntax::Eof {}

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
        let source = "permit # forbid";

        let mut lexer = PolicyLexer::new(source);
        while lexer.next_token().syntax() != PolicySyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @"
        error: unexpected character
          ╭▸ <test>:1:8
          │
        1 │ permit # forbid
          ╰╴       ━ not recognized
        ");
    }

    #[test]
    fn unexpected_character_emoji() {
        let source = "permit 🦀 forbid";

        let mut lexer = PolicyLexer::new(source);
        while lexer.next_token().syntax() != PolicySyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @"
        error: unexpected character
          ╭▸ <test>:1:8
          │
        1 │ permit 🦀 forbid
          ╰╴       ━━ not recognized
        ");
    }

    #[test]
    fn multiple_errors() {
        let source = r#"permit # "unterminated"#;

        let mut lexer = PolicyLexer::new(source);
        while lexer.next_token().syntax() != PolicySyntax::Eof {}

        insta::assert_snapshot!(render(source, &mut lexer), @r#"
        error: unexpected character
          ╭▸ <test>:1:8
          │
        1 │ permit # "unterminated
          ╰╴       ━ not recognized
        error: unterminated string
          ╭▸ <test>:1:10
          │
        1 │ permit # "unterminated
          ╰╴         ━━━━━━━━━━━━━ string is not closed
        "#);
    }
}
