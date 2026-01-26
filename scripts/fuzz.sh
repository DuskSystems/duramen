#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

TIME="${1:-30}"

cargo fuzz build

for TARGET in $(cargo fuzz list); do
  cargo fuzz run "${TARGET}" \
    -- \
    -timeout=0.1 \
    -max_total_time="${TIME}" \
    -fork="$(nproc)"
done
