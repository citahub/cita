sudo() {
    cmd=$*
    if [ "$(whoami)" = "root" ] ; then
        ${cmd}
    else
        /usr/bin/sudo ${cmd}
    fi
}
# 失败后不需要清理,保留现场;成功后清理现场.
cleanup() {
    for i in jsonrpc auth chain network consensus_tendermint trans_evm; do
        pkill $i||true
    done
    rm -rf ${BINARY_DIR}/node*
    rm -rf ${BINARY_DIR}/*.json
    sudo tc qdisc del dev lo root> /dev/null 2>&1||true
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
        height=$(${SOURCE_DIR}/tests/integrate_test/cita_blockNumber.sh 127.0.0.1 $((1337+${id})))
        if [ $? -eq 0 ] ; then
            echo ${height}
            return 0
        fi

        now=$(date +%s)
        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "timeout: ${timeout}s"
            return 1
        fi
        sleep 1
    done
}

# output information about time used if exit 0
check_height_growth () {
    if [ $# -ne 1 ] ; then
        echo "usage: $0 node_id"
        return 1
    fi
    id=$1
    timeout=60                  # seconds
    old=$(get_height ${id})
    if [ $? -ne 0 ] ; then
        echo "failed to get_height: ${old}"
        return 1
    fi

    start=$(date +%s)
    while [ 1 ] ; do
        new=$(get_height ${id})
        if [ $? -ne 0 ] ; then
            echo "failed to get_height: ${new}"
            return 1
        fi
        now=$(date +%s)
        if [ ${new} -gt ${old} ]; then
            echo "$((now-start))s"
            return 0
        fi
        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "timeout: $((now-start))s"
            return 1
        fi
        sleep 1
    done
}

# output information about time used if exit 0
check_height_sync () {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id refer_node_id"
        return 1
    fi
    id=$1
    target=$2
    timeout=60                  # seconds
    target_height=$(get_height ${target})
    if [ $? -ne 0 ] ; then
        echo "failed to get_height: ${target_height}"
        return 1
    fi
    start=$(date +%s)

    while [ 1 ] ; do
        height=$(get_height ${id})
        if [ $? -ne 0 ] ; then
            echo "failed to get_height: ${height}"
            return 1
        fi
        now=$(date +%s)
        if [ ${height} -gt ${target_height} ]; then
            echo "$((now-start)) s"
            return  0
        fi

        if [ $((now-start)) -gt ${timeout} ] ; then
            echo "timeout: $((now-start)) s"
            return 1
        fi
        sleep 1
    done
    return 1
}

check_height_stopped () {
    if [ $# -ne 1 ] ; then
        echo "usage: $0 node_id"
        return 1
    fi
    id=$1
    old=$(get_height ${id})
    if [ $? -ne 0 ] ; then
        echo "failed to get_height: ${old}"
        return 1
    fi
    sleep 3                     # 出块间隔
    new=$(get_height ${id})
    if [ $? -ne 0 ] ; then
        echo "failed to get_height: ${new}"
        return 1
    fi
    if [ $new -ne $old ]; then
        echo "height change from ${old} to ${new}"
        return 1
    fi
    return 0
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
