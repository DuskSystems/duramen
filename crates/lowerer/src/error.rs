use alloc::format;
use alloc::string::String;
use core::ops::Range;

use duramen_diagnostic::{Diagnostic, Suggestion};

pub enum LowerError {
    MissingEffect {
        span: Range<usize>,
    },
    InvalidScopeOperator {
        span: Range<usize>,
        variable: String,
    },
    ContextInScope {
        span: Range<usize>,
    },
    MissingExpression {
        span: Range<usize>,
        expected: &'static str,
    },
    UnaryOpLimit {
        span: Range<usize>,
        count: usize,
    },
    UnsupportedIndex {
        span: Range<usize>,
    },
    UnknownVariable {
        span: Range<usize>,
        name: String,
    },
    UnknownMethod {
        span: Range<usize>,
        name: String,
    },
    UnknownFunction {
        span: Range<usize>,
        name: String,
    },
    WrongArgumentCount {
        span: Range<usize>,
        function: String,
        expected: usize,
        found: usize,
    },

    NestedNamespace {
        span: Range<usize>,
    },
    QualifiedEntityName {
        span: Range<usize>,
    },
    QualifiedTypeName {
        span: Range<usize>,
    },
    InvalidContextType {
        span: Range<usize>,
    },
    MissingTypeExpression {
        span: Range<usize>,
    },

    InvalidEquals {
        span: Range<usize>,
    },
}

impl From<LowerError> for Diagnostic {
    fn from(value: LowerError) -> Self {
        match value {
            LowerError::MissingEffect { span } => Self::error("missing policy effect")
                .with_label(span, "expected `permit` or `forbid`"),
            LowerError::InvalidScopeOperator { span, variable } => {
                let label = if variable == "action" {
                    "expected `==` or `in`"
                } else {
                    "expected `==`, `in`, `is`, or `is ... in`"
                };

                Self::error(format!("invalid scope operator for `{variable}`"))
                    .with_label(span, label)
            }
            LowerError::ContextInScope { span } => Self::error("`context` is not a scope variable")
                .with_label(span, "not valid in policy scope")
                .with_note("`context` can only be used in policy conditions, not in scope"),
            LowerError::MissingExpression { span, expected } => {
                Self::error("missing expression").with_label(span, expected)
            }
            LowerError::UnaryOpLimit { span, count } => {
                Self::error(format!("found {count} chained unary operators"))
                    .with_label(span, "at most 4 allowed")
            }
            LowerError::UnsupportedIndex { span } => Self::error("indexing is not supported")
                .with_label(span, "not supported")
                .with_note("use `has` and `.` instead"),
            LowerError::UnknownVariable { span, name } => {
                Self::error(format!("unknown variable `{name}`"))
                    .with_label(span, "not a valid variable")
                    .with_note(
                        "`principal`, `action`, `resource`, and `context` are the only variables",
                    )
            }
            LowerError::UnknownMethod { span, name } => {
                Self::error(format!("unknown method `{name}`")).with_label(span, "unknown method")
            }
            LowerError::UnknownFunction { span, name } => {
                Self::error(format!("`{name}` is not a known function"))
                    .with_label(span, "unknown function")
            }
            LowerError::WrongArgumentCount {
                span,
                function,
                expected,
                found,
            } => Self::error(format!(
                "`{function}` expects {expected} argument(s), found {found}"
            ))
            .with_label(span, format!("expected {expected} argument(s)")),

            LowerError::NestedNamespace { span } => {
                Self::error("nested namespaces are not supported")
                    .with_label(span, "nested namespace")
            }
            LowerError::QualifiedEntityName { span } => {
                Self::error("unexpected namespace qualifier on entity name")
                    .with_label(span, "remove namespace qualifier")
            }
            LowerError::QualifiedTypeName { span } => {
                Self::error("unexpected namespace qualifier on type name")
                    .with_label(span, "remove namespace qualifier")
            }
            LowerError::InvalidContextType { span } => Self::error("invalid context type")
                .with_label(span, "expected a record type or type reference"),
            LowerError::MissingTypeExpression { span } => {
                Self::error("missing type expression").with_label(span, "expected a type")
            }

            LowerError::InvalidEquals { span } => {
                let suggestion =
                    Suggestion::fix(span.clone(), "==").with_message("use `==` for equality");

                Self::error("invalid operator `=`")
                    .with_label(span, "not a valid operator")
                    .with_suggestion(suggestion)
            }
        }
    }
}
