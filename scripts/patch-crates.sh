# source this file

update_trezoa_dependencies() {
  declare project_root="$1"
  declare trezoa_ver="$2"
  declare tomls=()
  while IFS='' read -r line; do tomls+=("$line"); done < <(find "$project_root" -name Cargo.toml)

  sed -i -e "s#\(trezoa-program = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-program = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-program-test = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-program-test = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-sdk = \"\).*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-sdk = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-client = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-client = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-cli-config = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-cli-config = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-clap-utils = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-clap-utils = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-account-decoder = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-account-decoder = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-faucet = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-faucet = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-zk-token-sdk = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(trezoa-zk-token-sdk = { version = \"\)[^\"]*\(\"\)#\1=$trezoa_ver\2#g" "${tomls[@]}" || return $?
}

patch_crates_io_trezoa() {
  declare Cargo_toml="$1"
  declare trezoa_dir="$2"
  cat >> "$Cargo_toml" <<EOF
[patch.crates-io]
EOF
patch_crates_io_trezoa_no_header "$Cargo_toml" "$trezoa_dir"
}

patch_crates_io_trezoa_no_header() {
  declare Cargo_toml="$1"
  declare trezoa_dir="$2"
  cat >> "$Cargo_toml" <<EOF
trezoa-account-decoder = { path = "$trezoa_dir/account-decoder" }
trezoa-clap-utils = { path = "$trezoa_dir/clap-utils" }
trezoa-client = { path = "$trezoa_dir/client" }
trezoa-cli-config = { path = "$trezoa_dir/cli-config" }
trezoa-program = { path = "$trezoa_dir/sdk/program" }
trezoa-program-test = { path = "$trezoa_dir/program-test" }
trezoa-sdk = { path = "$trezoa_dir/sdk" }
trezoa-faucet = { path = "$trezoa_dir/faucet" }
trezoa-zk-token-sdk = { path = "$trezoa_dir/zk-token-sdk" }
EOF
}
