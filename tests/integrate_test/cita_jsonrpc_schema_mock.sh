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
cleanup
echo "DONE"

################################################################################
echo -n "2) generate config  ...  "
if [ ! -d "resource" ]; then
    mkdir resource
fi

AUTHORITIES=`cat ${SOURCE_DIR}/tests/interfaces/rpc/config/authorities |xargs echo |sed "s/ /,/g"`

${BINARY_DIR}/scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
             --contract_arguments "SysConfig.economical_model=${ECONOMICAL_MODEL}" \
             --contract_arguments "SysConfig.chain_id=123" \
             --timestamp 1524000000 \
             --authorities ${AUTHORITIES} > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) just start node0  ...  "
${BINARY_DIR}/bin/cita setup node/0 > /dev/null
cp ${SOURCE_DIR}/tests/interfaces/rpc/config/genesis.json node/0/genesis.json
${BINARY_DIR}/bin/cita start node/0 trace > /dev/null &
echo "DONE"

################################################################################
echo -n "4) generate mock data  ...  "
AMQP_URL=amqp://guest:guest@localhost/node/0 \
        ${SOURCE_DIR}/target/debug/chain-executor-mock \
        -m ${SOURCE_DIR}/tests/interfaces/rpc/config/blockchain.yaml
echo "DONE"

################################################################################
echo -n "5) check mock data  ...  "
python2 ${SOURCE_DIR}/tests/interfaces/rpc/test_runner.py \
        --rpc-url http://127.0.0.1:1337 \
        --directory ${SOURCE_DIR}/tests/interfaces/rpc/
echo "DONE"

################################################################################
echo -n "6) stop node0  ...  "
${BINARY_DIR}/bin/cita stop node/0
echo "DONE"

################################################################################
echo -n "7) cleanup ... "
cleanup
echo "DONE"
