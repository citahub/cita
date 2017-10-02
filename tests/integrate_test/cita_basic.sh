#!/bin/bash
set -e

SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

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
./bin/admintool.sh > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) start nodes  ...  "
for i in {0..3} ; do
    bin/cita setup node$i  > /dev/null
done
for i in {0..3} ; do
    bin/cita start node$i > /dev/null
done
echo "DONE"

################################################################################
echo -n "4) check height growth  ...  "
msg=$(check_height_growth 0) || (echo "FAILED"
                                 echo "failed to check_height_growth 0: ${msg}"
                                 exit 1)
echo "${msg} DONE"

################################################################################
echo -n "5) create contract  ...  "
${BINARY_DIR}/bin/trans_evm --config ${SOURCE_DIR}/tests/wrk_benchmark_test/config_create.json 2>&1 |grep "sucess" > /dev/null
if [ $? -ne 0 ] ; then
    exit 1
fi
echo "DONE"

################################################################################
echo -n "6) call contract  ...  "
${BINARY_DIR}/bin/trans_evm --config ${SOURCE_DIR}/tests/wrk_benchmark_test/config_call.json 2>&1 |grep "sucess" > /dev/null
if [ $? -ne 0 ] ; then
    exit 1
fi
echo "DONE"

################################################################################
echo -n "7) stop node3, check height growth  ...  "
bin/cita stop node3
msg=$(check_height_growth 0) || (echo "FAILED"
                                 echo "failed to check_height_growth 0: ${msg}"
                                 exit 1)
echo "${msg} DONE"

################################################################################
echo -n "7) stop node2, check height stopped  ...  "
bin/cita stop node2
msg=$(check_height_stopped 0) || (echo "FAILED"
                                  echo "failed to check_height_stopped 0: ${msg}"
                                  exit 1)
echo "DONE"

################################################################################
echo -n "9) start node2, check height growth  ...  "
bin/cita start node2
msg=$(check_height_growth 0) || (echo "FAILED"
                                 echo "failed to check_height_growth 0: ${msg}"
                                 exit 1)
echo "${msg} DONE"

################################################################################
echo -n "10) start node3, check synch  ...  "
node0_height=$(get_height 0)

if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${node0_height}"
    exit 1
fi
bin/cita start node3
msg=$(check_height_sync 3 0) || (echo "FAILED"
                                  echo "failed to check_height_synch 3 0: ${msg}"
                                  exit 1)
echo "${msg} DONE"

################################################################################
echo -n "11) stop all nodes, check height changed after restart  ...  "
before_height=$(get_height 0)
if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${before_height}"
    exit 1
fi
for i in {0..3}; do
    bin/cita stop node$i
done
sleep 1
for i in {0..3}; do
    bin/cita start node$i
done
msg=$(check_height_growth 0) || (echo "FAILED"
                                 echo "failed to check_height_growth 0: ${msg}"
                                 exit 1)
after_height=$(get_height 0)
if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${after_height}"
    exit 1
fi
if [ $after_height -le $before_height ]; then
    echo "FAILED"
    echo "before:${before_height} after:${after_height}"
    exit 1
fi
echo "${msg} DONE"

################################################################################
echo -n "12) stop&clean node3, check height synch after restart  ...  "
bin/cita stop node3
bin/cita clean node3
bin/cita start node3
msg=$(check_height_sync 3 0) || (echo "FAILED"
                                  echo "failed to check_height_synch 3 0: ${msg}"
                                  exit 1)
echo "${msg} DONE"


################################################################################
echo -n "13) cleanup ... "
cleanup
echo "DONE"
