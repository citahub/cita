#!/bin/bash

CHAIN_NAME="test"

sudo() {
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        "$@"
    else
        /usr/bin/sudo "$@"
    fi
    set +o noglob
}

get_height() {
    local id=$1
    local timeout=$2

    if [ ! -n "$timeout" ]; then
        timeout=30
    fi

    start=$(date +%s)
    while true; do
        if height=$(blockNumber 127.0.0.1 $((1337+"${id}"))); then
            echo "${height}"
            return 0
        fi

        now=$(date +%s)
        if [ $((now-start-timeout)) -gt 0 ] ; then
            echo "timeout: ${timeout}"
            return 1
        fi
        sleep 1
    done
}

get_peer_count() {
    local id=$1
    local timeout=$2
    local peer_count
    local start
    local now

    if [ ! -n "$timeout" ]; then
        timeout=30
    fi

    start=$(date +%s)

    while true; do
        if peer_count=$(peerCount 127.0.0.1 $((1337+"${id}"))); then
            echo "${peer_count}"
            return 0
        fi

        now=$(date +%s)
        if [ $((now-start-timeout)) -gt 0 ] ; then
            echo "timeout: ${timeout}"
            return 1
        fi
        sleep 1
    done
}

check_height_growth() {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id old_height"
        return 1
    fi
    local id=$1
    local old=$2
    local new
    if new=$(get_height "${id}"); then
        if [ "${new}" -gt "${old}" ]; then
            echo "height growth"
            return 0
        fi
    fi
    return 1
}

