#!/usr/bin/env -S nix develop .#ci-msrv --command bash
set -euxo pipefail

cargo xtask fixtures
cargo build --lib --package duramen --all-features
cargo hack build --lib --package duramen --target thumbv7m-none-eabi --feature-powerset --exclude-features default,std,arbitrary,facet
cargo hack build --lib --package duramen --target wasm32-unknown-unknown --feature-powerset --exclude-features default,std,arbitrary,facet
