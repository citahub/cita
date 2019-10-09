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
    timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "4) check JSON-RPC CORS  ...  "
    # Check JSON-RPC CORS: Access-Control-Allow-Origin should be existed.
    curl -i -X POST -d '{"jsonrpc":"2.0","method":"peerCount","params":[],"id":2}'  "$IP":"$PORT" 2>/dev/null | grep -ic "^access-control-allow-origin: "
    echo "DONE"

    echo -n "5) stop node3, check height growth  ...  "
    bin/cita bebop stop $CHAIN_NAME/3 2>&1
    timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "6) stop node2, check height stopped  ...  "
    bin/cita bebop stop $CHAIN_NAME/2 2>&1
    timeout=$(check_height_stopped 0 30) || (echo "FAILED"
                                             echo "error msg: ${timeout}"
                                             exit 1)
    echo "${timeout}s DONE"

    echo -n "7) start node2, check height growth  ...  "
    bin/cita bebop start $CHAIN_NAME/2 2>&1
    timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "8) start node3, check synch  ...  "
    bin/cita bebop start $CHAIN_NAME/3 2>&1
    timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    echo "${timeout}s DONE"

    echo -n "9) stop all nodes, check height changed after restart  ...  "
    for i in {0..3}; do
        bin/cita bebop restart $CHAIN_NAME/$i 2>&1
    done

    timeout=$(check_height_growth_normal 0 300) || (echo "FAILED"
                                                    echo "error msg: ${timeout}"
                                                    exit 1)
    echo "${timeout}s DONE"

    echo -n "10) stop&clean node3, check height synch after restart  ...  "
    bin/cita bebop stop $CHAIN_NAME/3 2>&1
    bin/cita bebop clean $CHAIN_NAME/3 2>&1
    bin/cita bebop start $CHAIN_NAME/3 2>&1
    timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    echo "${timeout}s DONE"
}

main "$@"
