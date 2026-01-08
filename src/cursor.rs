pub struct Cursor<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Cursor<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    #[must_use]
    pub const fn source(&self) -> &'a str {
        self.source
    }

    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    pub const fn seek(&mut self, position: usize) {
        self.position = position;
    }

    #[inline]
    #[must_use]
    pub fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }

    #[inline]
    #[must_use]
    pub fn peek_next(&self) -> Option<u8> {
        self.source
            .as_bytes()
            .get(self.position.saturating_add(1))
            .copied()
    }

    #[inline]
    pub const fn bump(&mut self) {
        self.position = self.position.saturating_add(1);
    }

    #[inline]
    pub fn bump_char(&mut self) {
        if let Some(c) = self
            .source
            .get(self.position..)
            .and_then(|str| str.chars().next())
        {
            self.position = self.position.saturating_add(c.len_utf8());
        }
    }

    #[must_use]
    pub fn slice(&self, start: usize) -> &'a str {
        self.source.get(start..self.position).unwrap_or_default()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(byte) = self.peek() {
            if !Self::is_whitespace(byte) {
                break;
            }

            self.bump();
        }
    }

    pub fn skip_line(&mut self) {
        if let Some(remaining) = self.source.as_bytes().get(self.position..) {
            if let Some(position) = memchr::memchr2(b'\n', b'\r', remaining) {
                self.position = self.position.saturating_add(position);
            } else {
                self.position = self.source.len();
            }
        }
    }

    pub fn scan_string(&mut self) -> bool {
        loop {
            let Some(remaining) = self.source.as_bytes().get(self.position..) else {
                return false;
            };

            let Some(position) = memchr::memchr2(b'"', b'\\', remaining) else {
                self.position = self.source.len();
                return false;
            };

            self.position = self.position.saturating_add(position);

            match self.peek() {
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

    pub fn scan_ident(&mut self) -> &'a str {
        let start = self.position;
        while let Some(byte) = self.peek() {
            if !Self::is_ident_continue(byte) {
                break;
            }

            self.bump();
        }

        self.slice(start)
    }

    pub fn scan_integer(&mut self) {
        while let Some(byte) = self.peek() {
            if !Self::is_digit(byte) {
                break;
            }

            self.bump();
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_digit(byte: u8) -> bool {
        byte.is_ascii_digit()
    }

    #[inline]
    #[must_use]
    pub const fn is_whitespace(byte: u8) -> bool {
        byte.is_ascii_whitespace()
    }

    #[inline]
    #[must_use]
    pub const fn is_ident_start(byte: u8) -> bool {
        byte.is_ascii_alphabetic() || byte == b'_'
    }

    #[inline]
    #[must_use]
    pub const fn is_ident_continue(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_'
    }
}
