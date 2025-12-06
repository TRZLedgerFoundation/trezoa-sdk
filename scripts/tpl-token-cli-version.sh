# populate this on the stable branch
tplTokenCliVersion=

maybeTplTokenCliVersionArg=
if [[ -n "$tplTokenCliVersion" ]]; then
    # shellcheck disable=SC2034
    maybeTplTokenCliVersionArg="--version $tplTokenCliVersion"
fi
