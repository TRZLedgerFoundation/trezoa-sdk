#!/usr/bin/env bash

set -e
here=$(dirname "$0")

# shellcheck source=.buildkite/scripts/common.sh
source "$here"/common.sh

agent="${1-trezoa}"

group "downstream projects" \
#  '{ "name": "tpl", "command": "./ci/downstream-projects/run-tpl.sh", "timeout_in_minutes": 30, "agent": "'"$agent"'" }'
#  '{ "name": "openbook-dex", "command": "./ci/downstream-projects/run-openbook-dex.sh", "timeout_in_minutes": 30, "agent": "'"$agent"'" }' \
