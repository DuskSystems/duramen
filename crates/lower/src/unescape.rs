use alloc::string::String;
use alloc::vec::Vec;
use core::iter::Peekable;
use core::str::Chars;

use duramen_ast as ast;

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
        let mut result = String::with_capacity(self.input.len());
        let mut chars = self.input.chars().peekable();

        while let Some(char) = chars.next() {
            if char != '\\' {
                result.push(char);
                continue;
            }

            let escape = chars.next()?;

            match escape {
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                '\'' => result.push('\''),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                '0' => result.push('\0'),
                'x' => result.push(parse_hex_escape(&mut chars)?),
                'u' => result.push(parse_unicode_escape(&mut chars)?),
                _ => return None,
            }
        }

        Some(result)
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
        let mut elements = Vec::new();
        let mut chars = self.input.chars().peekable();

        while let Some(char) = chars.next() {
            if char == '*' {
                elements.push(ast::policy::PatternElem::Wildcard);
                continue;
            }

            if char != '\\' {
                elements.push(ast::policy::PatternElem::Char(char));
                continue;
            }

            match chars.next()? {
                '\\' => elements.push(ast::policy::PatternElem::Char('\\')),
                '"' => elements.push(ast::policy::PatternElem::Char('"')),
                '\'' => elements.push(ast::policy::PatternElem::Char('\'')),
                'n' => elements.push(ast::policy::PatternElem::Char('\n')),
                'r' => elements.push(ast::policy::PatternElem::Char('\r')),
                't' => elements.push(ast::policy::PatternElem::Char('\t')),
                '0' => elements.push(ast::policy::PatternElem::Char('\0')),
                '*' => elements.push(ast::policy::PatternElem::Char('*')),
                'x' => elements.push(ast::policy::PatternElem::Char(parse_hex_escape(
                    &mut chars,
                )?)),
                'u' => elements.push(ast::policy::PatternElem::Char(parse_unicode_escape(
                    &mut chars,
                )?)),
                _ => return None,
            }
        }

        Some(ast::policy::Pattern::new(elements))
    }
}

fn parse_hex_escape(chars: &mut Peekable<Chars<'_>>) -> Option<char> {
    let mut value = 0_u32;
    for _ in 0..2 {
        let digit = chars.next()?.to_digit(16)?;
        value = value.checked_mul(16)?.checked_add(digit)?;
    }

    char::from_u32(value).filter(char::is_ascii)
}

fn parse_unicode_escape(chars: &mut Peekable<Chars<'_>>) -> Option<char> {
    if chars.next()? != '{' {
        return None;
    }

    let mut value = 0_u32;
    let mut has_digits = false;

    loop {
        match chars.next()? {
            '}' => break,
            char => {
                let digit = char.to_digit(16)?;
                value = value.checked_mul(16)?.checked_add(digit)?;
                has_digits = true;
            }
        }
    }

    if !has_digits {
        return None;
    }

    char::from_u32(value)
}
