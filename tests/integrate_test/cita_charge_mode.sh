#!/bin/bash
set -e

if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install

test_charge_mode() {
    local version=$1
    local node_0_privkey
    local node_1_privkey
    local node_2_privkey
    local node_3_privkey
    node_0_privkey=$(cat ./"$CHAIN_NAME"/0/privkey)
    node_1_privkey=$(cat ./"$CHAIN_NAME"/1/privkey)
    node_2_privkey=$(cat ./"$CHAIN_NAME"/2/privkey)
    node_3_privkey=$(cat ./"$CHAIN_NAME"/3/privkey)
    cd ./scripts/txtool/txtool
    python3 "${SOURCE_DIR}"/tests/integrate_test/test_charge_mode.py \
            --miner-privkeys \
            "${node_0_privkey}" \
            "${node_1_privkey}" \
            "${node_2_privkey}" \
            "${node_3_privkey}" \
            --version="$version"
    cd ../../..
}

update_version() {
    local version=$1
    cd ./scripts/txtool/txtool
    python3 "${SOURCE_DIR}"/tests/integrate_test/update_version.py --version="$version"
    cd ../../..
}

main() {
    echo -n "0) prepare  ...  "
    # shellcheck source=/dev/null
    . ${SOURCE_DIR}/tests/integrate_test/util.sh
    cd "${BINARY_DIR}"
    cleanup
    echo "DONE"

    echo -n "1) generate config  ...  "
    create_config \
        --contract_arguments "SysConfig.checkFeeBackPlatform=true" \
        --contract_arguments "SysConfig.economicalModel=1" \
        --contract_arguments "VersionManager.version=0" \
        --contract_arguments "SysConfig.chainOwner=0x36a60d575b0dee0423abb6a57dbc6ca60bf47545"
    echo "DONE"

    echo -n "2) start nodes  ...  "
    start_nodes
    echo "DONE"

    echo -n "3) check alive  ...  "
    timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                                   echo "failed to check_height_growth 0: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "4) Run charge mode tests in v0 ...  "
    test_charge_mode 0
    echo "DONE"

    echo -n "5) Update to chainIDV1 ... "
    update_version 0
    echo "DONE"

    echo -n "6) check alive  ...  "
    timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                                   echo "failed to check_height_growth 0: ${timeout}"
                                                   exit 1)
    echo "${timeout}s DONE"

    echo -n "7) Run charge mode tests in v1 ...  "
    test_charge_mode 1
    echo "DONE"

    echo -n "8) cleanup ..."
    cleanup
    echo "DONE"
}

main "$@"
