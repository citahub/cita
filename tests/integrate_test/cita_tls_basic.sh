#!/bin/bash
set -e

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
fi
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
./scripts/create_cita_config.py create \
    --chain_name "node" \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --enable_tls \
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
    > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) start nodes  ...  "
for i in {0..3} ; do
    bin/cita setup node/$i  > /dev/null
done
for i in {0..3} ; do
    bin/cita start node/$i trace > /dev/null &
done
echo "DONE"

sleep 60

################################################################################
echo -n "4) check height growth normal  ...  "
timeout=$(check_height_growth_normal 0 15)||(echo "FAILED"
                                            echo "error msg: ${timeout}"
                                            exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "5) stop node3, check height growth  ...  "
bin/cita stop node/3 > /dev/null
timeout=$(check_height_growth_normal 0 15) || (echo "FAILED"
                                               echo "error msg: ${timeout}"
                                               exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "6) stop node2, check height stopped  ...  "
bin/cita stop node/2 > /dev/null
timeout=$(check_height_stopped 0 27) || (echo "FAILED"
                                         echo "error msg: ${timeout}"
                                         exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "7) start node2, check height growth  ...  "
bin/cita start node/2 trace > /dev/null &
sleep 24 #wait for recovery from stop
timeout=$(check_height_growth_normal 0 15) || (echo "FAILED"
                                               echo "error msg: ${timeout}"
                                               exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "8) start node3, check synch  ...  "
node0_height=$(get_height 0)

if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${node0_height}"
    exit 1
fi
bin/cita start node/3 trace > /dev/null &
timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                     echo "error msg: ${timeout}"
                                     exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "9) stop all nodes, check height changed after restart  ...  "
before_height=$(get_height 0)
if [ $? -ne 0 ] ; then
    echo "failed to get_height: ${before_height}"
    exit 1
fi
for i in {0..3}; do
    bin/cita stop node/$i > /dev/null
done
# sleep 1 # TODO: change to this value will produce very different result
for i in {0..3}; do
    bin/cita start node/$i trace > /dev/null &
done

timeout=$(check_height_growth_normal 0 300) || (echo "FAILED"
                                               echo "error msg: ${timeout}"
                                               exit 1)
after_height=$(get_height 0)|| (echo "failed to get_height: ${after_height}"
                                exit 1)
if [ $after_height -le $before_height ]; then
    echo "FAILED"
    echo "before:${before_height} after:${after_height}"
    exit 1
fi
echo "${timeout}s DONE"

################################################################################
echo -n "10) stop&clean node3, check height synch after restart  ...  "
bin/cita stop node/3 > /dev/null
bin/cita clean node/3 > /dev/null
bin/cita start node/3 trace > /dev/null &
timeout=$(check_height_sync 3 0) || (echo "FAILED"
                                     echo "error msg: ${timeout}"
                                     exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "11) cleanup ... "
cleanup
echo "DONE"
