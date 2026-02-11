use alloc::borrow::Cow;

use duramen_ast::PatternElement;
use duramen_escape::{EscapeError, Escaper};
use duramen_test::insta::assert_snapshot;

extern crate alloc;

// -- str --

#[test]
fn str_borrowed() {
    let result = Escaper::new(r#""hello""#).unescape_str().unwrap();
    assert_eq!(result, Cow::Borrowed("hello"));
}

#[test]
fn str_single_quote() {
    let result = Escaper::new(r#""it\'s""#).unescape_str().unwrap();
    assert_eq!(result, "it's");
}

#[test]
fn str_double_quote() {
    let result = Escaper::new(r#""say \"hi\"""#).unescape_str().unwrap();
    assert_eq!(result, r#"say "hi""#);
}

#[test]
fn str_unicode_underscore() {
    let result = Escaper::new(r#""\u{1_F6_00}""#).unescape_str().unwrap();
    assert_eq!(result, "\u{1F600}");
}

// -- pattern --

#[test]
fn pattern_empty() {
    let result = Escaper::new(r#""""#).unescape_pattern().unwrap();
    assert_eq!(result, []);
}

#[test]
fn pattern_borrowed() {
    let result = Escaper::new(r#""abc""#).unescape_pattern().unwrap();
    assert_eq!(result, [PatternElement::Literal(Cow::Borrowed("abc"))]);
}

#[test]
fn pattern_wildcard() {
    let result = Escaper::new(r#""a*b""#).unescape_pattern().unwrap();
    assert_eq!(
        result,
        [
            PatternElement::Literal(Cow::Borrowed("a")),
            PatternElement::Wildcard,
            PatternElement::Literal(Cow::Borrowed("b")),
        ]
    );
}

// -- strip_quotes --

#[test]
fn strip_quotes() {
    assert_eq!(Escaper::strip_quotes(r#""hello""#), "hello");
    assert_eq!(Escaper::strip_quotes(r#""unterminated"#), "unterminated");
    assert_eq!(Escaper::strip_quotes("bare"), "bare");
}

// -- offset --

#[test]
fn offset() {
    let error = EscapeError::InvalidEscape { span: 1..3 }.offset(10);
    assert_eq!(error, EscapeError::InvalidEscape { span: 11..13 });
}

// -- errors --

#[test]
fn lone_slash() {
    let errors = Escaper::new(r#""\"#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::LoneSlash { span: 1..2 }]);
    assert_snapshot!(errors[0].to_string(), @"unexpected end of escape sequence");
}

#[test]
fn bare_cr() {
    let errors = Escaper::new("\"\r\"").unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::BareCarriageReturn { span: 1..2 }]);
    assert_snapshot!(errors[0].to_string(), @"bare carriage return not allowed");
}

#[test]
fn hex_invalid_chars() {
    let errors = Escaper::new(r#""\xGG""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidHexEscape { span: 1..4 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid hex escape");
}

#[test]
fn hex_truncated() {
    let errors = Escaper::new(r#""\x""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidHexEscape { span: 1..3 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid hex escape");
}

#[test]
fn hex_second_digit() {
    let errors = Escaper::new(r#""\x4G""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidHexEscape { span: 1..5 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid hex escape");
}

#[test]
fn unicode_empty() {
    let errors = Escaper::new(r#""\u{}""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidUnicodeEscape { span: 1..5 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid unicode escape");
}

#[test]
fn unicode_no_brace() {
    let errors = Escaper::new(r#""\u1234""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidUnicodeEscape { span: 1..3 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid unicode escape");
}

#[test]
fn unicode_leading_underscore() {
    let errors = Escaper::new(r#""\u{_1}""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidUnicodeEscape { span: 1..5 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid unicode escape");
}

#[test]
fn unicode_unterminated() {
    let errors = Escaper::new(r#""\u{1234""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidUnicodeEscape { span: 1..8 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid unicode escape");
}

#[test]
fn unicode_invalid_chars() {
    let errors = Escaper::new(r#""\u{GH}""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::InvalidUnicodeEscape { span: 1..5 }]);
    assert_snapshot!(errors[0].to_string(), @"invalid unicode escape");
}

#[test]
fn unicode_out_of_range() {
    let errors = Escaper::new(r#""\u{110000}""#).unescape_str().unwrap_err();
    assert_eq!(
        errors,
        [EscapeError::OutOfRangeUnicodeEscape { span: 1..11 }]
    );
    assert_snapshot!(errors[0].to_string(), @"out of range unicode escape");
}
