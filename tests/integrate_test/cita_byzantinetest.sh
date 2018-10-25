#!/bin/bash
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
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
    --contract_arguments "SysConfig.economicalModel=${ECONOMICAL_MODEL}" \
    > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) start nodes  ...  "
for i in {0..3} ; do
    bin/cita setup node/$i  > /dev/null
done
for i in {0..3} ; do
    bin/cita start node/$i debug > /dev/null &
done
echo "DONE"

################################################################################
echo -n "4) check alive  ...  "
timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                              echo "failed to check_height_growth 0: ${timeout}"
                                              exit 1)
echo "${timeout}s DONE"

################################################################################
echo "5) set delay at one nodes, , output time used for produce block growth"
delay=10000
for i in {0..3}; do
    id=$(($i%4))
    echo -n "set delay at node ${id} ... "
    refer=$((($i+1)%4))
    port=$((4000+${id}))
    set_delay_at_port ${port} ${delay}
    timeout1=$(check_height_growth_normal ${refer} 60) ||(echo "FAILED"
                                                         echo "failed to check_height_growth: ${timeout}"
                                                         exit 1)
    unset_delay_at_port ${port}
    #synch for node ${id}
    timeout=$(check_height_sync ${id} ${refer}) ||(echo "FAILED"
                                                   echo "failed to check_height_sync: ${timeout}"
                                                   exit 1)
    echo "${timeout1}s DONE"
done

################################################################################
echo "6) set delay at two nodes, output time used for produce block"
delay=3000
for i in {0..3}; do
    id1=$i
    id2=$((($i+1)%4))
    refer=$((($i+2)%4))
    echo -n "set delay=${delay} at nodes ${id1},${id2} ... "
    set_delay_at_port $((4000+${id1})) ${delay}
    set_delay_at_port $((4000+${id2})) ${delay}

    timeout1=$(check_height_growth_normal ${refer} 100) ||(echo "FAILED"
                                                          echo "failed to check_height_growth ${refer}: ${timeout}"
                                                          exit 1)
    unset_delay_at_port $((4000+${id1}))
    unset_delay_at_port $((4000+${id2}))
    sleep 3
    timeout=$(check_height_growth_normal ${refer} 100) ||(echo "FAILED"
                                                        echo "failed to check_height_growth ${refer}: ${timeout}"
                                                        exit 1)
    #synch for node id1, id2
    timeout=$(check_height_sync ${id1} ${refer}) ||(echo "FAILED"
                                                    echo "failed to check_height_sync ${id1}: ${timeout}"
                                                    exit 1)
    timeout=$(check_height_sync ${id2} ${refer}) ||(echo "FAILED"
                                                    echo "failed to check_height_sync ${id2}: ${timeout}"
                                                    exit 1)
    echo "${timeout1}s DONE"
done

################################################################################
echo "7) set delay at all nodes, output time used for produce block"
for i in {0..6}; do
    delay=$((i*400))
    timeout=$(check_height_growth_normal 0 60) ||(echo "FAILED"
                                                  echo "failed to check_height_growth: ${timeout}"
                                                  exit 1)
    echo -n "set delay=${delay} ... "
    for node in {0..3} ; do
        set_delay_at_port $((4000+${node})) ${delay}
    done
    timeout=$(check_height_growth_normal 0 60) ||(echo "FAILED"
                                                  echo "failed to check_height_growth: ${timeout}"
                                                  exit 1)
    for node in {0..3} ; do
        unset_delay_at_port $((4000+${node}))
    done
    sleep 4
    echo "${timeout}s DONE"
done

echo "DONE"

echo "11) cleanup"
cleanup
echo "DONE"
exit 0
