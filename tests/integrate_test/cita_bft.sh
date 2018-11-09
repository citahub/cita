#!/bin/bash
set +e
tx_num=0
pid=0
CUR_PATH=$(cd `dirname $0`; pwd)
sudo rabbitmqctl stop_app
sudo rabbitmqctl reset
sudo rabbitmqctl start_app
cd ${CUR_PATH}/../../admintool/
./scripts/create_cita_config.py create \
    --chain_name "node" \
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"

setup_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node/${id}
    ./cita setup ${id}
}

start_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node/${id}
    ./cita start ${id}
}

stop_node() {
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node/${id}
    ./cita stop ${id}
}

stop_all () {
    stop_node 0
    stop_node 1
    stop_node 2
    stop_node 3
    stop_consensus 3
}

delete_pid_file()
{
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node/${id}
    if [ -f "consensus_cita_bft.pid" ]; then
        rm -rf consensus_cita_bft.pid
    fi
}

stop_consensus_cmd()
{
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node/${id}

    if [ ! -f "consensus_cita_bft.pid" ]; then
        consensus_cita_bft_pid=$(tail -3 .pid | head -1)
#        kill -9 $consensus_cita_bft_pid
    else
        consensus_cita_bft_pid=$(cat consensus_cita_bft.pid)
#        kill -9 $consensus_cita_bft_pid
        rm -rf consensus_cita_bft.pid
    fi

    flag=$(ps -ef | grep $consensus_cita_bft_pid | grep -v grep | wc -l)
    if [ $flag -gt 0 ]; then
        kill -9 $consensus_cita_bft_pid
    fi
}

stop_consensus()
{
#    killall consensus_cita_bft
    for((i=0; i<=$1;i++))
    do
        stop_consensus_cmd $i
    done
}

start_consensus_cmd()
{
    id=$1
    cd ${CUR_PATH}/../../admintool/release/node/${id}
    RUST_LOG=consensus_cita_bft bin/consensus_cita_bft -c consensus.json     >log/node/${id}.consensus  2>&1 &
    echo $! > consensus_cita_bft.pid
}

start_consensus()
{
    while :
    do
        process=$(ps -ef | grep "bin/consensus_cita_bft" | grep -v grep | wc -l)
        if [ $process -lt 4 ]; then
            break
        fi
    done
    for((i=0; i<=$1;i++))
    do
        start_consensus_cmd $i
        sleep 1
    done
}

start_all () {
    start_node 0
    start_node 1
    start_node 2
    start_node 3
}

get_height(){
    h=`${CUR_PATH}/blockNumber.sh`
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

delete_log()
{
    cd ${CUR_PATH}/../wrk_benchmark_test/
    if [ -f "result.log" ]; then
        rm -rf result.log
    fi
}

delete_tc()
{
    flag=$(tc -s qdisc  show dev lo | grep "qdisc prio" | wc -l)
    if [ $flag -eq 1 ]; then
        sudo tc qdisc del dev lo root
    fi
}

stop_trans()
{
    flag=$(ps -ef | grep benchmark.sh | grep -v grep | wc -l)
    if [ $flag -eq 1 ]; then
        killall benchmark.sh
    fi
    flag=$(ps -ef | grep make_transaction | grep -v grep | wc -l)
    if [ $flag -eq 1 ]; then
        killall make_transaction
    fi
}

start_send_tx()
{
    stop_trans
    cd ${CUR_PATH}/../wrk_benchmark_test/
#    if [ ! -f "result.log" ]; then
    if [ $1 -eq 0 ]; then
        ./benchmark.sh
        err=`echo $?`
        if [ $err -ne 0 ]; then
            echo "create account error!"
            delete_tc
            stop_all
            #stop_consensus
            stop_trans
            exit 1
        fi
    fi

    sleep 10

    #./setPortDelay.sh 4000 1000 10 > /dev/null &
    pid=$!
    ./benchmark.sh config_call.json 2 > result.log &
    while :
    do

            if [ ! -f "result.log" ]; then
                continue
            fi
            flag=$(grep "write successfully\.\[" result.log | wc -l)
            err=`echo $?`
            if [ $err -ne 0 ]; then
                continue
            fi

            if [ $flag -eq 1 ]; then
                #stop_consensus
                stop_consensus $2
                sleep 2
                break
            fi
    done
}

send_tx_over()
{
    cd ${CUR_PATH}/../wrk_benchmark_test/
    while :
    do
        if [ ! -f "result.log" ]; then
            continue
        fi

        flag=$(grep "send tx num" result.log | wc -l)
        err=`echo $?`
        if [ $err -ne 0 ]; then
            continue
        fi
        if [ $flag -eq 1 ]; then
            tx_num=$(grep "write successfully\.\[" result.log | grep  -o "[[:digit:]]*")
            flag=$(grep "send tx num" result.log | grep "$tx_num" | wc -l)
            if [ $flag -eq 0 ]; then
                echo "###send_tx_over exit [$tx_num]"
                delete_tc
                stop_all
                stop_trans
                exit 1
            fi
            break
        fi
    done
}


echo "###start nodes..."
(setup_node 0;start_node 0) &
(setup_node 1;start_node 1) &
(setup_node 2;start_node 2) &
(setup_node 3;start_node 3) &

echo "###wait for start..."
sleep 80

check_height_change

stop_num=0

echo "###start_test_consensus_cita_bft"
delete_pid_file 0
delete_pid_file 1
delete_pid_file 2
delete_pid_file 3
run=0
while :
do
    if [ $stop_num -gt 3 ]; then
        break
    fi
    sleep 30
    echo "###start consensus_cita_bft process $stop_num"
    delete_log
    start_send_tx $run $stop_num &
    start_consensus $stop_num &
    send_tx_over
    run=1
    echo "###end consensus_cita_bft process $stop_num"
    stop_num=$[$stop_num+1]
done

delete_tc
sleep 30
check_height_change
stop_all
stop_trans
echo "###Test OK"
exit 0
