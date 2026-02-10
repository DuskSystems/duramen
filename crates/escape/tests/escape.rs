use alloc::borrow::Cow;

use duramen_ast::PatternElement;
use duramen_escape::Escaper;
use duramen_test::insta::assert_snapshot;

extern crate alloc;

#[test]
fn str() {
    let result = Escaper::new(r#""hello\nworld""#, 0).unescape_str().unwrap();
    assert_eq!(result, "hello\nworld");
}

#[test]
fn str_borrowed() {
    let result = Escaper::new(r#""hello""#, 0).unescape_str().unwrap();
    assert_eq!(result, Cow::Borrowed("hello"));
}

#[test]
fn str_error() {
    let errors = Escaper::new(r#""\q""#, 0).unescape_str().unwrap_err();
    assert_eq!(errors.len(), 1);
}

#[test]
fn str_multiple_errors() {
    let errors = Escaper::new(r#""\q\w""#, 0).unescape_str().unwrap_err();
    assert_eq!(errors.len(), 2);
}

#[test]
fn str_offset() {
    let errors = Escaper::new(r#""\q""#, 10).unescape_str().unwrap_err();
    assert_eq!(errors[0].span().clone(), 11..13);
}

#[test]
fn pattern_wildcard() {
    let result = Escaper::new(r#""a*b""#, 0).unescape_pattern().unwrap();
    assert_eq!(
        result,
        [
            PatternElement::Literal(Cow::Borrowed("a")),
            PatternElement::Wildcard,
            PatternElement::Literal(Cow::Borrowed("b")),
        ]
    );
}

#[test]
fn pattern_escaped_star() {
    let result = Escaper::new(r#""a\*b""#, 0).unescape_pattern().unwrap();
    assert_eq!(result, [PatternElement::Literal(Cow::Owned("a*b".into()))]);
}

#[test]
fn pattern_error() {
    let errors = Escaper::new(r#""\q""#, 0).unescape_pattern().unwrap_err();
    assert_eq!(errors.len(), 1);
}

#[test]
fn strip_quotes() {
    assert_eq!(Escaper::strip_quotes(r#""hello""#), "hello");
    assert_eq!(Escaper::strip_quotes(r#""unterminated"#), "unterminated");
    assert_eq!(Escaper::strip_quotes("bare"), "bare");
}

#[test]
fn error_invalid_escape() {
    let errors = Escaper::new(r#""\q""#, 0).unescape_str().unwrap_err();
    assert_snapshot!(errors[0].to_string(), @"invalid escape sequence");
}

#[test]
fn error_lone_slash() {
    let errors = Escaper::new(r#""\"#, 0).unescape_str().unwrap_err();
    assert_snapshot!(errors[0].to_string(), @"unexpected end of escape sequence");
}

#[test]
fn error_bare_cr() {
    let errors = Escaper::new("\"\r\"", 0).unescape_str().unwrap_err();
    assert_snapshot!(errors[0].to_string(), @"bare carriage return not allowed");
}

#[test]
fn error_hex_format() {
    let errors = Escaper::new(r#""\xGG""#, 0).unescape_str().unwrap_err();
    assert_snapshot!(errors[0].to_string(), @"invalid hex escape");
}

#[test]
fn error_hex_range() {
    let errors = Escaper::new(r#""\xFF""#, 0).unescape_str().unwrap_err();
    assert_snapshot!(errors[0].to_string(), @"out of range hex escape");
}

#[test]
fn error_unicode_format() {
    let errors = Escaper::new(r#""\u{}""#, 0).unescape_str().unwrap_err();
    assert_snapshot!(errors[0].to_string(), @"invalid unicode escape");
}

#[test]
fn error_unicode_range() {
    let errors = Escaper::new(r#""\u{110000}""#, 0)
        .unescape_str()
        .unwrap_err();

    assert_snapshot!(errors[0].to_string(), @"out of range unicode escape");
}
