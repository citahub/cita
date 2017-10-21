#!/bin/bash
#usage: start demo nodes
#       ./cita_start_with_kafka.sh 
#       ./cita_start_with_kafka.sh debug

set +e

debug=$1
consensus=$2

SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install
. ${SOURCE_DIR}/tests/integrate_test/util.sh

if [ ! -n "$consensus" ]; then
    consensus="tendermint"
fi

echo "###cleanup"
cleanup

echo "###generate config files"
cd ${BINARY_DIR}
./bin/admintool.sh -n $consensus -k >/dev/null 2>&1

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

