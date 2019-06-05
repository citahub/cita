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

main() {
    echo "0) Prepare ..."
    # shellcheck source=/dev/null
    source "${SOURCE_DIR}/tests/integrate_test/util.sh"
    cd "${BINARY_DIR}"
    echo "DONE"

    echo "1) Generate configurations ..."
    create_config --timestamp 1524000000
    echo "DONE"

    echo "2) Run node-0, node-1, node-2"
    for id in {0,1,2}; do
        "${BINARY_DIR}"/bin/cita bebop setup "${CHAIN_NAME}"/${id} > /dev/null
    done
    for id in {0,1,2}; do
        "${BINARY_DIR}"/bin/cita bebop start "${CHAIN_NAME}"/${id}
    done
    echo "DONE"

    echo "3) Check all nodes grow up ..."
    for id in {0..2}; do
        echo "chech_height_growth_normal $id ..."
        timeout=$(check_height_growth_normal $id 60) || (echo "FAILED"
                                                         echo "error msg: ${timeout}"
                                                         exit 1)
    done
    echo "${timeout}s DONE"

    echo "4) Stop node-1 and node-2, so that node-0 cannot grow up via cita-consensus and cita-sync mechanisms"
    "${BINARY_DIR}"/bin/cita bebop stop "${CHAIN_NAME}"/1
    "${BINARY_DIR}"/bin/cita bebop stop "${CHAIN_NAME}"/2

    # Ensure that the current round of BFT has been finished. So that node-0 will
    # not continue growing up, which means its height stay the same.
    sleep 3
    echo "DONE"

    echo "5) Take snapshot on node-0 at height {0, 2, 100000} ..."
    for height in {0,2,10000}; do
        cd "${BINARY_DIR}"/"${CHAIN_NAME}"/0
        "${BINARY_DIR}"/bin/snapshot_tool \
            --cmd snapshot \
            --file snapshot-test-${height} \
            --end_height ${height} || (
                echo "FAILED"
                echo "error msg: fail to take snapshot at ${height}"
                exit 1)
    done

    cd "${BINARY_DIR}"
    echo "DONE"

    echo "6) Restore snapshot on node-0 ..."
    before_height=$(get_height 0)
    for height in {10000,2,0,10000,2,0}; do
        cd "${BINARY_DIR}"/"${CHAIN_NAME}"/0
        echo "restoring with before_height=${before_height}, height=${height} ..."

        "${BINARY_DIR}"/bin/snapshot_tool \
            --cmd restore \
            --file snapshot-test-${height} || (
                echo "FAILED"
                echo "error msg: fail to restore snapshot to ${height}"
                exit 1)

        case $height in
            0)     expect_height=${before_height};;
            2)     expect_height=2;;
            10000) expect_height=${before_height};;
        esac

        current_height=$(get_height 0)
        if [ "${current_height}" != "${expect_height}" ]; then
            echo "FAILED: expect_height(${expect_height})!= current_height(${current_height})"
            exit 1
        fi
    done

    cd "${BINARY_DIR}"
    echo "DONE"

    echo "7) Start node-1 and node-2 and check all grow up ..."
    for id in {1,2}; do
        "${BINARY_DIR}"/bin/cita bebop start "${CHAIN_NAME}"/${id}
    done

    wait_timeout=30
    for id in {0,1}; do
        timeout=$(check_height_growth_normal ${id} $wait_timeout) || (echo "FAILED"
                                                                      echo "error msg: ${timeout}"
                                                                      exit 1)
    done

    echo "DONE"
}

main "$@"
