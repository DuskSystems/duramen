use core::error::Error;
use std::fs;
use std::path::Path;

pub fn run() -> Result<(), Box<dyn Error>> {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = Path::new(manifest)
        .parent()
        .ok_or("Failed to find project root")?;

    let protobuf = root.join("protobuf");
    let out = root.join("src/policy/est/proto");

    fs::create_dir_all(&out)?;

    prost_build::Config::new()
        .out_dir(&out)
        .btree_map(["."])
        .compile_protos(&[protobuf.join("core.proto")], &[&protobuf])?;

    fs::rename(out.join("cedar_policy_core.rs"), out.join("types.rs"))?;

    Ok(())
}
