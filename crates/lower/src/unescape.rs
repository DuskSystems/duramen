use alloc::string::String;
use alloc::vec::Vec;

use duramen_ast as ast;
use memchr::{memchr, memchr2};

pub struct StringUnescaper<'a> {
    input: &'a str,
}

impl<'a> StringUnescaper<'a> {
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self { input }
    }

    #[must_use]
    pub fn unescape(self) -> Option<String> {
        if memchr(b'\\', self.input.as_bytes()).is_none() {
            return Some(String::from(self.input));
        }

        let mut result = String::with_capacity(self.input.len());

        let mut chars = self.input.chars();
        while let Some(char) = chars.next() {
            if char == '\\' {
                result.push(Self::parse_escape(&mut chars)?);
            } else {
                result.push(char);
            }
        }

        Some(result)
    }

    #[inline(always)]
    fn parse_escape(chars: &mut impl Iterator<Item = char>) -> Option<char> {
        match chars.next()? {
            '\\' => Some('\\'),
            '"' => Some('"'),
            '\'' => Some('\''),
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            '0' => Some('\0'),
            'x' => parse_hex(chars),
            'u' => parse_unicode(chars),
            _ => None,
        }
    }
}

pub struct PatternUnescaper<'a> {
    input: &'a str,
}

impl<'a> PatternUnescaper<'a> {
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self { input }
    }

    #[must_use]
    pub fn unescape(self) -> Option<ast::policy::Pattern> {
        if memchr2(b'*', b'\\', self.input.as_bytes()).is_none() {
            return Some(
                self.input
                    .chars()
                    .map(ast::policy::PatternElem::Char)
                    .collect(),
            );
        }

        let mut elements = Vec::new();
        let mut chars = self.input.chars();

        while let Some(char) = chars.next() {
            match char {
                '*' => elements.push(ast::policy::PatternElem::Wildcard),
                '\\' => {
                    let escaped = Self::parse_escape(&mut chars)?;
                    elements.push(ast::policy::PatternElem::Char(escaped));
                }
                _ => elements.push(ast::policy::PatternElem::Char(char)),
            }
        }

        Some(ast::policy::Pattern::new(elements))
    }

    #[inline(always)]
    fn parse_escape(chars: &mut impl Iterator<Item = char>) -> Option<char> {
        match chars.next()? {
            '*' => Some('*'),
            '\\' => Some('\\'),
            '"' => Some('"'),
            '\'' => Some('\''),
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            '0' => Some('\0'),
            'x' => parse_hex(chars),
            'u' => parse_unicode(chars),
            _ => None,
        }
    }
}

#[inline(always)]
fn parse_hex(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    let high = chars.next()?.to_digit(16)?;
    let low = chars.next()?.to_digit(16)?;
    let value = high * 16 + low;
    (value < 128).then(|| char::from_u32(value))?
}

#[inline(always)]
fn parse_unicode(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    if chars.next()? != '{' {
        return None;
    }

    let first = chars.next()?;
    if first == '}' {
        return None;
    }

    let mut value = first.to_digit(16)?;
    loop {
        match chars.next()? {
            '}' => return char::from_u32(value),
            char => value = value.checked_mul(16)?.checked_add(char.to_digit(16)?)?,
        }
    }
}
