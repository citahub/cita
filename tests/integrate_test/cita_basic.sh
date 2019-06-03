#!/bin/bash

main() {
    set -e
    if [[ $(uname) == 'Darwin' ]]
    then
        SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
    else
        SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
    fi
    BINARY_DIR=${SOURCE_DIR}/target/install

    echo -n "0) prepare  ...  "
    # shellcheck source=/dev/null
    . "${SOURCE_DIR}"/tests/integrate_test/util.sh
    cd "${BINARY_DIR}"
    echo "DONE"

    echo -n "1) generate config  ...  "
    if [[ -n "$1" ]]; then
        ./scripts/create_cita_config.py create \
            --chain_name "node" \
            --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
            --"$1"\
            --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
            > /dev/null 2>&1
    else
        ./scripts/create_cita_config.py create \
            --chain_name "node" \
            --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
            --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
            > /dev/null 2>&1
    fi
    echo "DONE"

    echo -n "2) start nodes  ...  "
    start_nodes
    echo "DONE"

    echo -n "3) check height growth normal  ...  "
    timeout=$(check_height_growth_normal 0 60)||(echo "FAILED"
                                                echo "error msg: ${timeout}"
                                                exit 1)
    echo "${timeout}s DONE"

    echo -n "4) stop node3, check height growth  ...  "
    bin/cita bebop stop node/3 > /dev/null
    timeout=$(check_height_growth_normal 0 30) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "5) stop node2, check height stopped  ...  "
    bin/cita bebop stop node/2 > /dev/null
    timeout=$(check_height_stopped 0 30) || (echo "FAILED"
                                             echo "error msg: ${timeout}"
                                             exit 1)
    echo "${timeout}s DONE"

    echo -n "6) start node2, check height growth  ...  "
    bin/cita bebop start node/2 trace > /dev/null &
    timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                                   echo "error msg: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "7) start node3, check synch  ...  "
    bin/cita bebop start node/3 > /dev/null &
    timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    echo "${timeout}s DONE"

    echo -n "8) stop all nodes, check height changed after restart  ...  "
    for i in {0..3}; do
        bin/cita bebop stop node/$i > /dev/null
    done
    for i in {0..3}; do
        bin/cita bebop start node/$i > /dev/null &
    done

    timeout=$(check_height_growth_normal 0 300) || (echo "FAILED"
                                                    echo "error msg: ${timeout}"
                                                    exit 1)
    echo "${timeout}s DONE"

    echo -n "9) stop&clean node3, check height synch after restart  ...  "
    bin/cita bebop stop node/3 > /dev/null
    bin/cita bebop clean node/3 > /dev/null
    bin/cita bebop start node/3 > /dev/null &
    timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
    echo "${timeout}s DONE"
}

main "$@"
