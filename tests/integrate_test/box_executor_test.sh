#!/usr/bin/env bash

set -e

ECONOMICAL_MODEL="0"
if [[ -n "$1" ]] && [ "$1" = "charge" ]; then
    ECONOMICAL_MODEL="1"
fi

if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install


main() {
    echo -n "0) prepare  ...  "
    # shellcheck source=/dev/null
    . ${SOURCE_DIR}/tests/integrate_test/util.sh
    cd "${BINARY_DIR}"
    echo "DONE"

    echo -n "1) generate config  ...  "
    if [ ! -d "resource" ]; then
        mkdir resource
    fi

    autorities=$(xargs echo < "${SOURCE_DIR}"/tests/interfaces/config/authorities | sed "s/ /,/g")
    create_config \
        --contract_arguments "SysConfig.economicalModel=${ECONOMICAL_MODEL}" \
        --contract_arguments "SysConfig.chainId=123" \
        --timestamp 1524000000 \
        --authorities "${autorities}"
    echo -n "DONE"

    echo -n "2) start cita-chain and cita-executor on node0 ...  "
    bin/cita bebop setup "${CHAIN_NAME}"/0

    tnode=$(echo "${CHAIN_NAME}"/0 | sed 's/\//%2f/g')
    curl -i -u guest:guest -H content-type:application/json -XDELETE \
        http://localhost:15672/api/queues/"${tnode}"/consensus > /dev/null

    node_dir="${BINARY_DIR}/${CHAIN_NAME}/0"
    bin/cita-chain -c "${node_dir}"/chain.toml &
    bin/cita-executor -c "${node_dir}"/executor.toml &
    echo -n "DONE"

    echo -n "3) testing ...  "
    AMQP_URL=amqp://guest:guest@localhost/${CHAIN_NAME}/0 \
        timeout 100s "${BINARY_DIR}"/bin/box_executor \
        -m "${SOURCE_DIR}"/tests/interfaces/config/blockchain.yaml
    echo -n "DONE"
}

main "$@"
