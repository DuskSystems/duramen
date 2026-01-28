//! Test cases sourced from:
//! <https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/parser/text_to_cst.rs>.

use core::error::Error;

use duramen_parser::PolicyParser;
use duramen_test::assert_snapshot;

#[test]
fn expr_overflow_negative() -> Result<(), Box<dyn Error>> {
    let source = "
      principal == -5555555555555555555555
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn expr_overflow_positive() -> Result<(), Box<dyn Error>> {
    let source = "
      principal == 5555555555555555555555
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn variable6() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(var : in 6, action, resource);
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn member7() -> Result<(), Box<dyn Error>> {
    let source = r#"
      permit(principal, action, resource)
      when{
        one{num:true,trivia:"first!"}
      };
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn ident3() -> Result<(), Box<dyn Error>> {
    let source = "
      if
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn ident4() -> Result<(), Box<dyn Error>> {
    let source = "
      if(true)
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn comments_policy_multiline() -> Result<(), Box<dyn Error>> {
    let source = "
      /* multi-line
      comment */
      permit(principal, action, resource)
      when{
        one.two
      };
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn comments_expr_multiline() -> Result<(), Box<dyn Error>> {
    let source = "
      1 /* multi-line
      comment */d
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotations_bad_id_hyphen() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @bad-annotation("bad") permit (principal, action, resource);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotations_bad_id_space() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @hi mom("this should be invalid")
      permit(principal, action, resource);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotations_bad_id_plus() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @hi+mom("this should be invalid")
      permit(principal, action, resource);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotations_bad_val_multiple() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @bad_annotation("bad","annotation") permit (principal, action, resource);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotations_bad_val_empty() -> Result<(), Box<dyn Error>> {
    let source = "
      @bad_annotation() permit (principal, action, resource);
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotations_bad_val_ident() -> Result<(), Box<dyn Error>> {
    let source = "
      @bad_annotation(bad_annotation) permit (principal, action, resource);
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn policy_annotation_bad_position() -> Result<(), Box<dyn Error>> {
    let source = r#"
      permit (@comment("your name here") principal, action, resource);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn error_recovery_single() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, !) when { principal.foo == resource.bar};
      permit(principal, action, resource);
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn error_recovery_multiple() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, !) when { principal.foo == resource.bar};
      permit(principal, action, +);
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn error_recovery_no_semicolon() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, !) when { principal.foo == resource.bar}
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn extended_has_paren_dot() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, resource) when {
        principal has a.(b)
      };
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn extended_has_number() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, resource) when {
        principal has a.1
      };
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn invalid_token_tilde() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, resource) when { ~ };
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn invalid_token_emoji() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, resource) when { 🚀 };
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn unclosed_string_when() -> Result<(), Box<dyn Error>> {
    let source = r#"
      permit(principal, action, resource) when {
        principal.foo = "bar
      };
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn unclosed_string_entity() -> Result<(), Box<dyn Error>> {
    let source = r#"
      permit(principal, action, resource == Photo::"mine.jpg);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn unclosed_string_annotation() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @id("0)permit(principal, action, resource);
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn unclosed_string_annotation_when() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @id("0)
      permit(principal, action, resource) when {
        principal.foo = "bar"
      };
    "#;

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn single_quote_string_when() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, resource) when {
        principal.foo = 'bar'
      };
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn single_quote_string_entity() -> Result<(), Box<dyn Error>> {
    let source = "
      permit(principal, action, resource == Photo::'mine.jpg');
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn single_quote_string_annotation() -> Result<(), Box<dyn Error>> {
    let source = "
      @id('0')permit(principal, action, resource);
    ";

    let result = PolicyParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}
