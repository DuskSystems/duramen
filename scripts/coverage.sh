#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

cargo xtask fixtures
cargo llvm-cov --workspace --all-features --doctests --html
