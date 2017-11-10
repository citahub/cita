#!/bin/bash
#usage: start demo nodes
#       ./cita_start.sh 
#       ./cita_start.sh [error,info, warn, debug, trace] [tendermint]

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
./bin/admintool.sh -n $consensus >/dev/null 2>&1


echo "###start nodes"
for i in {0..3} ; do
    setup_node $i
done

for i in {0..3} ; do
    start_node $i &
done

echo -n "###check height growth"
msg=$(check_height_growth 0 60)|| (echo "FAILED"
                                echo "check height growth: ${msg}"
                                exit 1)
echo "###CITA start OK"
exit 0

