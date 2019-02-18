#!/bin/bash
set -e

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/..)
fi

if [ "$1" = "help" ]; then
    echo "Admin private key, chain id, version, url  as the params.
        For example: \\
        ./env.sh scripts/amend_system_contracts.sh \\
        0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \\
        1 \\
        1 \\
        http://127.0.0.1:1337"
    exit 0
fi

# Clean tmp files
rm -rf ${SOURCE_DIR}/tmp
rm -f ${SOURCE_DIR}/scripts/genesis.json

# Just get the genensis.json
scripts/create_cita_config.py create \
    --chain_name tmp \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000" \
&& cp tmp/0/genesis.json scripts/ \
&& cd ./scripts/txtool/txtool \
&& python3 ${SOURCE_DIR}/scripts/amend_system_contracts.py \
    --privkey "$1" \
    --chain_id "$2" \
    --version "$3" \
    --url "$4" \
&& rm -rf ${SOURCE_DIR}/tmp \
&& rm -f ${SOURCE_DIR}/scripts/genesis.json
