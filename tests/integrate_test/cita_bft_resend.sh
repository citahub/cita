#!/usr/bin/env bash

# Set bash environment
set -e
if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
fi
# Set CITA system environment
BINARY_DIR=${SOURCE_DIR}/target/install
SUPER_ADMIN="0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"

main() {
    echo "0) Prepare ..."
    # shellcheck source=/dev/null
    source "${SOURCE_DIR}/tests/integrate_test/util.sh"
    cd "${BINARY_DIR}"
    cleanup
    echo "DONE"

    echo "1) Generate CITA configurations ..."
    scripts/create_cita_config.py create \
        --nodes "127.0.0.1:4000" \
        --super_admin "${SUPER_ADMIN}" \
        --chain_name "${CHAIN_NAME}" \
        --timestamp 1524000000
    echo "DONE"

    echo "2) Start CITA components manually"
    bin/cita bebop setup "${CHAIN_NAME}"/0
    bin/cita bebop start "${CHAIN_NAME}"/0
    sleep 3
    bin/cita bebop stop "${CHAIN_NAME}"/0

    cd "${CHAIN_NAME}"/0
    "${BINARY_DIR}"/bin/cita-auth -c auth.toml & auth_pid=$!
    "${BINARY_DIR}"/bin/cita-bft -c consensus.toml -p privkey & bft_pid=$!
    "${BINARY_DIR}"/bin/cita-chain -c chain.toml & chain_pid=$!
    "${BINARY_DIR}"/bin/cita-executor -c executor.toml & executor_pid=$!
    "${BINARY_DIR}"/bin/cita-jsonrpc -c jsonrpc.toml & jsonrpc_pid=$!
    "${BINARY_DIR}"/bin/cita-network -c network.toml & network_pid=$!
    wait_timeout=30
    timeout=$(check_height_growth_normal 0 $wait_timeout) || (echo "FAILED"
                                                              echo "error msg: ${timeout}"
                                                              exit 1)
    cd "${BINARY_DIR}"
    echo "DONE"

    echo "3) Kill cita-executor and cita-chain, and rm -rf data/statedb"
    kill ${executor_pid}
    sleep 3
    kill ${chain_pid}
    rm -rf "${CHAIN_NAME}"/0/data/statedb
    sleep 10
    echo "DONE"

    echo "4) Restart CITA"
    kill ${auth_pid} ${bft_pid} ${jsonrpc_pid} ${network_pid}

    bin/cita bebop start "${CHAIN_NAME}"/0

    wait_timeout=30
    timeout=$(check_height_growth_normal 0 $wait_timeout) || (echo "FAILED"
                                                              echo "error msg: ${timeout}"
                                                              exit 1)
    echo "DONE"

    echo "5) Cleanup ..."
    cleanup
    echo "DONE"
}

main "$@"
