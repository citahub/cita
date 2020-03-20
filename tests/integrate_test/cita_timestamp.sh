#!/bin/bash

set -e
if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install
IP="127.0.0.1"
PORT="1337"

main() {
    local node3_proposal

    echo -n "0) prepare  ...  "
    # shellcheck source=/dev/null
    . "${SOURCE_DIR}"/tests/integrate_test/util.sh
    cd "${BINARY_DIR}"
    echo "DONE"

    echo -n "1) generate config ... "
    create_config "$1"
    echo "DONE"

    echo -n "2) start nodes  ...  "
    start_nodes
    echo "DONE"

    echo -n "3) check height growth normal  ...  "
    timeout=$(check_height_growth_normal 0 10) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "4) restart node3, turn on timestamp modify switch, check height growth ... "
    export MODIFY_TIME=6000
    bin/cita bebop restart "$CHAIN_NAME"/3 2>&1
    timeout=$(check_height_growth_normal 0 30) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "5) check node3 proposer not appear in next 8 blocks ... "
    node3_proposal=$(cat "${BINARY_DIR}"/"${CHAIN_NAME}"/3/address)
    timeout=$(check_proposer 0 "$node3_proposal" 8 60) || (echo "FAILED"
                                                            echo "error msg: ${timeout}"
                                                            exit 1)
    echo "${timeout}"

    echo -n "6) restart node2, turn on timestamp modify swithc, check height growth ... "
    bin/cita bebop restart "$CHAIN_NAME"/2 2>&1
    timeout=$(check_height_growth_normal 0 30) || (echo "FAILED"
                                                     echo "error msg: ${timeout}"
                                                     exit 1)
    echo "${timeout}s DONE"
}

main "$@"
