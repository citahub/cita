#!/usr/bin/env bash

if [[ "$(git log -n 1 --format="%s")" =~ \[skip\ audit\] ]]; then
    echo "[Info_] Skip Security Audit."
    exit 0
fi

which cargo-audit
ret=$?
if [ "${ret}" -ne 0 ]; then
    echo "[Info_] Install Security Audit."
    # TODO: remove this version restriction after upgrade rustc version to 1.36+
    cargo install cargo-audit --version 0.7.0
fi

echo "[Info_] Run Security Audit."
# TODO: Remove these ignore crates
# RUSTSEC-2016-0005: rust-crypto
# RUSTSEC-2019-0026: sodiumoxide
# RUSTSEC-2019-0027: libsecp256k1
cargo audit --ignore RUSTSEC-2016-0005 --ignore RUSTSEC-2019-0026 --ignore RUSTSEC-2019-0027
