#!/usr/bin/env bash

set -e

ECONOMICAL_MODEL="0"
if [ -n $1 ] && [ "$1" = "charge" ]; then
    ECONOMICAL_MODEL="1"
fi

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install
TESTS_DIR=${SOURCE_DIR}/tests/interfaces/rpc/tests

################################################################################
echo -n "0) prepare  ...  "
. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}
echo "DONE"

################################################################################
echo -n "1) cleanup   ...  "
cleanup "mock-chain"
echo "DONE"

################################################################################
echo -n "2) generate config  ...  "
if [ ! -d "resource" ]; then
    mkdir resource
fi

AUTHORITIES=`cat ${SOURCE_DIR}/tests/interfaces/rpc/config/authorities |xargs echo |sed "s/ /,/g"`

${BINARY_DIR}/scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
             --chain_name "mock-chain" \
             --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
             --contract_arguments "SysConfig.economicalModel=${ECONOMICAL_MODEL}" \
             --contract_arguments "SysConfig.chainId=123" \
             --timestamp 1524000000 \
             --authorities ${AUTHORITIES}
echo "DONE"

################################################################################
echo -n "3) just start mock-chain/0  ...  "
${BINARY_DIR}/bin/cita setup mock-chain/0 > /dev/null
${BINARY_DIR}/bin/cita start mock-chain/0 trace
echo "DONE"

################################################################################
echo -n "4) generate mock data  ...  "
AMQP_URL=amqp://guest:guest@localhost/mock-chain/0 \
        ${BINARY_DIR}/bin/chain-executor-mock \
        -m ${SOURCE_DIR}/tests/interfaces/rpc/config/blockchain.yaml
echo "DONE"

################################################################################
echo -n "5) check mock data  ...  "
python3 ${SOURCE_DIR}/tests/interfaces/rpc/test_runner.py \
        --rpc-url http://127.0.0.1:1337 \
        --directory ${SOURCE_DIR}/tests/interfaces/rpc/
echo "DONE"

################################################################################
echo -n "6) stop mock-chain/0  ...  "
${BINARY_DIR}/bin/cita stop mock-chain/0
echo "DONE"

################################################################################
echo -n "7) cleanup ... "
cleanup "mock-chain"
echo "DONE"
