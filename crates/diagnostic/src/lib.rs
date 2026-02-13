#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod diagnostic;
pub use diagnostic::{Diagnostic, DiagnosticKind};

mod diagnostics;
pub use diagnostics::{Checkpoint, Diagnostics};

mod suggestion;
pub use suggestion::{Suggestion, SuggestionKind};
