#!/bin/bash
set +e
CUR_PATH=$(cd `dirname $0`; pwd)
cd ${CUR_PATH}/../../admintool/
./setup.sh
./admintool.sh

setup_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    ./cita setup ${id}
}

start_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    ./cita start ${id}
}

stop_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    ./cita stop ${id}
}

stop_all () {
    stop_node 0
    stop_node 1
    stop_node 2
    stop_node 3
}

start_all () {
    start_node 0
    start_node 1
    start_node 2
    start_node 3
}

check_height_change () {
    echo "check block height"
    old_height=`${CUR_PATH}/cita_blockNumber.sh`
    old_height=$(echo $old_height | sed 's/\"//g')
    old_height=$((old_height))
    echo "block height $old_height"
    sleep 30
    new_height=`${CUR_PATH}/cita_blockNumber.sh`
    new_height=$(echo $new_height | sed 's/\"//g')
    new_height=$((new_height))
    echo "block height $new_height"
    if [ $new_height -eq $old_height ]; then
        stop_all
        exit 1
    fi
}

echo "###start nodes..."
(setup_node 0;start_node 0) &
(setup_node 1;start_node 1) &
(setup_node 2;start_node 2) &
(setup_node 3;start_node 3) &
echo `date`
echo "###wait for start..."
sleep 120
echo `date`
check_height_change

cd ${CUR_PATH}/../wrk_benchmark_test/
./benchmark.sh
sleep 10
./benchmark.sh config_call.json 2

check_height_change

stop_all
echo "###Test OK"
exit 0

