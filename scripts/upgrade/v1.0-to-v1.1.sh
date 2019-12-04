#!/bin/bash

# This is a updrade helper for CITA from version 1.0.0 to version 1.1.0.
#
# 1. why this bash needed?
#
# In cita 1.1.0, a interface named `setBlockInterval` was added in system contract.
# So, if you were using cita version 1.1 below and wanted to upgrade to version 1.1, this file is prepared for you.
#
# II. How to use this bash?
#
#   1. Download the cita version 1.1 release package from https://github.com/citahub/cita/releases
#   2. Extract the package and copy directory bin/ and scripts/ to corresponding dir under your nodes.
#   3. Next, upgrade your node use this helper. Tap the command below, remember to update the admin privkey
#       and node url params.

#          ./scripts/upgrade/v1.0-to-v1.1.sh \
#                0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
#                http://127.0.0.1:1337  \
set -e

if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/..)
fi

if [ "$1" = "help" ]; then
    echo "Admin private key, url as the params.
        For example: \\
        bin/cita scripts/upgrade/v1.0-to-v1.1.sh \\
            0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \\
            http://127.0.0.1:1337"
    exit 0
fi

echo "==> Prepare environment"
rm -rf tmp/
rm -f "${SOURCE_DIR}"/genesis.json

echo "==> Create temp chain and get new genesis.json"
python3 scripts/create_cita_config.py create \
    --chain_name tmp \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000"

cp tmp/0/genesis.json scripts/

echo "==> Send tx to amend code"
cd ./scripts/txtool/txtool
python3 "${SOURCE_DIR}"/upgrade/v1.0-to-v1.1.py --privkey "$1" --url "$2"
cd -

echo "==> Clean temp data"
rm -rf tmp/
rm -f "${SOURCE_DIR}"/genesis.json
