#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

cargo fmt --all --check
cargo clippy --all-targets --all-features
typos
tombi lint --error-on-warnings
zizmor --pedantic .github
cargo deny check
cargo build --all-targets --all-features
cargo hack build --feature-powerset
cargo test --all-features
cargo test --all-features --doc
