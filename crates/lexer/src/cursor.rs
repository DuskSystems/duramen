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

    /// Returns the source as bytes.
    #[inline]
    const fn bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    /// Returns the current byte position.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Sets the byte position.
    pub const fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// Returns the current byte.
    #[must_use]
    pub fn current(&self) -> Option<u8> {
        self.bytes().get(self.position).copied()
    }

    /// Returns the next byte.
    #[must_use]
    pub fn peek(&self) -> Option<u8> {
        self.bytes().get(self.position + 1).copied()
    }

    /// Advance by one byte.
    pub const fn bump(&mut self) {
        self.position = self.position + 1;
    }

    /// Advance by `n` bytes.
    pub const fn bump_n(&mut self, n: usize) {
        self.position = self.position + n;
    }

    /// Advance by one UTF-8 character.
    pub fn bump_char(&mut self) {
        if let Some(remaining) = self.source.get(self.position..)
            && let Some(char) = remaining.chars().next()
        {
            self.position += char.len_utf8();
        }
    }

    /// Returns a slice from `start` to current position.
    #[must_use]
    pub fn slice(&self, start: usize) -> Option<&'a str> {
        self.source.get(start..self.position)
    }

    /// Skips whitespace characters.
    pub fn skip_whitespace(&mut self) -> bool {
        let start = self.position;

        while let Some(remaining) = self.source.get(self.position..)
            && let Some(char) = remaining.chars().next()
        {
            if !char.is_whitespace() {
                break;
            }

            self.position += char.len_utf8();
        }

        self.position > start
    }

    /// Skips to end of line.
    pub fn skip_line(&mut self) {
        if let Some(remaining) = self.bytes().get(self.position..) {
            if let Some(offset) = memchr::memchr2(b'\n', b'\r', remaining) {
                self.position += offset;
            } else {
                self.position = self.bytes().len();
            }
        }
    }

    /// Scans a string literal after the opening quote.
    ///
    /// Returns `true` if properly terminated, `false` if unterminated.
    #[must_use]
    pub fn scan_string(&mut self) -> bool {
        while let Some(remaining) = self.source.get(self.position..)
            && let Some(char) = remaining.chars().next()
        {
            self.position += char.len_utf8();

            match char {
                '"' => return true,
                '\\' => {
                    // Skip the escaped character
                    if let Some(remaining) = self.source.get(self.position..)
                        && let Some(escaped) = remaining.chars().next()
                    {
                        self.position += escaped.len_utf8();
                    }
                }
                _ => {}
            }
        }

        false
    }

    /// Scans an identifier.
    pub fn scan_identifier(&mut self) {
        while let Some(&byte) = self.bytes().get(self.position) {
            if !byte.is_ascii_alphanumeric() && byte != b'_' {
                break;
            }

            self.position += 1;
        }
    }

    /// Scans an integer.
    pub fn scan_integer(&mut self) {
        while let Some(&byte) = self.bytes().get(self.position) {
            if !byte.is_ascii_digit() {
                break;
            }

            self.position += 1;
        }
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

        let mut cursor = Cursor::new("\u{00A0}\u{2003}\u{3000}x");
        cursor.skip_whitespace();
        assert_eq!(cursor.current(), Some(b'x'));
    }

    #[test]
    fn skip_line() {
        let mut cursor = Cursor::new("view-permission policy\npermit");
        cursor.skip_line();
        assert_eq!(cursor.current(), Some(b'\n'));

        let mut cursor = Cursor::new("view-permission policy");
        cursor.skip_line();
        assert_eq!(cursor.current(), None);

        let mut cursor = Cursor::new("");
        cursor.skip_line();
        assert_eq!(cursor.current(), None);
    }

    #[test]
    fn scan_string() {
        let mut cursor = Cursor::new("alice\";");
        assert!(cursor.scan_string());
        assert_eq!(cursor.current(), Some(b';'));

        let mut cursor = Cursor::new("jane\\\"s_photo\"x");
        assert!(cursor.scan_string());
        assert_eq!(cursor.current(), Some(b'x'));

        let mut cursor = Cursor::new("\\🦀\"x");
        assert!(cursor.scan_string());
        assert_eq!(cursor.current(), Some(b'x'));

        let mut cursor = Cursor::new("VacationPhoto94.jpg");
        assert!(!cursor.scan_string());
        assert_eq!(cursor.current(), None);
    }

    #[test]
    fn scan_identifier() {
        let mut cursor = Cursor::new("jobLevel;");
        cursor.scan_identifier();
        assert_eq!(cursor.current(), Some(b';'));
    }

    #[test]
    fn scan_integer() {
        let mut cursor = Cursor::new("365 ");
        cursor.scan_integer();
        assert_eq!(cursor.current(), Some(b' '));
    }
}
