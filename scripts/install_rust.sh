#!/bin/sh
set -x

# 1) install rust
which cargo
if [ $? -ne 0 ]; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2017-08-04
fi;

# 2) install rustfmt
. ${HOME}/.cargo/env
which rustfmt
if [ $? -ne 0 ]; then
   cargo install --force --vers 0.9.0 rustfmt
fi;

