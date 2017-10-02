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
echo -n "4) check alive  ...  "
msg=$(check_height_growth 0) || (echo "FAILED"
                                 echo "failed to check_height_growth 0: ${msg}"
                                 exit 1)
echo "DONE ${msg}"

################################################################################
echo "5) set delay at one nodes, check height growth"
delay=10000
for i in {0..3}; do
    id=$(($i%4))
    echo -n "set delay at node ${id} ... "
    refer=$((($i+1)%4))
    port=$((4000+${id}))
    set_delay_at_port ${port} ${delay}
    msg=$(check_height_growth ${refer}) ||(echo "FAILED"
                                           echo "failed to check_height_growth: ${msg}"
                                           exit 1)
    unset_delay_at_port ${port}
    #synch for node ${id}
    msg=$(check_height_sync ${id} ${refer}) ||(echo "FAILED"
                                               echo "failed to check_height_growth: ${msg}"
                                               exit 1)
    echo "${msg} DONE"
done

################################################################################
echo "7) set delay at two nodes, check height growth"
delay=3000
for i in {0..3}; do
    id1=$i
    id2=$((($i+1)%4))
    refer=$((($i+2)%4))
    echo -n "set delay=${delay} at nodes ${id1},${id2} ... "
    set_delay_at_port $((4000+${id1})) ${delay}
    set_delay_at_port $((4000+${id2})) ${delay}

    msg=$(check_height_growth ${refer}) ||(echo "FAILED"
                                           echo "failed to check_height_growth ${refer}: ${msg}"
                                           exit 1)
    unset_delay_at_port $((4000+${id1}))
    unset_delay_at_port $((4000+${id2}))
    #synch for node id1, id2
    msg=$(check_height_sync ${id1} ${refer}) ||(echo "FAILED"
                                                echo "failed to check_height_sync ${id1}: ${msg}"
                                                exit 1)
    msg=$(check_height_sync ${id2} ${refer}) ||(echo "FAILED"
                                                echo "failed to check_height_sync ${id2}: ${msg}"
                                                exit 1)
    echo "${msg} DONE"
done


################################################################################
echo "8) set delay at all nodes, output time used for produce block"
for i in {0..6}; do
    delay=$((i*400))
    msg=$(check_height_growth 0) ||(echo "FAILED"
                                    echo "failed to check_height_growth: ${msg}"
                                    exit 1)
    echo -n "set delay=${delay} ... "
    for node in {0..3} ; do
        set_delay_at_port $((4000+${node})) ${delay}
    done
    msg=$(check_height_growth 0) ||(echo "FAILED"
                                    echo "failed to check_height_growth: ${msg}"
                                    exit 1)
    for node in {0..3} ; do
        unset_delay_at_port $((4000+${node}))
    done
    sleep 4
    echo "${msg} DONE"
done


echo "7) cleanup"
cleanup
echo "DONE"
exit 0
