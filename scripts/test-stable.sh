#!/usr/bin/env bash

set -e

here="$(dirname "$0")"

# shellcheck source=scripts/read-cargo-variable.sh
source "$here"/read-cargo-variable.sh

cargo="$(readCargoVariable name Cargo.toml)"

if [[ -z "$cargo" ]]; then
  cargo=cargo
fi

exec "$cargo" test --workspace --exclude trezoa-local-cluster "$@"
