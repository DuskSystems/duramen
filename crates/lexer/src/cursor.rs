use crate::lookup::{IDENTIFIER_TABLE, INTEGER_TABLE, WHITESPACE_TABLE};

/// A saved position that can be restored later.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Checkpoint(usize);

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
    #[must_use]
    const fn bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    /// Returns the current position in the source.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Saves the current position for later restoration.
    #[must_use]
    pub const fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.position)
    }

    /// Restores a previously saved position.
    pub const fn restore(&mut self, checkpoint: Checkpoint) {
        self.position = checkpoint.0;
    }

    /// Returns the current byte.
    #[must_use]
    pub fn current(&self) -> Option<u8> {
        self.bytes().get(self.position).copied()
    }

    /// Returns a slice from `start` to current position.
    #[must_use]
    pub fn slice(&self, start: usize) -> &'a str {
        &self.source[start..self.position]
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

    /// Skips whitespace characters.
    pub fn skip_whitespace(&mut self) -> bool {
        let start = self.position;

        let mut position = start;
        while let Some(&byte) = self.bytes().get(position) {
            // ASCII
            if byte < 128 {
                if WHITESPACE_TABLE[byte as usize] {
                    position += 1;
                    continue;
                }

                break;
            }

            // Unicode
            let Some(remaining) = self.source.get(position..) else {
                break;
            };

            let Some(char) = remaining.chars().next() else {
                break;
            };

            if !char.is_whitespace() {
                break;
            }

            position += char.len_utf8();
        }

        self.position = position;
        position > start
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
        while let Some(remaining) = self.bytes().get(self.position..) {
            let Some(offset) = memchr::memchr2(b'"', b'\\', remaining) else {
                // Unterminated string
                self.position = self.bytes().len();
                return false;
            };

            self.position += offset;

            let Some(&byte) = self.bytes().get(self.position) else {
                return false;
            };

            if byte == b'"' {
                self.position += 1;
                return true;
            }

            // Escaped character
            self.bump();
            self.bump_char();
        }

        false
    }

    /// Scans an identifier.
    pub fn scan_identifier(&mut self) {
        let mut position = self.position;
        while let Some(&byte) = self.bytes().get(position) {
            if !IDENTIFIER_TABLE[byte as usize] {
                break;
            }

            position += 1;
        }

        self.position = position;
    }

    /// Scans an integer.
    pub fn scan_integer(&mut self) {
        let mut position = self.position;
        while let Some(&byte) = self.bytes().get(position) {
            if !INTEGER_TABLE[byte as usize] {
                break;
            }

            position += 1;
        }

        self.position = position;
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
