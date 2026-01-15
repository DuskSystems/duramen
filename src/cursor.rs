use core::num::NonZeroUsize;

pub struct Cursor<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Cursor<'a> {
    pub const END: u8 = 0;

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
    pub fn current(&self) -> u8 {
        self.source
            .as_bytes()
            .get(self.position)
            .copied()
            .unwrap_or(Self::END)
    }

    #[inline]
    #[must_use]
    pub fn peek(&self, peek: NonZeroUsize) -> u8 {
        self.source
            .as_bytes()
            .get(self.position.saturating_add(peek.get()))
            .copied()
            .unwrap_or(Self::END)
    }

    #[inline]
    pub const fn bump(&mut self) {
        self.position = self.position.saturating_add(1);
    }

    #[inline]
    pub fn bump_char(&mut self) {
        if let Some(char) = self
            .source
            .get(self.position..)
            .and_then(|str| str.chars().next())
        {
            self.position = self.position.saturating_add(char.len_utf8());
        }
    }

    #[inline]
    #[must_use]
    pub fn slice(&self, start: usize) -> &'a str {
        self.source.get(start..self.position).unwrap_or_default()
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let byte = self.current();
            if byte == Self::END {
                break;
            }

            if Self::is_whitespace(byte) {
                self.bump();
                continue;
            }

            let len = self.unicode_whitespace_len();
            if len > 0 {
                self.position = self.position.saturating_add(len);
            } else {
                break;
            }
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

            match self.current() {
                b'"' => {
                    self.bump();
                    return true;
                }
                b'\\' => {
                    self.bump();
                    self.bump_char();
                }
                _ => return false,
            }
        }
    }

    #[inline]
    pub fn scan_ident(&mut self) -> &'a str {
        let start = self.position;

        loop {
            let byte = self.current();
            if byte == Self::END || !Self::is_ident_continue(byte) {
                break;
            }

            self.bump();
        }

        self.slice(start)
    }

    pub fn scan_integer(&mut self) {
        loop {
            let byte = self.current();
            if byte == Self::END || !Self::is_digit(byte) {
                break;
            }

            self.bump();
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_digit(byte: u8) -> bool {
        byte.is_ascii_digit()
    }

    #[inline]
    #[must_use]
    pub const fn is_whitespace(byte: u8) -> bool {
        matches!(byte, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
    }

    #[must_use]
    pub fn unicode_whitespace_len(&self) -> usize {
        let Some(remaining) = self.source.get(self.position..) else {
            return 0;
        };

        let Some(char) = remaining.chars().next() else {
            return 0;
        };

        if char.is_whitespace() {
            char.len_utf8()
        } else {
            0
        }
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
