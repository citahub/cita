#!/bin/bash
set -e

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install

cd ${BINARY_DIR} \
&& ./scripts/create_cita_config.py create \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000" \
&& python3 ${SOURCE_DIR}/tests/compatibility/check_genesis.py \
    --genesis test-chain/0/genesis.json \
&& rm -rf test-chain genesis
