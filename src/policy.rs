#![expect(clippy::todo, clippy::missing_errors_doc, reason = "WIP")]

#[cfg(any(feature = "serde", feature = "facet"))]
use alloc::collections::BTreeMap;
#[cfg(any(feature = "serde", feature = "facet"))]
use alloc::string::String;
use alloc::vec::Vec;
use core::error::Error;
use core::fmt;
use core::mem::size_of;

#[cfg(any(feature = "serde", feature = "facet", feature = "prost"))]
use bumpalo::Bump;

use crate::cst::{NodeData, Tree};
use crate::diagnostics::Diagnostic;

pub mod ast;
use ast::{AstNode as _, Policies, Policy as PolicyAst};

#[cfg(any(feature = "serde", feature = "facet", feature = "prost"))]
pub mod est;

mod lexer;
pub use lexer::{PolicyLexer, PolicyToken};

mod parser;
use parser::PolicyParser;

mod syntax;
pub use syntax::PolicySyntax;

const _: () = assert!(
    size_of::<NodeData<PolicySyntax>>() == 32,
    "NodeData must be 32 bytes for cache efficiency"
);

type PolicyTree = Tree<PolicySyntax>;

#[derive(Debug)]
pub struct PolicyErrors;

impl fmt::Display for PolicyErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl Error for PolicyErrors {}

#[derive(Debug)]
pub struct PolicySet<'a> {
    source: &'a str,
    tree: PolicyTree,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> PolicySet<'a> {
    pub(crate) const fn new(
        source: &'a str,
        tree: PolicyTree,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        Self {
            source,
            tree,
            diagnostics,
        }
    }

    #[must_use]
    pub fn parse(source: &'a str) -> Self {
        PolicyParser::new(source).parse()
    }

    #[must_use]
    pub const fn source(&self) -> &'a str {
        self.source
    }

    #[must_use]
    pub const fn tree(&self) -> &PolicyTree {
        &self.tree
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    #[must_use]
    pub fn root(&self) -> Option<Policies<'_>> {
        self.tree.first().and_then(Policies::cast)
    }

    pub fn policies(&self) -> impl Iterator<Item = PolicyAst<'_>> + use<'_> {
        self.root().into_iter().flat_map(|root| root.policies())
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, PolicyErrors> {
        let policy_set = self.to_policy_set_json()?;
        serde_json::to_string(&policy_set).map_err(|_serialize_error| PolicyErrors)
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, PolicyErrors> {
        let policy_set = self.to_policy_set_json()?;
        facet_json::to_string(&policy_set).map_err(|_serialize_error| PolicyErrors)
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, PolicyErrors> {
        use prost::Message as _;
        let policy_set = self.to_policy_set_proto()?;
        Ok(prost::bytes::Bytes::from(policy_set.encode_to_vec()))
    }

    #[cfg(feature = "prost")]
    fn to_policy_set_proto(&self) -> Result<est::proto::PolicySet, PolicyErrors> {
        let Some(root) = self.root() else {
            return Ok(est::proto::PolicySet {
                templates: Vec::new(),
                links: Vec::new(),
            });
        };

        let bump = Bump::new();
        let est_policies = est::convert_policies(&bump, &root, self.source)
            .map_err(|_convert_error| PolicyErrors)?;

        Ok(est::policies_to_proto(&est_policies))
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json_value(&self) -> Result<serde_json::Value, PolicyErrors> {
        let policy_set = self.to_policy_set_json()?;
        serde_json::to_value(&policy_set).map_err(|_serialize_error| PolicyErrors)
    }

    #[cfg(any(feature = "serde", feature = "facet"))]
    fn to_policy_set_json(&self) -> Result<est::json::PolicySetJson, PolicyErrors> {
        let Some(root) = self.root() else {
            return Ok(est::json::PolicySetJson {
                static_policies: BTreeMap::new(),
                templates: BTreeMap::new(),
                template_links: Vec::new(),
            });
        };

        let bump = Bump::new();
        let est_policies = est::convert_policies(&bump, &root, self.source)
            .map_err(|_convert_error| PolicyErrors)?;

        Ok(est::policies_to_json(&est_policies))
    }
}

impl fmt::Display for PolicySet<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.tree.walk() {
            if node.value().is_token() {
                f.write_str(&self.source[node.range()])?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Policy;

impl Policy {
    pub fn parse(_source: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, PolicyErrors> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Template;

impl Template {
    pub fn parse(_source: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn from_serde_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "serde")]
    pub fn to_serde_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn from_facet_json(_json: &str) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "facet")]
    pub fn to_facet_json(&self) -> Result<String, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn from_prost_bytes<B: prost::bytes::Buf>(_bytes: B) -> Result<Self, PolicyErrors> {
        todo!()
    }

    #[cfg(feature = "prost")]
    pub fn to_prost_bytes(&self) -> Result<prost::bytes::Bytes, PolicyErrors> {
        todo!()
    }
}
