#!/usr/bin/env bash

if [[ "$(git log -n 1 --format="%s")" =~ \[skip\ audit\] ]]; then
    echo "[Info_] Skip Security Audit."
    exit 0
fi

command -v cargo-audit
ret=$?
if [ "${ret}" -ne 0 ]; then
    echo "[Info_] Install Security Audit."
    cargo install cargo-audit
fi

echo "[Info_] Run Security Audit."
# TODO: Remove these ignore crates
# RUSTSEC-2016-0005: rust-crypto
# RUSTSEC-2019-0027: libsecp256k1
# RUSTSEC-2019-0031: spin < ring < tentacle-secio
cargo audit --ignore RUSTSEC-2016-0005 --ignore RUSTSEC-2019-0027 --ignore RUSTSEC-2019-0031
