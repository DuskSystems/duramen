#!/usr/bin/env bash
set -euo pipefail

git submodule update --init --recursive

tar \
  --extract \
  --gzip \
  --file cedar-integration-tests/corpus-tests.tar.gz \
  --directory cedar-integration-tests
