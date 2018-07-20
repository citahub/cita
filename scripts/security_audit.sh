#!/usr/bin/env bash

if [[ "$(git log -n 1 --format="%s")" =~ \[skip\ audit\] ]]; then
    echo "[Info_] Skip Security Audit."
    exit 0
fi

which cargo-audit
ret=$?
if [ "${ret}" -ne 0 ]; then
    echo "[Info_] Install Security Audit."
    cargo install cargo-audit
fi

echo "[Info_] Run Security Audit."
cargo audit
