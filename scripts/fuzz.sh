#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

TIME="${1:-30}"

rm -rf fuzz/artifacts
rm -rf fuzz/corpus

cargo fuzz build

for TARGET in $(cargo fuzz list); do
  # Timeout: 100 µs
  cargo fuzz run "${TARGET}" \
    -- \
    -timeout=0.0001 \
    -max_total_time="${TIME}" \
    -fork="$(nproc)"
done
