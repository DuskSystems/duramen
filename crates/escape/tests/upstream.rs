//! Source: <https://github.com/cedar-policy/cedar/blob/v4.9.0/cedar-policy-core/src/parser/unescape.rs>.

extern crate alloc;

use alloc::borrow::Cow;

use duramen_ast::PatternElement;
use duramen_escape::{EscapeError, Escaper};
use duramen_test::insta::assert_snapshot;

#[test]
fn str_valid_ascii_escapes() {
    let result = Escaper::new(r#""\t\r\n\\\0\x42""#).unescape_str().unwrap();
    assert_eq!(result, "\t\r\n\\\0\x42");
}

#[test]
fn str_invalid_hex_out_of_range() {
    let errors = Escaper::new(r#""abc\xFFdef""#).unescape_str().unwrap_err();
    assert_eq!(errors, [EscapeError::OutOfRangeHexEscape { span: 4..8 }]);
    assert_snapshot!(errors[0].to_string(), @"out of range hex escape");
}

#[test]
fn str_valid_unicode_escapes() {
    let result = Escaper::new(r#""\u{0}\u{1}\u{1234}\u{12345}\u{054321}\u{123}\u{42}""#)
        .unescape_str()
        .unwrap();

    assert_eq!(result, "\u{0}\u{1}\u{1234}\u{12345}\u{054321}\u{123}\u{42}");
}

#[test]
fn str_invalid_unicode_out_of_range() {
    let errors = Escaper::new(r#""abc\u{1111111}\u{222222222}FFdef""#)
        .unescape_str()
        .unwrap_err();

    assert_eq!(
        errors,
        [
            EscapeError::OutOfRangeUnicodeEscape { span: 4..14 },
            EscapeError::OutOfRangeUnicodeEscape { span: 15..25 },
        ]
    );

    assert_snapshot!(errors[0].to_string(), @"out of range unicode escape");
    assert_snapshot!(errors[1].to_string(), @"out of range unicode escape");
}

#[test]
fn str_invalid_escape_sequences() {
    let errors = Escaper::new(r#""abc\*\bdef""#).unescape_str().unwrap_err();

    assert_eq!(
        errors,
        [
            EscapeError::InvalidEscape { span: 4..6 },
            EscapeError::InvalidEscape { span: 6..8 },
        ]
    );

    assert_snapshot!(errors[0].to_string(), @"invalid escape sequence");
    assert_snapshot!(errors[1].to_string(), @"invalid escape sequence");
}

#[test]
fn pattern_valid_ascii_with_escaped_star() {
    let result = Escaper::new(r#""\t\r\n\\\0\x42\*""#)
        .unescape_pattern()
        .unwrap();

    assert_eq!(
        result,
        [PatternElement::Literal(Cow::Owned(
            "\t\r\n\\\0\x42*".into()
        ))]
    );
}

#[test]
fn pattern_invalid_hex_out_of_range() {
    let errors = Escaper::new(r#""abc\xFF\xFEdef""#)
        .unescape_pattern()
        .unwrap_err();

    assert_eq!(
        errors,
        [
            EscapeError::OutOfRangeHexEscape { span: 4..8 },
            EscapeError::OutOfRangeHexEscape { span: 8..12 },
        ]
    );

    assert_snapshot!(errors[0].to_string(), @"out of range hex escape");
    assert_snapshot!(errors[1].to_string(), @"out of range hex escape");
}

#[test]
fn pattern_escaped_star_with_unicode() {
    let result = Escaper::new(r#""👀👀\*🤞🤞\*🤝""#)
        .unescape_pattern()
        .unwrap();

    assert_eq!(
        result,
        [PatternElement::Literal(Cow::Owned("👀👀*🤞🤞*🤝".into()))]
    );
}

#[test]
fn pattern_invalid_escape_sequences() {
    let errors = Escaper::new(r#""abc\d\bdef""#)
        .unescape_pattern()
        .unwrap_err();

    assert_eq!(
        errors,
        [
            EscapeError::InvalidEscape { span: 4..6 },
            EscapeError::InvalidEscape { span: 6..8 },
        ]
    );

    assert_snapshot!(errors[0].to_string(), @"invalid escape sequence");
    assert_snapshot!(errors[1].to_string(), @"invalid escape sequence");
}
