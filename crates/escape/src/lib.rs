#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use duramen_ast::PatternElement;
use rustc_literal_escaper::{EscapeError as RustcEscapeError, unescape_str};

mod error;
pub use error::EscapeError;

/// Handles unescaping of string and pattern literals.
pub struct Escaper<'r> {
    raw: &'r str,
    start: usize,
}

impl<'r> Escaper<'r> {
    #[must_use]
    pub const fn new(raw: &'r str, start: usize) -> Self {
        Self { raw, start }
    }

    /// Strips quotes and unescapes a string literal.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid escape sequences.
    pub fn unescape_str(&self) -> Result<String, Vec<EscapeError>> {
        let inner = Self::strip_quotes(self.raw);
        let start = self.start + 1;

        let mut output = String::with_capacity(inner.len());
        let mut errors = Vec::new();

        unescape_str(
            inner,
            &mut |range: Range<usize>, result: Result<char, RustcEscapeError>| match result {
                Ok(ch) => output.push(ch),
                Err(err) if err.is_fatal() => {
                    errors.push(EscapeError::new(
                        err,
                        (start + range.start)..(start + range.end),
                    ));
                }
                Err(_) => {}
            },
        );

        if errors.is_empty() {
            Ok(output)
        } else {
            Err(errors)
        }
    }

    /// Strips quotes and unescapes a pattern literal.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid escape sequences.
    pub fn unescape_pattern(&self) -> Result<Vec<PatternElement>, Vec<EscapeError>> {
        let inner = Self::strip_quotes(self.raw);
        let start = self.start + 1;

        let bytes = inner.as_bytes();

        let mut elements = Vec::new();
        let mut errors = Vec::new();

        unescape_str(
            inner,
            &mut |range: Range<usize>, result: Result<char, RustcEscapeError>| match result {
                Ok('*') => elements.push(PatternElement::Wildcard),
                Ok(ch) => elements.push(PatternElement::Char(ch)),
                Err(RustcEscapeError::InvalidEscape) if &bytes[range.clone()] == br"\*" => {
                    elements.push(PatternElement::Char('*'));
                }
                Err(err) if err.is_fatal() => {
                    errors.push(EscapeError::new(
                        err,
                        (start + range.start)..(start + range.end),
                    ));
                }
                Err(_) => {}
            },
        );

        if errors.is_empty() {
            Ok(elements)
        } else {
            Err(errors)
        }
    }

    /// Strips surrounding quotes from a string literal.
    #[must_use]
    pub fn strip_quotes(raw: &str) -> &str {
        let bytes = raw.as_bytes();

        // Quoted string
        if bytes.len() >= 2 && bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"' {
            return &raw[1..raw.len() - 1];
        }

        // Unterminated string
        if !bytes.is_empty() && bytes[0] == b'"' {
            return &raw[1..];
        }

        raw
    }
}
