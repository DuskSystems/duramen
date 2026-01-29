use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;

use duramen_cst::{PolicyBuilder, PolicyTree};
use duramen_diagnostics::{Diagnostic, Severity};
use duramen_lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub struct PolicyParseResult {
    pub tree: PolicyTree,
    pub diagnostics: Vec<Diagnostic>,
}

impl PolicyParseResult {
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| matches!(diagnostic.severity(), Severity::Error))
    }

    #[must_use]
    pub fn print(&self, source: &str) -> String {
        let mut output = String::new();
        for node in self.tree.children() {
            let span = node.span();
            if let Some(text) = source.get(span.start as usize..span.end as usize) {
                output.push_str(text);
            }
        }

        output
    }
}

pub struct PolicyParser<'a> {
    pub lexer: Lexer<'a>,
    pub current: Token,
    pub position: usize,
    pub builder: PolicyBuilder,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> PolicyParser<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next().unwrap_or(Token::new(TokenKind::Unknown, 0));

        Self {
            lexer,
            current,
            position: 0,
            builder: PolicyBuilder::new(),
            diagnostics: Vec::new(),
        }
    }

    // TODO: Infallible?
    #[expect(clippy::unused_self, clippy::missing_errors_doc, reason = "TODO")]
    pub fn parse(&self) -> Result<PolicyParseResult, Box<dyn Error>> {
        Err("TODO".into())
    }
}
