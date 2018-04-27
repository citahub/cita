#!/usr/bin/env bash

set -e

function replace_default_feature () {
    local workspacedir="${1}"
    local old_feature="${2}"
    local new_feature="${3}"
    if [ "${old_feature}" = "${new_feature}" ]; then
        return
    fi
    local before_feature='[ \t]*default[ \t]*=[ \t]*\[.*\"'
    local after_feature='\".*'
    find "${workspacedir}" -mindepth 2 -name "Cargo.toml" \
            | xargs grep -l "^${before_feature}${old_feature}${after_feature}" \
            | while read cargotoml; do
        if [ -f "${cargotoml}" ]; then
            echo "[Info ] Replace [${old_feature}] by [${new_feature}] for [${cargotoml}] ..."
            sed -i "s/\(${before_feature}\)${old_feature}\(${after_feature}\)\$/\1${new_feature}\2/" "${cargotoml}"
        else
            echo "[Error] [${cargotoml}] is not a file."
        fi
    done
}

replace_default_feature "$@"
