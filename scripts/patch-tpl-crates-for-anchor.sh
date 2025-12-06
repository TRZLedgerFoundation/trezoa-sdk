tpl_memo_version=
tpl_token_version=
tpl_token_2022_version=
tpl_tlv_account_resolution_verison=
tpl_transfer_hook_interface_version=

get_tpl_versions() {
    declare tpl_dir="$1"
    tpl_memo_version=$(readCargoVariable version "$tpl_dir/memo/program/Cargo.toml")
    tpl_token_version=$(readCargoVariable version "$tpl_dir/token/program/Cargo.toml")
    tpl_token_2022_version=$(readCargoVariable version "$tpl_dir/token/program-2022/Cargo.toml"| head -c1) # only use the major version for convenience
    tpl_tlv_account_resolution_verison=$(readCargoVariable version "$tpl_dir/libraries/tlv-account-resolution/Cargo.toml")
    tpl_transfer_hook_interface_version=$(readCargoVariable version "$tpl_dir/token/transfer-hook/interface/Cargo.toml")
}

patch_tpl_crates() {
    declare project_root="$1"
    declare Cargo_toml="$2"
    declare tpl_dir="$3"
    update_tpl_dependencies "$project_root"
    patch_crates_io "$Cargo_toml" "$tpl_dir"
}

update_tpl_dependencies() {
    declare project_root="$1"
    declare tomls=()
    while IFS='' read -r line; do tomls+=("$line"); done < <(find "$project_root" -name Cargo.toml)

    sed -i -e "s#\(tpl-memo = \"\)[^\"]*\(\"\)#\1$tpl_memo_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-memo = { version = \"\)[^\"]*\(\"\)#\1$tpl_memo_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-token = \"\)[^\"]*\(\"\)#\1$tpl_token_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-token = { version = \"\)[^\"]*\(\"\)#\1$tpl_token_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-token-2022 = \"\).*\(\"\)#\1$tpl_token_2022_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-token-2022 = { version = \"\)[^\"]*\(\"\)#\1$tpl_token_2022_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-tlv-account-resolution = \"\)[^\"]*\(\"\)#\1=$tpl_tlv_account_resolution_verison\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-tlv-account-resolution = { version = \"\)[^\"]*\(\"\)#\1=$tpl_tlv_account_resolution_verison\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-transfer-hook-interface = \"\)[^\"]*\(\"\)#\1=$tpl_transfer_hook_interface_version\2#g" "${tomls[@]}" || return $?
    sed -i -e "s#\(tpl-transfer-hook-interface = { version = \"\)[^\"]*\(\"\)#\1=$tpl_transfer_hook_interface_version\2#g" "${tomls[@]}" || return $?

    # patch ahash. This is super brittle; putting here for convenience, since we are already iterating through the tomls
    ahash_minor_version="0.8"
    sed -i -e "s#\(ahash = \"\)[^\"]*\(\"\)#\1$ahash_minor_version\2#g" "${tomls[@]}" || return $?
}

patch_crates_io() {
    declare Cargo_toml="$1"
    declare tpl_dir="$2"
    cat >> "$Cargo_toml" <<EOF
    tpl-memo = { path = "$tpl_dir/memo/program" }
    tpl-token = { path = "$tpl_dir/token/program" }
    tpl-token-2022 = { path = "$tpl_dir/token/program-2022" }
    tpl-tlv-account-resolution = { path = "$tpl_dir/libraries/tlv-account-resolution" }
    tpl-transfer-hook-interface = { path = "$tpl_dir/token/transfer-hook/interface" }
EOF
}
