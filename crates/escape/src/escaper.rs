use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use duramen_ast::PatternElement;
use memchr::{memchr2, memchr3};

use crate::error::EscapeError;

/// Handles unescaping of string and pattern literals.
pub struct Escaper<'a> {
    raw: &'a str,
    start: usize,
}

impl<'a> Escaper<'a> {
    #[must_use]
    pub const fn new(raw: &'a str, start: usize) -> Self {
        Self { raw, start }
    }

    /// Strips quotes and unescapes a string literal.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid escape sequences.
    pub fn unescape_str(&self) -> Result<Cow<'a, str>, Vec<EscapeError>> {
        let inner = Self::strip_quotes(self.raw);
        let start = self.start + 1;

        let bytes = inner.as_bytes();

        let Some(first) = memchr2(b'\\', b'\r', bytes) else {
            return Ok(Cow::Borrowed(inner));
        };

        let mut output = String::with_capacity(inner.len());
        output.push_str(&inner[..first]);

        let mut errors = Vec::new();

        let mut position = first;
        while position < bytes.len() {
            match bytes[position] {
                b'\\' => match Self::parse_escape(&inner[position + 1..], start + position) {
                    Ok((char, consumed)) => {
                        output.push(char);
                        position += 1 + consumed;
                    }
                    Err(error) => {
                        position = error.span().end - start;
                        errors.push(error);
                    }
                },
                b'\r' => {
                    errors.push(EscapeError::BareCarriageReturn {
                        span: (start + position)..(start + position + 1),
                    });

                    position += 1;
                }
                _ => {
                    let rest = &bytes[position..];
                    if let Some(next) = memchr2(b'\\', b'\r', rest) {
                        output.push_str(&inner[position..position + next]);
                        position += next;
                    } else {
                        output.push_str(&inner[position..]);
                        break;
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(Cow::Owned(output))
        } else {
            Err(errors)
        }
    }

    /// Strips quotes and unescapes a pattern literal.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid escape sequences.
    pub fn unescape_pattern(&self) -> Result<Vec<PatternElement<'a>>, Vec<EscapeError>> {
        let inner = Self::strip_quotes(self.raw);
        let start = self.start + 1;

        let bytes = inner.as_bytes();

        let Some(first) = memchr3(b'\\', b'\r', b'*', bytes) else {
            if inner.is_empty() {
                return Ok(Vec::new());
            }

            return Ok(vec![PatternElement::Literal(Cow::Borrowed(inner))]);
        };

        let mut elements = Vec::new();
        let mut errors = Vec::new();

        let mut owned: Option<String> = None;
        let mut section = 0;

        let mut position = first;
        while position < bytes.len() {
            match memchr3(b'\\', b'\r', b'*', &bytes[position..]) {
                None => {
                    if let Some(buffer) = &mut owned {
                        buffer.push_str(&inner[position..]);
                    }

                    break;
                }
                Some(offset) => {
                    let next = position + offset;

                    if let Some(buffer) = &mut owned {
                        buffer.push_str(&inner[position..next]);
                    }

                    position = next;
                    match bytes[position] {
                        b'*' => {
                            Self::flush_segment(
                                &mut elements,
                                &mut owned,
                                inner,
                                section,
                                position,
                            );

                            elements.push(PatternElement::Wildcard);

                            position += 1;
                            section = position;
                        }
                        b'\r' => {
                            Self::flush_segment(
                                &mut elements,
                                &mut owned,
                                inner,
                                section,
                                position,
                            );

                            errors.push(EscapeError::BareCarriageReturn {
                                span: (start + position)..(start + position + 1),
                            });

                            position += 1;
                            section = position;
                        }
                        b'\\' => {
                            if bytes.get(position + 1) == Some(&b'*') {
                                let buffer = owned.get_or_insert_with(|| {
                                    let mut output = String::with_capacity(bytes.len() - section);
                                    output.push_str(&inner[section..position]);
                                    output
                                });

                                buffer.push('*');
                                position += 2;
                            } else {
                                match Self::parse_escape(&inner[position + 1..], start + position) {
                                    Ok((decoded, consumed)) => {
                                        let buffer = owned.get_or_insert_with(|| {
                                            let mut output =
                                                String::with_capacity(bytes.len() - section);
                                            output.push_str(&inner[section..position]);
                                            output
                                        });

                                        buffer.push(decoded);
                                        position += 1 + consumed;
                                    }
                                    Err(error) => {
                                        Self::flush_segment(
                                            &mut elements,
                                            &mut owned,
                                            inner,
                                            section,
                                            position,
                                        );

                                        position = error.span().end - start;
                                        section = position;

                                        errors.push(error);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Self::flush_segment(&mut elements, &mut owned, inner, section, bytes.len());

        if errors.is_empty() {
            Ok(elements)
        } else {
            Err(errors)
        }
    }

    fn flush_segment(
        elements: &mut Vec<PatternElement<'a>>,
        owned: &mut Option<String>,
        inner: &'a str,
        section: usize,
        position: usize,
    ) {
        if let Some(buffer) = owned.take() {
            if !buffer.is_empty() {
                elements.push(PatternElement::Literal(Cow::Owned(buffer)));
            }

            return;
        }

        if section < position {
            elements.push(PatternElement::Literal(Cow::Borrowed(
                &inner[section..position],
            )));
        }
    }

    /// Strips surrounding quotes from a string literal.
    #[must_use]
    pub fn strip_quotes(input: &str) -> &str {
        let bytes = input.as_bytes();

        // Quoted string
        if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
            return &input[1..input.len() - 1];
        }

        // Unterminated string
        if !bytes.is_empty() && bytes[0] == b'"' {
            return &input[1..];
        }

        input
    }

    fn parse_escape(after: &str, offset: usize) -> Result<(char, usize), EscapeError> {
        let bytes = after.as_bytes();

        let Some(&next) = bytes.first() else {
            return Err(EscapeError::LoneSlash {
                span: offset..offset + 1,
            });
        };

        match next {
            b'n' => Ok(('\n', 1)),
            b'r' => Ok(('\r', 1)),
            b't' => Ok(('\t', 1)),
            b'\\' => Ok(('\\', 1)),
            b'0' => Ok(('\0', 1)),
            b'\'' => Ok(('\'', 1)),
            b'"' => Ok(('"', 1)),
            b'x' => Self::parse_hex_escape(&after[1..], offset),
            b'u' => Self::parse_unicode_escape(&after[1..], offset),
            _ => {
                let len = after.chars().next().map_or(1, char::len_utf8);
                Err(EscapeError::InvalidEscape {
                    span: offset..offset + 1 + len,
                })
            }
        }
    }

    fn parse_hex_escape(after: &str, offset: usize) -> Result<(char, usize), EscapeError> {
        let bytes = after.as_bytes();

        if bytes.len() < 2 {
            return Err(EscapeError::InvalidHexEscape {
                span: offset..offset + 2 + after.len(),
            });
        }

        let Some(high) = Self::hex_digit(bytes[0]) else {
            let len = after.chars().next().map_or(1, char::len_utf8);
            return Err(EscapeError::InvalidHexEscape {
                span: offset..offset + 2 + len,
            });
        };

        let Some(low) = Self::hex_digit(bytes[1]) else {
            let len = after[1..].chars().next().map_or(1, char::len_utf8);
            return Err(EscapeError::InvalidHexEscape {
                span: offset..offset + 3 + len,
            });
        };

        let value = high * 16 + low;
        if value > 0x7F {
            return Err(EscapeError::OutOfRangeHexEscape {
                span: offset..offset + 4,
            });
        }

        Ok((char::from(value as u8), 3))
    }

    fn parse_unicode_escape(after: &str, offset: usize) -> Result<(char, usize), EscapeError> {
        let bytes = after.as_bytes();

        if bytes.first() != Some(&b'{') {
            return Err(EscapeError::InvalidUnicodeEscape {
                span: offset..offset + 2,
            });
        }

        let mut value = 0;

        let mut digits = 0;
        let mut position = 1;

        if bytes.get(position) == Some(&b'_') {
            return Err(EscapeError::InvalidUnicodeEscape {
                span: offset..offset + 2 + position + 1,
            });
        }

        loop {
            let Some(&byte) = bytes.get(position) else {
                return Err(EscapeError::InvalidUnicodeEscape {
                    span: offset..offset + 2 + position,
                });
            };

            match byte {
                b'}' => {
                    position += 1;
                    break;
                }
                b'_' => {
                    position += 1;
                }
                _ => {
                    let Some(digit) = Self::hex_digit(byte) else {
                        let len = after[position..].chars().next().map_or(1, char::len_utf8);
                        return Err(EscapeError::InvalidUnicodeEscape {
                            span: offset..offset + 2 + position + len,
                        });
                    };

                    digits += 1;

                    if digits > 6 {
                        return Err(EscapeError::OutOfRangeUnicodeEscape {
                            span: offset..offset + 2 + position + 1,
                        });
                    }

                    value = value * 16 + digit;
                    position += 1;
                }
            }
        }

        if digits == 0 {
            return Err(EscapeError::InvalidUnicodeEscape {
                span: offset..offset + 2 + position,
            });
        }

        let Some(char) = char::from_u32(value) else {
            return Err(EscapeError::OutOfRangeUnicodeEscape {
                span: offset..offset + 2 + position,
            });
        };

        Ok((char, position + 1))
    }

    fn hex_digit(byte: u8) -> Option<u32> {
        match byte {
            b'0'..=b'9' => Some(u32::from(byte - b'0')),
            b'a'..=b'f' => Some(u32::from(byte - b'a') + 10),
            b'A'..=b'F' => Some(u32::from(byte - b'A') + 10),
            _ => None,
        }
    }
}
