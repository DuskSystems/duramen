use crate::cursor::Cursor;
use crate::token::{Token, TokenKind};

/// Lexer for source code.
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given source.
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }

    /// Peeks at the next token without consuming it.
    #[must_use]
    pub fn peek_token(&mut self) -> Option<Token> {
        let checkpoint = self.cursor.checkpoint();
        let token = self.next_token();
        self.cursor.restore(checkpoint);
        token
    }

    /// Peeks at the kind of the next token without consuming it.
    #[must_use]
    pub fn peek(&mut self) -> Option<TokenKind> {
        self.peek_token().map(|token| token.kind)
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.cursor.current()?;

        let start = self.cursor.position();
        let kind = self.scan_token();
        let len = self.cursor.position() - start;

        Some(Token::new(kind, len))
    }

    /// Scans the next token.
    fn scan_token(&mut self) -> TokenKind {
        let Some(current) = self.cursor.current() else {
            return TokenKind::Unknown;
        };

        match current {
            // Newline
            b'\n' | b'\r' => {
                self.cursor.skip_newline();
                TokenKind::Newline
            }
            // Whitespace
            b' ' | b'\t' | 0x0B | 0x0C => {
                self.cursor.skip_whitespace();
                TokenKind::Whitespace
            }
            // Identifier Start
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let start = self.cursor.position();
                self.cursor.scan_identifier();
                TokenKind::from_identifier(self.cursor.slice(start))
            }
            // Digits
            b'0'..=b'9' => {
                self.cursor.scan_integer();
                TokenKind::Integer
            }
            // String
            b'"' => {
                self.cursor.bump();
                if self.cursor.scan_string() {
                    TokenKind::String
                } else {
                    TokenKind::StringUnterminated
                }
            }
            b'/' => {
                if self.cursor.peek() == Some(b'/') {
                    self.cursor.bump_n(2);
                    self.cursor.skip_line();
                    TokenKind::Comment
                } else {
                    self.cursor.bump();
                    TokenKind::Slash
                }
            }
            b'(' => {
                self.cursor.bump();
                TokenKind::OpenParenthesis
            }
            b')' => {
                self.cursor.bump();
                TokenKind::CloseParenthesis
            }
            b'{' => {
                self.cursor.bump();
                TokenKind::OpenBrace
            }
            b'}' => {
                self.cursor.bump();
                TokenKind::CloseBrace
            }
            b'[' => {
                self.cursor.bump();
                TokenKind::OpenBracket
            }
            b']' => {
                self.cursor.bump();
                TokenKind::CloseBracket
            }
            b'@' => {
                self.cursor.bump();
                TokenKind::At
            }
            b',' => {
                self.cursor.bump();
                TokenKind::Comma
            }
            b'.' => {
                self.cursor.bump();
                TokenKind::Dot
            }
            b'?' => {
                self.cursor.bump();
                TokenKind::QuestionMark
            }
            b';' => {
                self.cursor.bump();
                TokenKind::Semicolon
            }
            b'+' => {
                self.cursor.bump();
                TokenKind::Plus
            }
            b'-' => {
                self.cursor.bump();
                TokenKind::Minus
            }
            b'*' => {
                self.cursor.bump();
                TokenKind::Asterisk
            }
            b'%' => {
                self.cursor.bump();
                TokenKind::Percent
            }
            b':' => {
                if self.cursor.peek() == Some(b':') {
                    self.cursor.bump_n(2);
                    TokenKind::Colon2
                } else {
                    self.cursor.bump();
                    TokenKind::Colon
                }
            }
            b'&' => {
                if self.cursor.peek() == Some(b'&') {
                    self.cursor.bump_n(2);
                    TokenKind::Ampersand2
                } else {
                    self.cursor.bump();
                    TokenKind::Ampersand
                }
            }
            b'|' => {
                if self.cursor.peek() == Some(b'|') {
                    self.cursor.bump_n(2);
                    TokenKind::Pipe2
                } else {
                    self.cursor.bump();
                    TokenKind::Pipe
                }
            }
            b'!' => {
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump_n(2);
                    TokenKind::BangEquals
                } else {
                    self.cursor.bump();
                    TokenKind::Bang
                }
            }
            b'=' => {
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump_n(2);
                    TokenKind::Equals2
                } else {
                    self.cursor.bump();
                    TokenKind::Equals
                }
            }
            b'<' => {
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump_n(2);
                    TokenKind::LessThanEquals
                } else {
                    self.cursor.bump();
                    TokenKind::LessThan
                }
            }
            b'>' => {
                if self.cursor.peek() == Some(b'=') {
                    self.cursor.bump_n(2);
                    TokenKind::GreaterThanEquals
                } else {
                    self.cursor.bump();
                    TokenKind::GreaterThan
                }
            }
            // Non ASCII
            128.. => {
                if self.cursor.skip_whitespace() {
                    TokenKind::Whitespace
                } else {
                    self.cursor.bump_char();
                    TokenKind::Unknown
                }
            }
            _ => {
                self.cursor.bump();
                TokenKind::Unknown
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn whitespace() {
        let mut lexer = Lexer::new("  \t\n");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Whitespace, 3)));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Newline, 1)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn integer() {
        let mut lexer = Lexer::new("365");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Integer, 3)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn string() {
        let mut lexer = Lexer::new(r#""alice""#);
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::String, 7)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new(r#""VacationPhoto94.jpg"#);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::StringUnterminated, 20))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("department");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Identifier, 10)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn keyword() {
        let mut lexer = Lexer::new("permit");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::PermitKeyword, 6)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn comment() {
        let mut lexer = Lexer::new("// jane's friends view-permission policy");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Comment, 40)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn punctuation() {
        let mut lexer = Lexer::new("::==");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Colon2, 2)));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Equals2, 2)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new(".");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Dot, 1)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new(">=");
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::GreaterThanEquals, 2))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn unknown() {
        let mut lexer = Lexer::new("#");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Unknown, 1)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new("🦀");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Unknown, 4)));
        assert_eq!(lexer.next(), None);
    }
}
