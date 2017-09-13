#!/bin/bash
set +e
CUR_PATH=$(cd `dirname $0`; pwd)
cd ${CUR_PATH}/../../admintool/
./admintool.sh

setup_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    ./cita setup ${id}
}

start_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    ./cita start ${id} debug
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

get_height(){
    nodeid=$1
    if [ ! -n "$nodeid" ]; then
        nodeid=0
    fi
    h=`${CUR_PATH}/cita_blockNumber.sh 127.0.0.1 $((1337+${nodeid}))`
    h=$(echo $h | sed 's/\"//g')
    echo $((h))    
}

check_height_change () {
    echo "check block height"
    old_height=$(get_height)
    echo "block height $old_height"
    sleep 30
    new_height=$(get_height)
    echo "block height $new_height"
    if [ $new_height -eq $old_height ]; then
        stop_all
        exit 1
    fi
}

check_height_stop () {
    echo "check block height"
    old_height=$(get_height)
    echo "block height $old_height"
    sleep 30
    new_height=$(get_height)
    echo "block height $new_height"
    if [ $new_height -ne $old_height ]; then
        stop_all
        exit 1
    fi
}

create_contract() {
    cd ${CUR_PATH}/../wrk_benchmark_test/
    echo "create contract"
    ./benchmark.sh
    if [ $? -ne 0 ]
    then  
        exit
    fi
}

send_tx() {
    cd ${CUR_PATH}/../wrk_benchmark_test/
    while [ 0 -le 1 ]
    do
        echo "call contract"
        ./benchmark.sh config_call.json >/dev/null
        if [ $? -ne 0 ]
        then  
            echo "call contract error"
        fi
        sleep 5
    done
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
create_contract
(send_tx)&
pid=$!

echo "###stop node3..."
stop_node 3
check_height_change

echo "###stop node2..."
stop_node 2
check_height_stop

echo "###start node2..."
start_node 2
before_height=$(get_height)
#recover too slow
sleep 420
after_height=$(get_height)
echo "node2 recover block height $after_height"
if [ $after_height -le $before_height ]; then
    stop_all
    exit 1
fi

echo "###start node3...check sync"
node0_height=$(get_height)
echo "node0 block height $node0_height"
start_node 3
sleep 3
befor_sync_height=$(get_height 3)
echo "node3 block height before sync $befor_sync_height"
sleep 60
after_sync_height=$(get_height 3)
echo "node3 block height after sync $after_sync_height"
if [ $after_sync_height -le $befor_sync_height ]; then
    stop_all
    exit 1
fi
echo "wait recover from sync..."
sleep 420
check_height_change

echo "###stop all node...check for restart"
before_height=$(get_height)
echo "before restart block height $before_height"
stop_all
sleep 3
start_all
sleep 60
after_height=$(get_height)
echo "after restart block height $after_height"
if [ $after_height -le $before_height ]; then
    stop_all
    exit 1
fi


echo "###node3 clean data then sync"
stop_node 3
setup_node 3
start_node 3
sleep 300
check_height_change
node0_height=$(get_height)
node3_height=$(get_height 3)
echo "node0 height $node0_height"
echo "node3 height $node3_height"
if [ $node0_height -ne $node3_height ]; then
    stop_all
    exit 1
fi

kill -9 $pid
stop_all
echo "###Test OK"
exit 0

