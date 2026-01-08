use core::error::Error;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn run() -> Result<(), Box<dyn Error>> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = Path::new(manifest)
        .parent()
        .ok_or("Failed to find project root")?;

    let submodule = Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .current_dir(root)
        .status()?;

    if !submodule.success() {
        return Err("Failed to initialize submodule".into());
    }

    let submodule = root.join("cedar-integration-tests");

    let archive = submodule.join("corpus-tests.tar.gz");
    let tests = submodule.join("corpus-tests");

    if archive.exists() && !tests.exists() {
        let extract = Command::new("tar")
            .args(["xzf", "corpus-tests.tar.gz"])
            .current_dir(&submodule)
            .status()?;

        if !extract.success() {
            return Err("Failed to extract corpus tests".into());
        }
    }

    Ok(())
}
