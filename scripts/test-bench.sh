#!/usr/bin/env bash
#
# Run benchmarks for the workspace
#

set -e

source ci/_

echo --- Running benchmarks
_ cargo bench --workspace --no-run