# output information about time used if exit 0
check_peer_count() {
    if [ $# -ne 3 ] ; then
        echo "usage: $0 node_id expected_count timeout"
        return 1
    fi
    local id=$1
    local expected_count=$2
    local timeout=$3
    local start
    local peer_count
    local now

    start=$(date +%s)
    while true; do
        if peer_count=$(get_peer_count "${id}" "${timeout}"); then
            if [ $((peer_count)) -eq $((expected_count)) ]; then
                echo "$((now-start))"
                return 0
            fi
        fi
        now=$(date +%s)
        if [ $((now-start)) -gt "${timeout}" ] ; then
            echo "time used: $((now-start)) \
                get peer count: ${peer_count} \
                expected count: ${expected_count}"
            return 1
        fi
        sleep 1
    done
}

# output information about time used if exit 0
check_peer_count_max() {
    if [ $# -ne 3 ] ; then
        echo "usage: $0 node_id expected_count timeout"
        return 1
    fi
    local id=$1
    local max_count=$2
    local timeout=$3
    local start
    local peer_count
    local now

    start=$(date +%s)
    while true; do
        if peer_count=$(get_peer_count "${id}" "${timeout}"); then
            if [ $((peer_count)) -le $((max_count)) ]; then
                echo "$((now-start))"
                return 0
            fi
        fi
        now=$(date +%s)
        if [ $((now-start)) -gt "${timeout}" ] ; then
            echo "time used: $((now-start)) \
                get peer count: ${peer_count} \
                expected count: ${expected_count}"
            return 1
        fi
        sleep 1
    done
}

check_height_growth_normal() {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 id timeout"
        return 1
    fi

    local id=$1
    local timeout=$2
    local old
    local now

    if old=$(get_height "${id}"); then
        start=$(date +%s)
        while true; do
            if check_height_growth "${id}" "${old}"; then
                return 0
            fi
            now=$(date +%s)
            if [ $((now-start)) -gt "${timeout}" ] ; then
                echo "check_height_growth_normal failed id(${id}) timeout(${timeout})"
                return 1
            fi
            sleep 1
        done
    fi
    echo "failed to get old height"
    return 1
}

# output information about time used if exit 0
check_height_sync() {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id refer_node_id"
        return 1
    fi
    local id=$1
    local refer=$2
    local timeout=180
    local refer_height
    local now
    local height

    if refer_height=$(get_height "${refer}"); then
        start=$(date +%s)
        while true; do
            now=$(date +%s)
            if [ $((now-start)) -gt ${timeout} ] ; then
                echo "check_height_sync timeout(${timeout}) \
                    time used $((now-start)) \
                    refer height ${refer_height} \
                    sync height ${height}"
                return 1
            fi
            if height=$(get_height "${id}"); then
                if [ "${height}" -gt "${refer_height}" ]; then
                    echo "$((now-start))"
                    return  0
                fi
            fi
        done
    fi
    echo "check_height_sync failed to get_height(refer): ${refer_height}"
    return 1
}

check_height_stopped() {
    if [ $# -ne 2 ] ; then
        echo "usage: $0 node_id timeout"
        return 1
    fi
    local id=$1
    local timeout=$2
    local old
    local now
    local new

    if old=$(get_height "${id}"); then
        start=$(date +%s)
        while true; do
            now=$(date +%s)
            if [ $((now-start)) -gt "${timeout}" ] ; then
                echo "$((now-start))"
                return 0
            fi
            if new=$(get_height "${id}"); then
                if [ "$new" -gt "$old" ]; then
                    echo "check_height_stopped height change from ${old} to ${new}"
                    return 1
                fi
                sleep 1
                continue
            fi
        done
    fi
    echo "check_height_stopped failed to get_height(old): ${old}"
    return 1
}

set_delay_at_port() {
    if [ $# -ne 2 ] ; then
        echo "usage: set_delay_at_port port delay"
        return 1
    fi
    local port=$1
    local delay=$2

    # TODO: need more description
    sudo tc qdisc  add dev lo root        handle  1:  prio bands 4                                         >/dev/null 2>&1 || true
    sudo tc qdisc  add dev lo parent 1:4  handle 40:  netem delay "${delay}"ms                               >/dev/null 2>&1 || true
    sudo tc filter add dev lo protocol ip parent  1:0 prio 4 u32 match ip dport "${port}" 0xffff flowid 1:4  >/dev/null 2>&1 || true
}

unset_delay_at_port() {
    if [ $# -ne 1 ] ; then
        echo "usage: $0 port"
        return 1
    fi
    local port=$1
    #sudo tc filter del dev lo protocol ip parent  1:0 prio 4 u32 match ip dport ${port} 0xffff flowid 1:4  >/dev/null 2>&1 || true
    sudo tc qdisc del dev lo root> /dev/null 2>&1||true
}

blockNumber() {
    local ip=$1
    local port=$2
    local response
    local height

    if [ ! -n "$ip" ]; then
        ip="127.0.0.1"
    fi
    if [ ! -n "$port" ]; then
        port=1337
    fi
    if response=$(curl -s -X POST -d '{"jsonrpc":"2.0","method":"blockNumber","params":[],"id":2}' $ip:$port); then
        height=$(echo "${response}" |jq ".result"|sed 's/\"//g')
        if [ "$height" == null ]; then
           exit 1
        fi
        echo $((height))
        return 0
    fi
    echo "failed to get glock number"
    return 1
}

peerCount() {
    local ip=$1
    local port=$2
    local response
    local count

    if [ ! -n "$ip" ]; then
        ip="127.0.0.1"
    fi
    if [ ! -n "$port" ]; then
        port=1337
    fi
    if response=$(curl -s -X POST -d '{"jsonrpc":"2.0","method":"peerCount","params":[],"id":2}' $ip:$port); then
        count=$(echo "${response}" |jq ".result"|sed 's/\"//g')
        if [ "$count" == null ]; then
           exit 1
        fi
        echo $((count))
        return 0
    fi
    echo "failed to get peer count"
    return 1
}

start_nodes() {
    local num=$1
    if [ ! -n "$num" ]; then
        num=4
    fi
    for ((i=0; i<num; i++)); do
        bin/cita bebop setup $CHAIN_NAME/$i
    done
    for ((i=0; i<num; i++)); do
        bin/cita bebop start $CHAIN_NAME/$i trace
    done
}

config_script() {
    ./scripts/create_cita_config.py "$@"
}

create_config() {
    local param="create \
        --chain_name $CHAIN_NAME \
        --super_admin 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523 \
        --nodes 127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003 \
        $*"
    # shellcheck disable=SC2086
    config_script $param
}
