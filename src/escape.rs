use alloc::borrow::Cow;
use alloc::string::String;

use bumpalo::Bump;
use memchr::memchr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyEscape<'a> {
    inner: &'a str,
    has_escapes: bool,
}

impl<'a> LazyEscape<'a> {
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        let inner = Self::strip_quotes(text);
        let has_escapes = memchr(b'\\', inner.as_bytes()).is_some();
        Self { inner, has_escapes }
    }

    // FIXME
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            inner: "",
            has_escapes: false,
        }
    }

    pub const fn inner(&self) -> &str {
        self.inner
    }

    #[must_use]
    pub fn unescape(&self) -> Cow<'a, str> {
        if self.has_escapes {
            Cow::Owned(Self::unescape_inner(self.inner))
        } else {
            Cow::Borrowed(self.inner)
        }
    }

    #[must_use]
    pub fn unescape_in(&self, bump: &'a Bump) -> &'a str {
        if self.has_escapes {
            bump.alloc_str(&Self::unescape_inner(self.inner))
        } else {
            self.inner
        }
    }

    fn strip_quotes(text: &str) -> &str {
        if text.len() >= 2 {
            if text.starts_with('"') && text.ends_with('"') {
                return &text[1..text.len() - 1];
            }

            if text.starts_with('\'') && text.ends_with('\'') {
                return &text[1..text.len() - 1];
            }
        }

        text
    }

    fn unescape_inner(string: &str) -> String {
        let mut result = String::with_capacity(string.len());

        let bytes = string.as_bytes();
        let mut index = 0;

        while index < bytes.len() {
            let slice = &bytes[index..];

            if let Some(position) = memchr(b'\\', slice) {
                if position > 0 {
                    result.push_str(&string[index..index + position]);
                }

                index += position + 1;

                let Some(&escape_byte) = bytes.get(index) else {
                    result.push('\\');
                    break;
                };

                index += 1;

                match escape_byte {
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'"' => result.push('"'),
                    b'\'' => result.push('\''),
                    b'0' => result.push('\0'),
                    b'\\' => result.push('\\'),
                    b'*' => result.push('*'),
                    b'x' => {
                        if index + 1 < bytes.len() {
                            let hex_1 = bytes[index];
                            let hex_2 = bytes[index + 1];

                            if hex_1.is_ascii_hexdigit()
                                && hex_2.is_ascii_hexdigit()
                                && let Ok(number) =
                                    u8::from_str_radix(&string[index..index + 2], 16)
                            {
                                result.push(number as char);
                                index += 2;
                                continue;
                            }
                        }

                        result.push('\\');
                        result.push('x');
                    }
                    b'u' => {
                        if bytes.get(index) == Some(&b'{') {
                            index += 1;

                            let start = index;
                            while index < bytes.len()
                                && bytes[index] != b'}'
                                && bytes[index].is_ascii_hexdigit()
                            {
                                index += 1;
                            }

                            if let Ok(number) = u32::from_str_radix(&string[start..index], 16)
                                && let Some(character) = char::from_u32(number)
                            {
                                result.push(character);
                                if bytes.get(index) == Some(&b'}') {
                                    index += 1;
                                }

                                continue;
                            }

                            result.push_str("\\u{");
                        } else {
                            result.push('\\');
                            result.push('u');
                        }
                    }
                    other => {
                        result.push('\\');
                        result.push(other as char);
                    }
                }
            } else {
                result.push_str(&string[index..]);
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::ToOwned as _;

    use super::*;

    #[test]
    fn borrowed() {
        let lazy = LazyEscape::new("\"hello world\"");
        let unescape = lazy.unescape();
        assert_eq!(unescape, Cow::Borrowed("hello world"));
    }

    #[test]
    fn owned() {
        let lazy = LazyEscape::new("\"hello\\nworld\"");
        let unescape = lazy.unescape();
        assert_eq!(unescape, Cow::Owned::<str>("hello\nworld".to_owned()));
    }
}
