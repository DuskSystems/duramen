#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use {
    duramen_ast as ast, duramen_cst as cst, duramen_diagnostic as diagnostic,
    duramen_escape as escape, duramen_evaluate as evaluate, duramen_lexer as lexer,
    duramen_lowerer as lowerer, duramen_parser as parser, duramen_runtime as runtime,
    duramen_suggest as suggest, duramen_syntax as syntax, duramen_validate as validate,
};
