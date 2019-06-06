#!/usr/bin/env bash

# Set bash environment
if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
fi

set -e
# Set CITA system environment
BINARY_DIR=${SOURCE_DIR}/target/install

amend_chain_name() {
    python3 make_tx.py \
    --code 0xffffffffffffffffffffffffffffffffff0200000000000000000000000000000000000000000000000000000000000000000027306573742d636861696e00000000000000000000000000000000000000000014 \
    --to 0xffffffffffffffffffffffffffffffffff010002 \
    --value 3 \
    --privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 > /dev/null

    python3 send_tx.py > /dev/null

    sleep 10

    curl -X POST --data '{"jsonrpc":"2.0","method":"getMetaData","params":["latest"],"id":1}' 127.0.0.1:1337 \
     | grep "\"chainName\"\:\"0est-chain\""
}

amend_abi() {
    # set abi of 0xffffffffffffffffffffffffffffffffff020000 as "amendabitest"
    python3 make_tx.py \
    --code 0xffffffffffffffffffffffffffffffffff0200000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000c616d656e64616269746573740000000000000000000000000000000000000000 \
    --to 0xffffffffffffffffffffffffffffffffff010002 \
    --value 1 \
    --privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 > /dev/null

    python3 send_tx.py > /dev/null

    sleep 10

    # ascii of "amendabitest"
    curl -X POST --data '{"jsonrpc":"2.0","method":"getAbi","params":["0xffffffffffffffffffffffffffffffffff020000", "latest"],"id":1}' 127.0.0.1:1337 \
     | grep "616d656e6461626974657374"
}

amend_balance() {
    # set balance of 0xffffffffffffffffffffffffffffffffff020000 as 1234(0x4d2)
    python3 make_tx.py \
    --code 0xffffffffffffffffffffffffffffffffff02000000000000000000000000000000000000000000000000000000000000000004d2 \
    --to 0xffffffffffffffffffffffffffffffffff010002 \
    --value 5 \
    --privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 > /dev/null

    python3 send_tx.py > /dev/null

    sleep 10

    # hex of 1234
    curl -X POST --data '{"jsonrpc":"2.0","method":"getBalance","params":["0xffffffffffffffffffffffffffffffffff020000", "latest"],"id":1}' 127.0.0.1:1337 \
     | grep "0x4d2"
}

amend_code() {
    # set code of 0xffffffffffffffffffffffffffffffffff020004 as "deadbeef"
    python3 make_tx.py \
    --code 0xffffffffffffffffffffffffffffffffff020004deadbeef \
    --to 0xffffffffffffffffffffffffffffffffff010002 \
    --value 2 \
    --privkey 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 > /dev/null

    python3 send_tx.py > /dev/null

    sleep 10

    # hex of 1234
    curl -X POST --data '{"jsonrpc":"2.0","method":"getCode","params":["0xffffffffffffffffffffffffffffffffff020004", "latest"],"id":1}' 127.0.0.1:1337 \
     | grep "0xdeadbeef"
}

main() {
    echo "0) Prepare ..."
    # shellcheck source=/dev/null
    source "${SOURCE_DIR}/tests/integrate_test/util.sh"
    cd "${BINARY_DIR}"
    echo "DONE"

    echo "1) Generate configurations ..."
    create_config
    echo "DONE"

    echo "2) Run nodes"
    start_nodes
    echo "DONE"

    echo "3) Check node grow up ..."
    timeout=$(check_height_growth_normal 0 30) || (echo "FAILED"
                                                  echo "error msg: ${timeout}"
                                                  exit 1)
    echo "${timeout}s DONE"

    cd "${BINARY_DIR}"/scripts/txtool/txtool
    echo "4) Amend chain name ..."
    amend_chain_name
    echo "DONE"
    echo "5) Amend abi ..."
    amend_abi
    echo "DONE"
    echo "6) Amend balance ..."
    amend_balance
    echo "DONE"
    echo "7) Amend code ..."
    amend_code
    echo "DONE"
}

main "$@"
