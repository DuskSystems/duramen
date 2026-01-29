//! Source: <https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/validator/cedar_schema/test.rs>.

use duramen_lower::SchemaLowerer;
use duramen_parser::SchemaParser;
use duramen_test::assert_snapshot;

#[test]
fn simple_entity() {
    let source = r"
        entity User;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn entity_with_attributes() {
    let source = r"
        entity User = {
            name: String,
            age: Long,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn entity_with_optional_attribute() {
    let source = r"
        entity User = {
            name: String,
            nickname?: String,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn entity_with_parent() {
    let source = r"
        entity User in [Group];
        entity Group;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn multiple_entity_names() {
    let source = r"
        entity User, Admin = {
            name: String,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn simple_action() {
    let source = r#"
        action "Foo";
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn action_with_applies_to() {
    let source = r#"
        entity User;
        entity Resource;
        action "Foo" appliesTo {
            principal: [User],
            resource: [Resource],
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn action_with_context() {
    let source = r#"
        entity User;
        entity Resource;
        action "Foo" appliesTo {
            principal: [User],
            resource: [Resource],
            context: {
                ip_address: String,
            },
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn action_with_parents() {
    let source = r#"
        entity User;
        entity Resource;
        action "Base" appliesTo {
            principal: [User],
            resource: [Resource],
        };
        action "Derived" in ["Base"] appliesTo {
            principal: [User],
            resource: [Resource],
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn multiple_action_names() {
    let source = r#"
        entity User;
        entity Resource;
        action "Read", "Write" appliesTo {
            principal: [User],
            resource: [Resource],
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn type_alias() {
    let source = r"
        type Email = String;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn record_type() {
    let source = r"
        type Address = {
            street: String,
            city: String,
            zip: Long,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn set_type() {
    let source = r"
        type Tags = Set<String>;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn entity_reference_type() {
    let source = r"
        entity User;
        type Owner = User;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn enum_type() {
    let source = r#"
        type Status = enum["active", "inactive", "pending"];
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn empty_enum() {
    let source = r"
        type Status = enum[];
    ";

    let tree = SchemaParser::new(source).parse();
    let diagnostics = SchemaLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: expected enum variant
      ╭▸ test.cedarschema:2:23
      │
    2 │         type Status = enum[];
      ╰╴                      ━━━━━━ expected enum variant
    ");
}

#[test]
fn simple_namespace() {
    let source = r"
        namespace MyApp {
            entity User;
        }
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn nested_namespace_path() {
    let source = r"
        namespace MyApp::Users {
            entity User;
        }
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn nested_namespace_declaration() {
    let source = r"
        namespace Outer {
            namespace Inner {
                entity User;
            }
        }
    ";

    let tree = SchemaParser::new(source).parse();
    let diagnostics = SchemaLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: nested namespaces are not supported
      ╭▸ test.cedarschema:3:13
      │
    3 │ ┏             namespace Inner {
    4 │ ┃                 entity User;
    5 │ ┃             }
    6 │ ┃         }
      ╰╴┗━━━━━━━━┛ nested namespace
    ");
}

#[test]
fn multiple_namespaces() {
    let source = r"
        namespace App1 {
            entity User;
        }
        namespace App2 {
            entity Resource;
        }
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn entity_annotation() {
    let source = r#"
        @doc("A user entity")
        entity User;
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn action_annotation() {
    let source = r#"
        entity User;
        entity Resource;
        @doc("Read action")
        action "Read" appliesTo {
            principal: [User],
            resource: [Resource],
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn duplicate_annotation() {
    let source = r#"
        @doc("First")
        @doc("Second")
        entity User;
    "#;

    let tree = SchemaParser::new(source).parse();
    let diagnostics = SchemaLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: duplicate annotation `@doc`
      ╭▸ test.cedarschema:3:9
      │
    2 │ ┌          @doc("First")
    3 │ │          @doc("Second")
      │ │┏━━━━━━━━│┛
      │ └┃────────┤
      │  ┃        first defined here
    4 │  ┃         entity User;
      ╰╴ ┗━━━━━━━━┛ duplicate annotation
    "#);
}

#[test]
fn annotation_without_value() {
    let source = r"
        @deprecated
        entity User;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn primitive_types() {
    let source = r"
        entity User = {
            name: String,
            age: Long,
            active: Bool,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn nested_record() {
    let source = r"
        entity User = {
            address: {
                street: String,
                city: String,
            },
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn set_of_entities() {
    let source = r"
        entity User;
        entity Group = {
            members: Set<User>,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn named_type_reference() {
    let source = r"
        type Email = String;
        entity User = {
            email: Email,
        };
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn empty_entity_name() {
    let source = r"
        entity;
    ";

    let tree = SchemaParser::new(source).parse();
    // Parser may catch this, but lowerer should handle gracefully
    drop(SchemaLowerer::new(source).lower(tree.tree()));
}

#[test]
fn invalid_escape_in_string() {
    let source = r#"
        entity User = {
            "na\qme": String,
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let diagnostics = SchemaLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid escape sequence: invalid escape
      ╭▸ test.cedarschema:3:13
      │
    3 │             "na\qme": String,
      ╰╴            ━━━━━━━━━━━━━━━━ invalid escape
    "#);
}

#[test]
fn invalid_escape_in_annotation() {
    let source = r#"
        @doc("\q")
        entity User;
    "#;

    let tree = SchemaParser::new(source).parse();
    let diagnostics = SchemaLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedarschema", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid escape sequence: invalid escape
      ╭▸ test.cedarschema:2:9
      │
    2 │ ┏         @doc("\q")
    3 │ ┃         entity User;
      ╰╴┗━━━━━━━━┛ invalid escape
    "#);
}

#[test]
fn entity_with_tags() {
    let source = r"
        entity User tags String;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn entity_with_attributes_and_tags() {
    let source = r"
        entity User = {
            name: String,
        } tags Long;
    ";

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn photo_sharing_app() {
    let source = r#"
        namespace PhotoApp {
            entity User = {
                name: String,
                email: String,
            };

            entity Photo = {
                title: String,
                owner: User,
            };

            entity Album in [User] = {
                name: String,
                photos: Set<Photo>,
            };

            action "View", "Edit", "Delete" appliesTo {
                principal: [User],
                resource: [Photo, Album],
            };

            action "Share" appliesTo {
                principal: [User],
                resource: [Photo, Album],
                context: {
                    recipients: Set<User>,
                },
            };
        }
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn github_style() {
    let source = r#"
        entity User;
        entity Team in [Team];
        entity Organization = {
            name: String,
        };
        entity Repository in [Organization] = {
            name: String,
            visibility: String,
        };

        action "Read" appliesTo {
            principal: [User, Team],
            resource: [Repository],
        };

        action "Write" appliesTo {
            principal: [User, Team],
            resource: [Repository],
        };

        action "Admin" appliesTo {
            principal: [User],
            resource: [Repository, Organization],
        };
    "#;

    let tree = SchemaParser::new(source).parse();
    let result = SchemaLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}
