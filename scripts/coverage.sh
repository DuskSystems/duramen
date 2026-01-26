#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

cargo llvm-cov --all-features --doctests --codecov --output-path codecov.json
