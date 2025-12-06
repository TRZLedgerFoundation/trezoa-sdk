#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

# shellcheck source=net/scripts/trezoa-user-authorized_keys.sh
source trezoa-user-authorized_keys.sh

# trezoa-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL datacenter nodes.
for i in "${!TREZOA_USERS[@]}"; do
  echo "environment=\"TREZOA_USER=${TREZOA_USERS[i]}\" ${TREZOA_PUBKEYS[i]}"
done

