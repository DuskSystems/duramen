#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

cargo fmt --all --check
cargo xtask fixtures
cargo xtask prost
cargo fmt --all
git diff --exit-code
cargo clippy --workspace --all-targets --all-features
typos
zizmor --pedantic .github
cargo deny check
cargo hack build --workspace --feature-powerset --optional-deps
cargo nextest run --workspace --all-features
cargo test --workspace --doc --all-features
