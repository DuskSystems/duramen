extern crate alloc;

mod ungram;

use core::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("ungram") => match args.get(2).map(String::as_str) {
            Some("policy") => ungram::policy::generate()?,
            Some("schema") => ungram::schema::generate()?,
            Some(_) | None => return Err("Expected one of target: policy, schema".into()),
        },
        Some(_) | None => return Err("Expected one of command: ungram".into()),
    }

    Ok(())
}
