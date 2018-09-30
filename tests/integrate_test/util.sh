
sudo(){
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo $*
    fi
    set +o noglob
}

# 失败后不需要清理,保留现场;成功后清理现场.
cleanup() {
    for pid in cita-forever cita-jsonrpc cita-auth cita-chain cita-network cita-bft trans_evm cita-executor; do
        ps ax | grep ${pid} | grep -v grep | awk '{print $1}' | xargs -n 1 -I %  kill -9 % 2>&1 >/dev/null ||true
    done

    rm -rf ${BINARY_DIR}/${1:-node}*
    rm -rf ${BINARY_DIR}/*.json
    sudo tc qdisc del dev lo root> /dev/null 2>&1||true

    pid_file=/tmp/cita_basic-trans_evm.pid
    if [ -e ${pid_file} ] ; then
        for pid in $(cat ${pid_file}) ; do
            kill -9 ${pid}  2>&1 > /dev/null || true
        done
    fi
}

get_height(){
    if [ $# -ne 1 ] ; then
        echo "usage: $0 node_id"
        return 1
    fi
    id=$1
    timeout=60                  # 60 seconds
    start=$(date +%s)

    while [ 1 ] ; do
        height=$(${SOURCE_DIR}/tests/integrate_test/blockNumber.sh 127.0.0.1 $((1337+${id})))
        if [ $? -eq 0 ] ; then
            echo ${height}
            return 0
        fi

        now=$(date +%s)
        if [ $((now-start-timeout)) -gt 0 ] ; then
            echo "timeout: ${timeout}"
            return 1
        fi
        sleep 1
    done
    return 1
}

# output information about time used if exit 0
check_height_growth () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id timeout"
        return 1
    fi
    id=$1
    timeout=$2                 # seconds
    old=$(get_height ${id})
    if [[ $? -ne 0 ]]; then
        echo "failed to get_height(old): ${old}"
        return 1
    fi
    start=$(date +%s)
    while [ 1 ] ; do
        new=$(get_height ${id})
        if [[ $? -ne 0 ]] ; then
            echo "failed to get_height! old height: ${old} new height: ${new}"
            return 1
        fi

        now=$(date +%s)
        if [ ${new} -gt $(($old + 2)) ]; then
            echo "$((now-start))"
            return 0
        fi
        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "time used: $((now-start)) old height: ${old} new height: ${new}"
            return 20
        fi
        sleep 1
    done
    return 1
}

check_height_growth_normal () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 id timeout"
        return 1
    fi

    id=$1
    timeout=$2
    start=$(date +%s)
    for i in {0..1}; do
        msg=$(check_height_growth ${id} ${timeout})
        if [ $? -ne 0 ] ; then
            echo "check_height_growth_normal failed id(${id}) timeout(${timeout}) msg(${msg})"
            return 1
        fi
        if [[ ${msg} -lt ${timeout} ]]; then
            now=$(date +%s)
            echo "$((now-start))"
            return 0
        fi
    done
    echo "check_height_growth_normal timeout(${timeout}) msg(${msg})"
    return 1
}

# output information about time used if exit 0
check_height_sync () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id refer_node_id"
        return 1
    fi
    id=$1
    refer=$2
    timeout=180                  # seconds
    refer_height=$(get_height ${refer})
    if [ $? -ne 0 ] ; then
        echo "check_height_sync failed to get_height(refer): ${refer_height}"
        return 1
    fi
    start=$(date +%s)

    while [ 1 ] ; do
        height=$(get_height ${id})
        if [ $? -ne 0 ] ; then
            echo "check_height_sync failed to get_height(sync): ${height}"
            return 1
        fi
        now=$(date +%s)
        if [ ${height} -gt ${refer_height} ]; then
            echo "$((now-start))"
            return  0
        fi

        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "check_height_sync timeout(${timeout}) time used $((now-start))  refer height ${refer_height} sync height ${height}"
            return 1
        fi
        sleep 1
    done
    return 1
}

check_height_stopped () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id timeout"
        return 1
    fi
    id=$1
    timeout=$2
    old=$(get_height ${id})
    if [ $? -ne 0 ] ; then
        echo "check_height_stopped failed to get_height(old): ${old}"
        return 1
    fi

    start=$(date +%s)
    while [ 1 ] ; do
        now=$(date +%s)
        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "$((now-start))"
            return 0
        fi
        new=$(get_height ${id})
        if [ $? -ne 0 ] ; then
            echo "check_height_stopped failed to get_height(new): ${new}"
            return 1
        fi
        if [ $new -gt $(($old + 2)) ]; then
            # if two more blocks was generated, it shows cita still reach consensus.
            echo "check_height_stopped height change from ${old} to ${new}"
            return 1
        fi
        sleep 1
    done
    return 1
}

set_delay_at_port() {
    if [ $# -ne 2 ] ; then
        echo "usage: set_delay_at_port port delay"
        return 1
    fi
    port=$1
    delay=$2
    # TODO: need more description
    sudo tc qdisc  add dev lo root        handle  1:  prio bands 4                                         >/dev/null 2>&1 || true
    sudo tc qdisc  add dev lo parent 1:4  handle 40:  netem delay ${delay}ms                               >/dev/null 2>&1 || true
    sudo tc filter add dev lo protocol ip parent  1:0 prio 4 u32 match ip dport ${port} 0xffff flowid 1:4  >/dev/null 2>&1 || true
}
unset_delay_at_port() {
    if [ $# -ne 1 ] ; then
        echo "usage: $0 port"
        return 1
    fi
    port=$1
    #sudo tc filter del dev lo protocol ip parent  1:0 prio 4 u32 match ip dport ${port} 0xffff flowid 1:4  >/dev/null 2>&1 || true
    sudo tc qdisc del dev lo root> /dev/null 2>&1||true
}

setup_node() {
    id=$1
    ./bin/cita setup node/${id}
}

start_node() {
    id=$1
    ./bin/cita start node/${id} ${debug}
}

stop_node() {
    id=$1
    ./bin/cita stop node/${id}
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
