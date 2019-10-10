#!/bin/bash
# set -e

if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$0")"/../..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$0")"/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install

test_fee_back() {
    local version=$1
    cd ./scripts/txtool/txtool
    python3 "${SOURCE_DIR}"/tests/integrate_test/test_fee_back.py --version="$version"
    cd ../../..
}

test_perm_denied() {
    local version=$1
    cd ./scripts/txtool/txtool
    python3 "${SOURCE_DIR}"/tests/integrate_test/test_perm_denied.py --version="$version"
    cd ../../..
}

test_change_block_interval() {
    local version=$1
    cd ./scripts/txtool/txtool
    python3 "${SOURCE_DIR}"/tests/integrate_test/test_block_interval.py --version="$version"
    cd ../../..
}

main() {
    echo -n "0) prepare  ...  "
    # shellcheck source=/dev/null
    . ${SOURCE_DIR}/tests/integrate_test/util.sh
    cd "${BINARY_DIR}"
    echo "DONE"

    echo -n "1) generate config  ...  "
    create_config \
        --contract_arguments "SysConfig.checkFeeBackPlatform=true" \
        --contract_arguments "SysConfig.checkCreateContractPermission=true" \
        --contract_arguments "SysConfig.economicalModel=1" \
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

    echo -n "4) Run fee back tests ... "
    test_fee_back 2
    echo "DONE"

    echo -n "5) Check permission denied ... "
    test_perm_denied 2
    echo "Done"

    echo -e "6) Run block interval tests ...\n"
    test_change_block_interval 2
    timeout=$(check_height_growth_normal 0 15)
    timeout=$(check_height_growth_normal 0 30) && (echo "FAILED"
                                                    echo "failed to change block interval"
                                                    exit 1)
    echo "Done"
}

main "$@"
