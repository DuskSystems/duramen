extern crate alloc;

mod fixtures;
mod ungram;

use core::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("ungram") => {
            ungram::policy::generate()?;
            ungram::schema::generate()?;
        }
        Some("fixtures") => {
            fixtures::run()?;
        }
        Some(_) | None => {
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
    eprintln!("  ungram    Generate code from grammar files");
    eprintln!("  fixtures  Initialize test fixtures");
}
