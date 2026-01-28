//! Test cases sourced from:
//! <https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/validator/cedar_schema/test.rs>.

use core::error::Error;

use duramen_parser::SchemaParser;
use duramen_test::assert_snapshot;

#[test]
fn entity_decl_quoted() -> Result<(), Box<dyn Error>> {
    let source = r#"
      entity "A";
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn action_applesto_empty() -> Result<(), Box<dyn Error>> {
    let source = "
      action A in [B, C] appliesTo {};
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn action_resource_trailing_comma() -> Result<(), Box<dyn Error>> {
    let source = "
      action A in [B, C] appliesTo { principal: X, resource: [Y,]};
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn action_attributes_equals() -> Result<(), Box<dyn Error>> {
    let source = "
      action A in [B, C] appliesTo { principal: X, resource: [Y,Z]} = attributes {};
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn type_decl_quoted_name() -> Result<(), Box<dyn Error>> {
    let source = r#"
      type "A" = B;
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn type_decl_quoted_value() -> Result<(), Box<dyn Error>> {
    let source = r#"
      type A = "B";
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn enum_entity_empty() -> Result<(), Box<dyn Error>> {
    let source = "
      entity Application enum [ ];
      entity User in [ Application ];
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn action_eid_invalid_escape() -> Result<(), Box<dyn Error>> {
    let source = r#"
      namespace NS1 {
        entity PrincipalEntity = {};
        entity SystemEntity1 = {};
        entity SystemEntity2 in [SystemEntity1] = {};
        action "Group1";
      }
      namespace NS2 {
        entity SystemEntity1 in [NS1::SystemEntity2] = {};
        action "Group1" in [NS1::Action::"Group1"];
        action "Action1" in [Action::"\6"] appliesTo {
          principal: [NS1::PrincipalEntity],
          resource: [NS2::SystemEntity1],
          context: {}
        };
      }
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn just_context() -> Result<(), Box<dyn Error>> {
    let source = r#"
      action "Foo" appliesTo { context: {} };
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn just_principal() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      action \"Foo\" appliesTo { principal: a, context: {} };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn just_resource() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      action \"Foo\" appliesTo { resource: a, context: {} };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn resource_only() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      action \"Foo\" appliesTo {
        resource: [a]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn resources_only() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      entity b;
      action \"Foo\" appliesTo {
        resource: [a, b]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn principal_only() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      action \"Foo\" appliesTo {
        principal: [a]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn principals_only() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      entity b;
      action \"Foo\" appliesTo {
        principal: [a, b]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn empty_principal() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      entity b;
      action Foo appliesTo {
        principal: [],
        resource: [a, b]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn empty_resource() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      entity b;
      action Foo appliesTo {
        principal: [a, b],
        resource: []
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_principal() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      entity b;
      entity c;
      entity d;
      action \"Foo\" appliesTo {
        principal: [a, b],
        principal: [c]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_resource() -> Result<(), Box<dyn Error>> {
    let source = "
      entity a;
      entity b;
      entity c;
      entity d;
      action \"Foo\" appliesTo {
        resource: [a, b],
        resource: [c]
      };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn use_reserved_namespace() -> Result<(), Box<dyn Error>> {
    let source = "
      namespace __cedar {}
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn use_reserved_namespace_nested() -> Result<(), Box<dyn Error>> {
    let source = "
      namespace __cedar::Foo {}
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_namespace() -> Result<(), Box<dyn Error>> {
    let source = "
      namespace A {}
      namespace A {}
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_actions() -> Result<(), Box<dyn Error>> {
    let source = "
      action A;
      action A appliesTo { context: {} };
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_actions_quoted() -> Result<(), Box<dyn Error>> {
    let source = r#"
      action A;
      action "A";
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_actions_namespaced() -> Result<(), Box<dyn Error>> {
    let source = r#"
      namespace Foo {
        action A;
        action "A";
      };
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_entity_types() -> Result<(), Box<dyn Error>> {
    let source = "
      entity A;
      entity A {};
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_entity_types_comma() -> Result<(), Box<dyn Error>> {
    let source = "
      entity A,A {};
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_common_types() -> Result<(), Box<dyn Error>> {
    let source = "
      type A = Bool;
      type A = Long;
    ";

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}

#[test]
fn duplicate_annotations() -> Result<(), Box<dyn Error>> {
    let source = r#"
      @doc("This entity defines our central user type")
      @doc
      entity User {
        manager: User,
        team: String
      };
    "#;

    let result = SchemaParser::new(source).parse()?;
    assert!(result.has_errors());

    let rendered = result
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"");
    Ok(())
}
