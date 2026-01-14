#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

cargo doc --workspace --all-features --no-deps --document-private-items
rm target/doc/.lock
echo '<meta http-equiv="refresh" content="0; url=duramen/index.html">' > target/doc/index.html
