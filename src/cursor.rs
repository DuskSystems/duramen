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
    pub const fn position(&self) -> usize {
        self.position
    }

    #[inline(always)]
    #[must_use]
    pub fn current(&self) -> u8 {
        self.source
            .as_bytes()
            .get(self.position)
            .copied()
            .unwrap_or(0)
    }

    #[inline(always)]
    pub const fn bump(&mut self) {
        self.position += 1;
    }

    #[inline(always)]
    pub const fn bump_n(&mut self, count: usize) {
        self.position += count;
    }

    #[inline(always)]
    pub fn bump_char(&mut self) {
        if let Some(char) = self
            .source
            .get(self.position..)
            .and_then(|str| str.chars().next())
        {
            self.position += char.len_utf8();
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn slice(&self, start: usize) -> &'a str {
        self.source.get(start..self.position).unwrap_or_default()
    }

    #[inline(always)]
    pub fn skip_whitespace(&mut self) {
        let bytes = self.source.as_bytes();

        while let Some(&byte) = bytes.get(self.position) {
            if Self::is_whitespace(byte) {
                self.position += 1;
                continue;
            }

            if byte >= 0x80 {
                let len = self.unicode_whitespace_len();
                if len > 0 {
                    self.position += len;
                    continue;
                }
            }

            break;
        }
    }

    #[inline(always)]
    pub fn skip_line(&mut self) {
        if let Some(remaining) = self.source.as_bytes().get(self.position..) {
            if let Some(position) = memchr::memchr2(b'\n', b'\r', remaining) {
                self.position += position;
            } else {
                self.position = self.source.len();
            }
        }
    }

    #[inline(always)]
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

            self.position += offset;

            match self.current() {
                b'"' => {
                    self.position += 1;
                    return true;
                }
                b'\\' => {
                    self.position += 1;
                    self.bump_char();
                }
                _ => return false,
            }
        }
    }

    #[inline(always)]
    pub fn scan_ident(&mut self) -> &'a str {
        let start = self.position;
        let bytes = self.source.as_bytes();

        let end = bytes.get(start..).map_or(0, |slice| {
            slice
                .iter()
                .position(|&byte| !Self::is_ident_continue(byte))
                .unwrap_or(slice.len())
        });

        self.position = start + end;
        self.slice(start)
    }

    pub fn scan_integer(&mut self) {
        let bytes = self.source.as_bytes();

        let end = bytes.get(self.position..).map_or(0, |slice| {
            slice
                .iter()
                .position(|&byte| !Self::is_digit(byte))
                .unwrap_or(slice.len())
        });

        self.position += end;
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_digit(byte: u8) -> bool {
        byte.is_ascii_digit()
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_whitespace(byte: u8) -> bool {
        byte.is_ascii_whitespace() || byte == 0x0B
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_eof(&self) -> bool {
        self.position >= self.source.len()
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

    #[inline(always)]
    #[must_use]
    pub const fn is_ident_start(byte: u8) -> bool {
        byte.is_ascii_alphabetic() || byte == b'_'
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_ident_continue(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_'
    }
}
