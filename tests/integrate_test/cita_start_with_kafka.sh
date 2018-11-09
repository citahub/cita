#!/bin/bash
#usage: start demo nodes
#       ./cita_start_with_kafka.sh
#       ./cita_start_with_kafka.sh [error,info, warn, debug, trace]

set +e

debug=$1
consensus=$2

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install
. ${SOURCE_DIR}/tests/integrate_test/util.sh

if [ ! -n "$consensus" ]; then
    consensus="cita-bft"
fi

echo "###cleanup"
cleanup

echo "###generate config files"
cd ${BINARY_DIR}
./scripts/create_cita_config.py create \
    --chain_name "node" \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" >/dev/null 2>&1

echo "###wait for kafka start"
$SOURCE_DIR/tests/integrate_test/kafka_start.sh ${BINARY_DIR}
if [ "$?" -ne "0" ]; then
    exit 1
fi
sleep 5

echo "###start nodes"
for i in {0..3} ; do
    setup_node $i
done

for i in {0..3} ; do
    start_node $i &
done

echo -n "###check height growth"
msg=$(check_height_growth 0)|| (echo "FAILED"
                                echo "check height growth: ${msg}"
                                exit 1)
echo "###CITA start OK"
exit 0

