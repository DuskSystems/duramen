use crate::cursor::Cursor;
use crate::lookup::ByteLookup;
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

    /// Returns the current byte offset.
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> usize {
        self.cursor.position()
    }

    /// Sets the byte offset.
    #[inline(always)]
    pub const fn set_offset(&mut self, offset: usize) {
        self.cursor.set_position(offset);
    }

    /// Returns the next token.
    #[inline]
    pub fn next_token(&mut self) -> Option<Token> {
        self.cursor.current()?;

        let start = self.cursor.position();
        let kind = self.scan_token();
        let len = self.cursor.position() - start;

        Some(Token::new(kind, len))
    }

    /// Scans the next token.
    #[inline]
    fn scan_token(&mut self) -> TokenKind {
        let Some(current) = self.cursor.current() else {
            return TokenKind::Unknown;
        };

        // Whitespace
        if self.cursor.skip_whitespace() {
            return TokenKind::Whitespace;
        }

        // Identifier or Keyword
        if ByteLookup::is_identifier_start(current) {
            let start = self.cursor.position();
            self.cursor.scan_identifier();

            let Some(text) = self.cursor.slice(start) else {
                return TokenKind::Unknown;
            };

            return TokenKind::from_identifier(text);
        }

        // Integer
        if ByteLookup::is_digit(current) {
            self.cursor.scan_integer();
            return TokenKind::Integer;
        }

        // String
        if current == b'"' {
            self.cursor.bump();

            if self.cursor.scan_string() {
                return TokenKind::String;
            }

            return TokenKind::StringUnterminated;
        }

        // Comment
        if current == b'/' && self.cursor.peek() == Some(b'/') {
            self.cursor.bump_n(2);
            self.cursor.skip_line();
            return TokenKind::Comment;
        }

        // Punctuation
        if let Some((kind, len)) = TokenKind::from_punctuation(current, self.cursor.peek()) {
            self.cursor.bump_n(len as usize);
            return kind;
        }

        // Unknown character
        self.cursor.bump_char();

        TokenKind::Unknown
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
    use duramen_test::assert_eq;

    use super::*;

    #[test]
    fn empty() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn whitespace() {
        let mut lexer = Lexer::new("  \t\n");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Whitespace, 4)));
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
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Permit, 6)));
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
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Eq2, 2)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new(".");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Dot, 1)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Lexer::new(">=");
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::GtEq, 2)));
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
