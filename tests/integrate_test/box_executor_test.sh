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
CHAIN_NAME="mock-chain"
NODE_NAME="${CHAIN_NAME}/0"
NODE_DIR="${BINARY_DIR}/${NODE_NAME}"
TNODE=`echo ${NODE_NAME} | sed 's/\//%2f/g'`

################################################################################
echo -n "0) prepare  ...  "
. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}
echo "DONE"

################################################################################
echo -n "1) cleanup   ...  "
cleanup ${CHAIN_NAME}
echo -n "DONE"

################################################################################
echo -n "2) generate config  ...  "
if [ ! -d "resource" ]; then
    mkdir resource
fi

AUTHORITIES=`cat ${SOURCE_DIR}/tests/interfaces/rpc/config/authorities | xargs echo |sed "s/ /,/g"`
${BINARY_DIR}/scripts/create_cita_config.py create \
             --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
             --chain_name ${CHAIN_NAME} \
             --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
             --contract_arguments "SysConfig.economicalModel=${ECONOMICAL_MODEL}" \
             --contract_arguments "SysConfig.chainId=123" \
             --timestamp 1524000000 \
             --authorities ${AUTHORITIES}
echo -n "DONE"

################################################################################
echo -n "3) start cita-chain and cita-executor on ${NODE_NAME} ...  "
${BINARY_DIR}/bin/cita setup ${NODE_NAME}

curl -i -u guest:guest -H content-type:application/json -XDELETE \
    http://localhost:15672/api/queues/${TNODE}/consensus > /dev/null

cd ${NODE_DIR}
${BINARY_DIR}/bin/cita-chain    -c chain.toml &
chain_pid=$!
${BINARY_DIR}/bin/cita-executor -c executor.toml &
executor_pid=$!
echo -n "DONE"

##  ################################################################################
echo -n "4) testing ...  "
AMQP_URL=amqp://guest:guest@localhost/${NODE_NAME} \
    timeout 100s ${BINARY_DIR}/bin/box_executor \
    -m ${SOURCE_DIR}/tests/interfaces/rpc/config/blockchain.yaml
echo -n "DONE"

################################################################################
echo -n "5) stop cita-chain and cita-executor on ${NODE_NAME}"
kill "${chain_pid}"
kill "${executor_pid}"
cleanup ${CHAIN_NAME}
echo -n "PASS"
exit 0
