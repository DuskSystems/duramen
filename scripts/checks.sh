#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

MAIN=$(git rev-parse --verify origin/main || git rev-parse --verify main)
BASE=$(git merge-base "${MAIN}" HEAD)

committed "${BASE}..HEAD"
typos
tombi lint --error-on-warnings
zizmor --pedantic .github
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
cargo deny check
cargo build --workspace --all-targets --all-features
cargo hack build --workspace --feature-powerset
cargo test --workspace --all-features
cargo test --workspace --all-features --doc
