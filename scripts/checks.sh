#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
typos
tombi lint --error-on-warnings
zizmor --pedantic .github
cargo deny check
cargo build --workspace --all-targets --all-features
cargo hack build --workspace --feature-powerset
cargo test --workspace --all-features
cargo test --workspace --all-features --doc
