use core::error::Error;

use super::shared::Config;

const CONFIG: Config = Config {
    grammar: "grammar/schema.ungram",
    output: "src/schema/syntax.rs",

    name: "SchemaKind",
    comment: "Syntax kinds for Cedar schemas.",

    punctuation: &[
        ("@", "AT"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_BRACE"),
        ("}", "R_BRACE"),
        ("[", "L_BRACKET"),
        ("]", "R_BRACKET"),
        (";", "SEMI"),
        (":", "COLON"),
        ("::", "COLON2"),
        (",", "COMMA"),
        (".", "DOT"),
        ("?", "QUESTION"),
        ("=", "EQ"),
        ("<", "LT"),
        (">", "GT"),
    ],
    tokens: &[("@ident", "IDENT"), ("@string", "STRING")],
    literals: &["STRING"],
};

pub fn generate() -> Result<(), Box<dyn Error>> {
    super::shared::generate(&CONFIG)
}
