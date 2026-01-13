mod fixtures;
mod prost;

use core::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("fixtures") => fixtures::run()?,
        Some("prost") => prost::run()?,
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }

    Ok(())
}

#[expect(clippy::print_stderr, reason = "Help message")]
fn print_usage() {
    eprintln!("Usage: cargo xtask <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  fixtures  Initialize test fixtures");
    eprintln!("  prost     Generate protobuf types from core.proto");
}
