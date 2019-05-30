#!/usr/bin/env bash

# Set bash environment
set -e
SOURCE_DIR=$(cd $(dirname "$0")/../..; pwd)
if [[ `uname` == 'Darwin' ]]
then
    SED="gsed"
else
    SED="sed"
fi

# Set CITA system environment
BINARY_DIR=${SOURCE_DIR}/target/install
CHAIN_NAME="test-chain"
SUPER_ADMIN="0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"

################################################################################
echo "0) Prepare ..."
source "${SOURCE_DIR}/tests/integrate_test/util.sh"
cd "${BINARY_DIR}"
echo "DONE"

################################################################################
echo "1) Generate CITA configurations ..."
${BINARY_DIR}/scripts/create_cita_config.py create \
    --nodes "127.0.0.1:4000" \
    --super_admin "${SUPER_ADMIN}"
echo "DONE"

################################################################################
echo "2) Run node-0"
${BINARY_DIR}/bin/cita bebop setup ${CHAIN_NAME}/0 > /dev/null
${BINARY_DIR}/bin/cita bebop start ${CHAIN_NAME}/0 trace
echo "DONE"

sleep 10

################################################################################
echo "3) Check node grow up ..."
echo "chech_height_growth_normal 0 ..."
timeout=`check_height_growth_normal 0 15`||(echo "FAILED"
                                              echo "error msg: ${timeout}"
                                              exit 1)
echo "${timeout}s DONE"

################################################################################
echo "4) Amend chain name ..."
cd ${BINARY_DIR}/scripts/txtool/txtool

curl -X POST --data '{"jsonrpc":"2.0","method":"getMetaData","params":["latest"],"id":1}' 127.0.0.1:1337 \
 | grep "\"chainName\"\:\"test-chain\""

python3 make_tx.py \
--code 0xffffffffffffffffffffffffffffffffff0200000000000000000000000000000000000000000000000000000000000000000027306573742d636861696e00000000000000000000000000000000000000000014 \
--to 0xffffffffffffffffffffffffffffffffff010002 \
--value 3 \
--privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6

python3 send_tx.py

sleep 10

curl -X POST --data '{"jsonrpc":"2.0","method":"getMetaData","params":["latest"],"id":1}' 127.0.0.1:1337 \
 | grep "\"chainName\"\:\"0est-chain\""

################################################################################
echo "5) Amend abi ..."
cd ${BINARY_DIR}/scripts/txtool/txtool

# set abi of 0xffffffffffffffffffffffffffffffffff020000 as "amendabitest"
python3 make_tx.py \
--code 0xffffffffffffffffffffffffffffffffff0200000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000c616d656e64616269746573740000000000000000000000000000000000000000 \
--to 0xffffffffffffffffffffffffffffffffff010002 \
--value 1 \
--privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6

python3 send_tx.py

sleep 10

# ascii of "amendabitest"
curl -X POST --data '{"jsonrpc":"2.0","method":"getAbi","params":["0xffffffffffffffffffffffffffffffffff020000", "latest"],"id":1}' 127.0.0.1:1337 \
 | grep "616d656e6461626974657374"

################################################################################
echo "6) Amend balance ..."
cd ${BINARY_DIR}/scripts/txtool/txtool

# set balance of 0xffffffffffffffffffffffffffffffffff020000 as 1234(0x4d2)
python3 make_tx.py \
--code 0xffffffffffffffffffffffffffffffffff02000000000000000000000000000000000000000000000000000000000000000004d2 \
--to 0xffffffffffffffffffffffffffffffffff010002 \
--value 5 \
--privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6

python3 send_tx.py

sleep 10

# hex of 1234
curl -X POST --data '{"jsonrpc":"2.0","method":"getBalance","params":["0xffffffffffffffffffffffffffffffffff020000", "latest"],"id":1}' 127.0.0.1:1337 \
 | grep "0x4d2"

################################################################################
echo "7) Amend code ..."
cd ${BINARY_DIR}/scripts/txtool/txtool

# set code of 0xffffffffffffffffffffffffffffffffff020004 as "deadbeef"
python3 make_tx.py \
--code 0xffffffffffffffffffffffffffffffffff020004deadbeef \
--to 0xffffffffffffffffffffffffffffffffff010002 \
--value 2 \
--privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6

python3 send_tx.py

sleep 10

# hex of 1234
curl -X POST --data '{"jsonrpc":"2.0","method":"getCode","params":["0xffffffffffffffffffffffffffffffffff020004", "latest"],"id":1}' 127.0.0.1:1337 \
 | grep "0xdeadbeef"
