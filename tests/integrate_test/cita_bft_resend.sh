#!/usr/bin/env bash

# Set bash environment
set -e
if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
    SED="gsed"
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
    SED="sed"
fi

# Set CITA system environment
BINARY_DIR=${SOURCE_DIR}/target/install
CHAIN_NAME="mock-chain"
SUPER_ADMIN="0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"

################################################################################
echo "0) Prepare ..."
source "${SOURCE_DIR}/tests/integrate_test/util.sh"
cd "${BINARY_DIR}"
echo "DONE"

################################################################################
echo "1) Clean Up ..."
cleanup "${CHAIN_NAME}"
echo "DONE"

################################################################################
echo "2) Generate CITA configurations ..."
${BINARY_DIR}/scripts/create_cita_config.py create \
    --nodes "127.0.0.1:4000" \
    --super_admin "${SUPER_ADMIN}" \
    --chain_name "${CHAIN_NAME}" \
    --timestamp 1524000000
echo "DONE"

################################################################################
echo "3) Start CITA components manually"
${BINARY_DIR}/bin/cita setup ${CHAIN_NAME}/0
${BINARY_DIR}/bin/cita start ${CHAIN_NAME}/0 trace
sleep 3
${BINARY_DIR}/bin/cita stop ${CHAIN_NAME}/0

cd ${CHAIN_NAME}/0
${BINARY_DIR}/bin/cita-auth -c auth.toml & auth_pid=$!
${BINARY_DIR}/bin/cita-bft -c consensus.toml -p privkey & bft_pid=$!
${BINARY_DIR}/bin/cita-chain -c chain.toml & chain_pid=$i
${BINARY_DIR}/bin/cita-executor -c executor.toml & executor_pid=$!
${BINARY_DIR}/bin/cita-jsonrpc -c jsonrpc.toml & jsonrpc_pid=$!
${BINARY_DIR}/bin/cita-network -c network.toml & network_pid=$!
wait_timeout=30
timeout=`check_height_growth_normal 0 $wait_timeout` || (echo "FAILED"
                                                         echo "error msg: ${timeout}"
                                                         exit 1)

################################################################################
echo "4) Kill cita-executor and cita-chain, and rm -rf data/statedb"
jobs -l | grep cita-executor | awk '{print $2}' | xargs -I {} kill {}
sleep 3
jobs -l | grep cita-chai     | awk '{print $2}' | xargs -I {} kill {}
rm -rf data/statedb
sleep 10

################################################################################
echo "5) Restart CITA"
kill ${auth_pid} ${bft_pid} ${jsonrpc_pid} ${network_pid}

cd ${BINARY_DIR}
${BINARY_DIR}/bin/cita start ${CHAIN_NAME}/0 trace

wait_timeout=30
timeout=`check_height_growth_normal 0 $wait_timeout` || (echo "FAILED"
                                                         echo "error msg: ${timeout}"
                                                         exit 1)


################################################################################
echo "6) Cleanup"
cleanup "${CHAIN_NAME}"
echo "DONE"
