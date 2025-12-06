#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

[[ -d /home/trezoa/.ssh ]] || exit 1

if [[ ${#TREZOA_PUBKEYS[@]} -eq 0 ]]; then
  echo "Warning: source trezoa-user-authorized_keys.sh first"
fi

# trezoa-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL testnets
for key in "${TREZOA_PUBKEYS[@]}"; do
  echo "$key" >> /trezoa-scratch/authorized_keys
done

sudo -u trezoa bash -c "
  cat /trezoa-scratch/authorized_keys >> /home/trezoa/.ssh/authorized_keys
"
