#!/usr/bin/env bash
#
# Run code coverage for the workspace
#

set -e

source ci/_

if ! command -v grcov > /dev/null; then
  echo "Please install grcov:"
  echo "  cargo install grcov"
  exit 1
fi

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests"

echo --- cleaning up test coverage files
find . -name "*.profraw" -delete || true

echo --- running tests with coverage enabled
cargo +nightly test --workspace

echo --- generating code coverage report
mkdir -p target/cov
grcov . \
  --binary-path target/debug/ \
  --source-dir . \
  --output-type html \
  --output-path target/cov/lcov-local/ \
  --ignore "/*" \
  --ignore "**/tests/*" \
  --ignore "**/benches/*" \
  --ignore "**/build.rs" \
  --ignore "**/target/*"

echo --- open target/cov/lcov-local/index.html to view the report
