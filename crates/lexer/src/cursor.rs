/// Cursor for traversing the source.
pub struct Cursor<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new cursor at the start of the source.
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    /// Returns the current byte position.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns the current byte.
    #[must_use]
    pub fn current(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }

    /// Returns the next byte.
    #[must_use]
    pub fn peek(&self) -> Option<u8> {
        self.source
            .as_bytes()
            .get(self.position.checked_add(1)?)
            .copied()
    }

    /// Advance by one byte.
    pub const fn bump(&mut self) {
        if let Some(position) = self.position.checked_add(1) {
            self.position = position;
        }
    }

    /// Advance by `n` bytes.
    pub fn bump_n(&mut self, n: u8) {
        for _ in 0..n {
            self.bump();
        }
    }

    /// Advance by one UTF-8 character.
    pub fn bump_char(&mut self) {
        if let Some(remaining) = self.source.get(self.position..)
            && let Some(char) = remaining.chars().next()
            && let Some(position) = self.position.checked_add(char.len_utf8())
        {
            self.position = position;
        }
    }

    /// Returns a slice from `start` to current position.
    #[must_use]
    pub fn slice(&self, start: usize) -> Option<&'a str> {
        self.source.get(start..self.position)
    }

    /// Skips whitespace characters.
    pub fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.bump();
        }
    }

    /// Skips to end of line.
    pub fn skip_line(&mut self) {
        let Some(remaining) = self.source.as_bytes().get(self.position..) else {
            return;
        };

        if let Some(offset) = memchr::memchr2(b'\n', b'\r', remaining)
            && let Some(position) = self.position.checked_add(offset)
        {
            self.position = position;
        } else {
            self.position = self.source.len();
        }
    }

    /// Scans a string literal after the opening quote.
    ///
    /// Returns `true` if properly terminated, `false` if unterminated.
    pub fn scan_string(&mut self) -> bool {
        let bytes = self.source.as_bytes();

        loop {
            let Some(remaining) = bytes.get(self.position..) else {
                return false;
            };

            let Some(offset) = memchr::memchr2(b'"', b'\\', remaining) else {
                self.position = self.source.len();
                return false;
            };

            let Some(position) = self.position.checked_add(offset) else {
                return false;
            };

            self.position = position;

            match self.current() {
                Some(b'"') => {
                    self.bump();
                    return true;
                }
                Some(b'\\') => {
                    self.bump();
                    self.bump_char();
                }
                _ => return false,
            }
        }
    }

    /// Scans an identifier.
    pub fn scan_identifier(&mut self) {
        while self.is_identifier_continue() {
            self.bump();
        }
    }

    /// Scans an integer.
    pub fn scan_integer(&mut self) {
        while self.is_digit() {
            self.bump();
        }
    }

    /// Checks if current byte is whitespace.
    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        matches!(
            self.current(),
            Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
        )
    }

    /// Checks if current byte is a digit.
    #[must_use]
    pub fn is_digit(&self) -> bool {
        matches!(self.current(), Some(b'0'..=b'9'))
    }

    /// Checks if current byte can start an identifier.
    #[must_use]
    pub fn is_identifier_start(&self) -> bool {
        matches!(self.current(), Some(b'A'..=b'Z' | b'a'..=b'z' | b'_'))
    }

    /// Checks if current byte can continue an identifier.
    #[must_use]
    pub fn is_identifier_continue(&self) -> bool {
        matches!(
            self.current(),
            Some(b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_char() {
        let mut cursor = Cursor::new("🦀x");
        cursor.bump_char();
        assert_eq!(cursor.current(), Some(b'x'));

        let mut cursor = Cursor::new("");
        cursor.bump_char();
        assert_eq!(cursor.current(), None);
    }

    #[test]
    fn skip_whitespace() {
        let mut cursor = Cursor::new("  \t\n\r\x0B\x0Cx");
        cursor.skip_whitespace();
        assert_eq!(cursor.current(), Some(b'x'));
    }

    #[test]
    fn skip_line() {
        let mut cursor = Cursor::new("hello\nworld");
        cursor.skip_line();
        assert_eq!(cursor.current(), Some(b'\n'));

        let mut cursor = Cursor::new("hello");
        cursor.skip_line();
        assert_eq!(cursor.current(), None);

        let mut cursor = Cursor::new("");
        cursor.skip_line();
        assert_eq!(cursor.current(), None);
    }

    #[test]
    fn scan_string() {
        let mut cursor = Cursor::new("hello\"x");
        assert!(cursor.scan_string());
        assert_eq!(cursor.current(), Some(b'x'));

        let mut cursor = Cursor::new("one\\\"two\"x");
        assert!(cursor.scan_string());
        assert_eq!(cursor.current(), Some(b'x'));

        let mut cursor = Cursor::new("\\🦀\"x");
        assert!(cursor.scan_string());
        assert_eq!(cursor.current(), Some(b'x'));

        let mut cursor = Cursor::new("hello");
        assert!(!cursor.scan_string());
        assert_eq!(cursor.current(), None);
    }

    #[test]
    fn scan_identifier() {
        let mut cursor = Cursor::new("abc123_!");
        cursor.scan_identifier();
        assert_eq!(cursor.current(), Some(b'!'));
    }

    #[test]
    fn scan_integer() {
        let mut cursor = Cursor::new("12345x");
        cursor.scan_integer();
        assert_eq!(cursor.current(), Some(b'x'));
    }
}
