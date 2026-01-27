#!/usr/bin/env -S nix develop .#ci-msrv --command bash
set -euxo pipefail

cargo build --workspace --lib --all-features
cargo hack build --workspace --lib --target thumbv7m-none-eabi --feature-powerset --exclude-features default,std
cargo hack build --workspace --lib --target wasm32-unknown-unknown --feature-powerset --exclude-features default,std
