use core::error::Error;

use super::shared::Config;

const CONFIG: Config = Config {
    grammar: "grammar/policy.ungram",
    output: "src/policy/syntax.rs",

    name: "PolicyKind",
    comment: "Syntax kinds for Cedar policies.",

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
        ("==", "EQ2"),
        ("!=", "NEQ"),
        ("=", "EQ"),
        ("<", "LT"),
        ("<=", "LTEQ"),
        (">", "GT"),
        (">=", "GTEQ"),
        ("&&", "AMP2"),
        ("||", "PIPE2"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("%", "PERCENT"),
        ("!", "BANG"),
    ],
    tokens: &[
        ("@ident", "IDENT"),
        ("@int", "INT"),
        ("@string", "STRING"),
        ("@slot", "SLOT"),
    ],
    literals: &["INT", "STRING"],
};

pub fn generate() -> Result<(), Box<dyn Error>> {
    super::shared::generate(&CONFIG)
}
