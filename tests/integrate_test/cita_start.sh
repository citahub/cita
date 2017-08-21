#usage: start demo nodes
#       ./cita_start.sh 
#       ./cita_start.sh debug
#!/bin/bash
set +e
consensus=$1
debug=$2
if [ ! -n "$consensus" ]; then
    consensus="tendermint"
fi
CUR_PATH=$(cd `dirname $0`; pwd)
cd ${CUR_PATH}/../../admintool/
./setup.sh
./admintool.sh -n $consensus

setup_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    sudo ./cita setup ${id}
}

start_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node${id}
    ./cita start ${id} ${debug}
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

echo "###CITA start OK"
exit 0

