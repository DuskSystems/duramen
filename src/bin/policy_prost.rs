use core::error::Error;
use core::hint::black_box;
use std::fs;
use std::path::Path;

use duramen::policy::PolicySet;

fn main() -> Result<(), Box<dyn Error>> {
    let corpus = Path::new("cedar-integration-tests/corpus-tests");

    for entry in fs::read_dir(corpus)? {
        let path = entry?.path();
        if path
            .extension()
            .is_some_and(|extension| extension == "cedar")
        {
            let source = fs::read_to_string(&path)?;
            let parsed = PolicySet::parse(&source);
            if !parsed.has_errors() {
                let bytes = parsed.to_prost_bytes()?;
                black_box(bytes);
            }
        }
    }

    Ok(())
}
