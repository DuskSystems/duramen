#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

cargo xtask fixtures
cargo bench --all-features
