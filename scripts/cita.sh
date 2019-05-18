#!/bin/bash
# -*- tab-width:4;indent-tabs-mode:nil -*-
# ex: ts=4 sw=4 et

# Commands Paths
if [[ $(uname) == 'Darwin' ]]; then
    CITA_BIN=$(dirname "$(realpath "$0")")
else
    CITA_BIN=$(dirname "$(readlink -f "$0")")
fi
CITA_SCRIPTS=$(dirname "$CITA_BIN")/scripts

if [ "$1" != "bebop" ]; then
    if stat "$CITA_BIN"/cita-env > /dev/null 2>&1; then
        "$CITA_BIN"/cita-env bin/cita bebop "$@"
    else
        echo -e "\033[0;31mPlease run this command after build 🎨"
        echo -e "\033[0;32mRun \`cita bebop\` to preview help! 🎸 \033[0m\n"
    fi
    exit 0
fi

# Delete the verbose parameters
set -- "${@:2}"

# Exit immediately if a command exits with a non-zero status
set -e

# Add cita scripts into system executable paths
export PATH=$CITA_BIN:$PATH
SERVICES=( forever auth bft chain executor jsonrpc network )
SCRIPT=$(basename "$0")
COMMAND=$1
NODE_NAME=$2
NODE_PATH="$(dirname "$CITA_BIN")/${NODE_NAME}"
NODE_LOGS_DIR="${NODE_PATH}/logs"
NODE_DATA_DIR="${NODE_PATH}/data"
TNODE=$(echo "${NODE_NAME}" | sed 's/\//%2f/g')

sudo(){
    set -o noglob

    if [ "$(whoami)" == "root" ] ; then
        "$@"
    else
        /usr/bin/sudo "$@"
    fi
    set +o noglob
}

usage() {
    cat <<EOF
Usage: $SCRIPT <command> <node> [options]
where <command> is one of the following:
    { help | create | append | port | setup | start | stop
      restart ping | top | backup | clean | logs | logrotate }
Run \`$SCRIPT help\` for more detailed information.
EOF
}


# INFORMATIONAL COMMANDS
help() {
    cat <<EOF
Usage: $SCRIPT <command> <node> [options]
This is the primary script for controlling the $SCRIPT node.
 INFORMATIONAL COMMANDS
    help
        You are here.
 BUILDING COMMANDS
    create <config>
        Creates blockchains according to the following config,
        use "cita create -h" to get more information.
        "cita-config" has the same function.
    append <config>
        Append a node into an existed chain,
        use "cita append -h" to get more information.
        "cita-config" has the same function.
    port <ports>
        Sets docker port, for example: "cita port 1337:1337 1338:1338",
        expose docker port 1337 and 1338.
 SERVICE CONTROL COMMANDS
    setup <node>
        Ensuring the required runtime environment for $SCRIPT node, like
        RabbitMQ service. You should run this command at the first time
        of running $SCRIPT node.
    start <node>
        Starts the $SCRIPT node in the background. If the node is already
        started, you will get the message "Node is already running!" If the
        node is not already running, no output will be given.
    stop <node> [debug] [mock]
        Stops the running $SCRIPT node. Prints "ok" when successful.  When
        the node is already stopped or not responding, prints:
        "Node 'NODE_NAME' not responding to pings."
    restart <node>
        Stops and then starts the running $SCRIPT node. Prints "ok"
        when successful.  When the node is already stopped or not
        responding, prints: "Node 'NODE_NAME' not responding to
        pings."
 DIAGNOSTIC COMMANDS
    ping <node>
        Checks that the $SCRIPT node is running. Prints "pong" when
        successful.  When the node is stopped or not responding, prints:
        "Node 'NODE_NAME' not responding to pings."
    top <node>
        Prints services processes information similar
        to the information provided by the \`top\` command.
    stat <node> (deprecated, use 'top' instead)
    logs <node> <service>
        Fetch the logs of the specified service.
 SCRIPTING COMMANDS
    backup <node>
        Backup the node's data and logs into backup directory, which actually
        copy that data and logs into backup directory. Prints the specified
        backup commands. When the node is running, prints:
        "Node is already running!"
    clean <node>
        Clean the node's data and logs, which actually move that data and logs
        into backup directory. Prints the specified backup commands. When the
        node is running, prints: "Node is already running!"
    logrotate <node>
        Archives the current node logs, starts fresh logs. Prints the archived
        logs path.
EOF

}


# BUILDING COMMANDS
create() {
    "$CITA_SCRIPTS"/create_cita_config.py "$@"
}

append() {
    "$CITA_SCRIPTS"/create_cita_config.py "$@"
}

# SERVICE CONTROL COMMANDS
start_rabbitmq() {
    # Config and start RabbitMQ
    if [[ $(uname) == 'Darwin' ]]
    then
        pgrep -f rabbitmq-server > /dev/null || brew services restart rabbitmq > /dev/null
        RABBITMQ_USER=cita_monitor
        RABBITMQ_PASSWD=cita_monitor
        sudo rabbitmqctl list_vhosts | grep "${NODE_NAME}" > /dev/null || sudo rabbitmqctl add_vhost "${NODE_NAME}" > /dev/null
        sudo rabbitmqctl set_permissions -p "${NODE_NAME}" guest '.*' '.*' '.*'       > /dev/null
        sudo rabbitmq-plugins enable rabbitmq_management                       > /dev/null
        sudo rabbitmqctl  list_users | grep ${RABBITMQ_USER}                   > /dev/null || sudo rabbitmqctl add_user ${RABBITMQ_USER} ${RABBITMQ_PASSWD} > /dev/null
        sudo rabbitmqctl  set_user_tags  ${RABBITMQ_USER} monitoring           > /dev/null
        sudo rabbitmqctl set_permissions -p "${NODE_NAME}"  ${RABBITMQ_USER} '.*' '.*' '.*'      > /dev/null
    else
        flock -x -w 30 /tmp/rabbitmq.lock -c "ps -C rabbitmq-server > /dev/null || sudo /etc/init.d/rabbitmq-server restart > /dev/null"
        RABBITMQ_USER=cita_monitor
        RABBITMQ_PASSWD=cita_monitor
        flock -x -w 30 /tmp/rabbitmq.lock -c "sudo rabbitmqctl list_vhosts | grep ${NODE_NAME} > /dev/null || sudo rabbitmqctl add_vhost ${NODE_NAME} > /dev/null"
        flock -x -w 30 /tmp/rabbitmq.lock -c "sudo rabbitmqctl set_permissions -p ${NODE_NAME} guest '.*' '.*' '.*' > /dev/null"
        flock -x -w 30 /tmp/rabbitmq.lock -c "sudo rabbitmq-plugins enable rabbitmq_management > /dev/null"
        flock -x -w 30 /tmp/rabbitmq.lock -c "sudo rabbitmqctl  list_users | grep ${RABBITMQ_USER} > /dev/null || sudo rabbitmqctl add_user ${RABBITMQ_USER} ${RABBITMQ_PASSWD} > /dev/null"
        flock -x -w 30 /tmp/rabbitmq.lock -c "sudo rabbitmqctl  set_user_tags  ${RABBITMQ_USER} monitoring > /dev/null"
        flock -x -w 30 /tmp/rabbitmq.lock -c "sudo rabbitmqctl set_permissions -p ${NODE_NAME}  ${RABBITMQ_USER} '.*' '.*' '.*' > /dev/null"
    fi
}

do_setup() {
    for i in {1..3}
    do
        start_rabbitmq
        if curl http://localhost:15672/ > /dev/null 2>&1; then
            return 0
        fi
    done
    echo "Failed to start RabbitMQ after $i times."
    exit 1
}


do_start() {
    debug=$1
    mock=$2

    # Make sure log directory exists
    mkdir -p "${NODE_LOGS_DIR}"

    # Tricky
    if [[ -z ${mock} ]]; then
        config="${NODE_PATH}/forever.toml"
    else
        config="${NODE_PATH}/forever_mock.toml"
    fi

    # Start cita-forever
    if [ -z "${debug}" ]; then
        cita-forever -c "${config}" start 2>&1
    else
        RUST_LOG=cita_auth=${debug},cita_chain=${debug},cita_executor=${debug},cita_jsonrpc=${debug},cita_network=${debug},cita_bft=${debug},\
core_executor=${debug},engine=${debug},jsonrpc_types=${debug},libproto=${debug},proof=${debug},txpool=${debug},core=${debug} \
        cita-forever -c "${config}" start 2>&1
    fi

    # Wait for the node to come up
    WAIT=3
    while [ $WAIT -gt 0 ]; do
        WAIT="$(( WAIT - 1 ))"
        sleep 1
        do_ping
        if [ "${PING_STATUS}" == "pong" ]; then
            echo "start...ok"
            exit 0
        fi
    done
    echo "Failed to start within 3 seconds,"
    echo "See ${NODE_LOGS_DIR}/cita-forever.log for detail"
    exit 1
}

do_stop() {
    cita-forever stop > /dev/null 2>&1

    # Make sure node stopped
    do_ping
    if [ "${PING_STATUS}" == "pong" ]; then
        echo "Failed to stop,"
        echo "See ${NODE_LOGS_DIR}/cita-forever.log for detail"
        exit 1
    fi

    echo "stop...ok"
}


# DIAGNOSTIC COMMANDS
PING_STATUS=""
do_ping() {
    pidfile="${NODE_PATH}/.cita-forever.pid"
    if [[ ! -e "$pidfile" ]]; then
        PING_STATUS="pang"
        return
    fi

    alive=$(ps -p "$(cat "${pidfile}")" | wc -l)
    if [ "${alive}" -le "1" ]; then
        PING_STATUS="pang"
        return
    fi

    PING_STATUS="pong"
}

do_top() {
    for service in "${SERVICES[@]}"; do
        pidfile="${NODE_PATH}/.cita-${service}.pid"
        if [ -e "$pidfile" ]; then
            ps -p "$(cat "${pidfile}")" -f | tail -n +2
        fi
    done
}

do_status() {
    while IFS= read -r -d '' pid_file; do
        pid=$(cat "${pid_file}")
        pgrep -f "${pid}" || true
    done <  <(find . -name "*.pid")
}


# SCRIPTING COMMANDS
do_clean() {
    # Clean empty node always successfully
    if [[ ! -d ${NODE_DATA_DIR} || ! -d ${NODE_LOGS_DIR} ]]; then
        echo "Node ${NODE_NAME} has no data and logs directories"
        exit 0
    fi

    # Move data/ and logs/ into backup directory
    backup_dir=$(pwd)/backup.$(date -Iseconds)
    mkdir -p "${backup_dir}"
    if [ -e "${NODE_DATA_DIR}" ] ; then
        echo "mv ${NODE_DATA_DIR} ${backup_dir}"
        mv "${NODE_DATA_DIR}" "${backup_dir}"
    fi
    if [ -e "${NODE_LOGS_DIR}" ] ; then
        echo "mv ${NODE_LOGS_DIR} ${backup_dir}"
        mv "${NODE_LOGS_DIR}" "${backup_dir}"
    fi
}

do_backup() {
    # Backup empty node always successfully
    if [[ ! -d ${NODE_DATA_DIR} || ! -d ${NODE_LOGS_DIR} ]]; then
        echo "Node ${NODE_NAME} has no data and logs directories"
        exit 0
    fi

    # Copy data/ and logs/ into backup directory
    backup_dir="$(pwd)/backup.$(date -Iseconds)"
    mkdir -p "${backup_dir}"
    if [ -e "${NODE_DATA_DIR}" ] ; then
        echo "cp -r ${NODE_DATA_DIR} ${backup_dir}/"
        cp -r "${NODE_DATA_DIR}" "${backup_dir}"/
    fi
    if [ -e "${NODE_LOGS_DIR}" ] ; then
        echo "cp -r ${NODE_LOGS_DIR} ${backup_dir}/"
        cp -r "${NODE_LOGS_DIR}" "${backup_dir}"/
    fi
}

do_logs() {
    service0=$1
    if [ -z "${service0}" ]; then
        echo "'${SCRIPT} logs' requires exactly 2 arguments."
        echo
        echo "Usage:  ${SCRIPT} logs NODE_NAME SERVICE"
        echo
        exit 1
    fi

    for service in "${SERVICES[@]}"; do
        if [[ $service = "$service0" || cita-"${service}" = "$service0" ]]; then
            tail -f "${NODE_LOGS_DIR}/cita-${service}.log"
            exit 0
        fi
    done

    echo "No such service: ${service0}"
    exit 1
}

do_logrotate() {
    logs=$(ls -1 "${NODE_LOGS_DIR}"/cita-*.log)
    cita-forever logrotate > /dev/null 2>&1

    # Wait for services to rotate their logs
    sleep 2
    for logfile in ${NODE_LOGS_DIR}/cita-*.log; do
        if [[ ${logs} != *"${logfile}"* ]]; then
            echo "./${NODE_NAME}/logs/${logfile##*/}"
        fi
    done
}

clear_rabbit_mq() {
    MQ_COMMAND="curl -i -u guest:guest -H content-type:application/json -XDELETE http://localhost:15672/api/queues/${TNODE}"

    "$MQ_COMMAND"/auth              > /dev/null 2>&1 || true
    "$MQ_COMMAND"/chain             > /dev/null 2>&1 || true
    "$MQ_COMMAND"/consensus         > /dev/null 2>&1 || true
    "$MQ_COMMAND"/jsonrpc           > /dev/null 2>&1 || true
    "$MQ_COMMAND"/network           > /dev/null 2>&1 || true
    "$MQ_COMMAND"/network_tx        > /dev/null 2>&1 || true
    "$MQ_COMMAND"/network_consensus > /dev/null 2>&1 || true
    "$MQ_COMMAND"/executor          > /dev/null 2>&1 || true
}

node_down_check() {
    do_ping
    if [ "${PING_STATUS}" == "pong" ]; then
        echo "Node is already running!"
        exit 1
    fi
}

node_up_check() {
    do_ping
    if [ "${PING_STATUS}" == "pang" ]; then
        echo "Node '${NODE_NAME}' not responding to pings"
        exit 1
    fi
}

# Commands not depend on $NODE_PATH
case "${COMMAND}" in
    help)
        help
        exit 0
        ;;

    usage)
        usage
        exit 0
        ;;

    create)
        create "$@"
        exit 0
        ;;

    append)
        append "$@"
        exit 0
        ;;

esac

if [ $# -lt 2 ]; then
    usage
    exit 1
fi

# Make sure the node directory exists
if [ ! -d "${NODE_PATH}" ]; then
    echo "No such node directory: ${NODE_NAME}"
    exit 1
elif [[ ! -e "${NODE_PATH}/forever.toml" && ! -e "${NODE_PATH}/forever_mock.toml" ]]; then
    echo "'${NODE_NAME}' is not a ${SCRIPT} node directory"
    exit 1
fi

# Enter the node directory
pushd . > /dev/null
cd "${NODE_PATH}"

case "${COMMAND}" in
    setup)
        do_setup
        ;;

    start)
        do_stop      # TODO: should not do so, but present tests need this
        node_down_check

        # Make sure the RabbitMQ fresh
        clear_rabbit_mq

        do_start "$3" "$4"
        ;;

    stop)
        node_up_check
        do_stop
        ;;

    restart)
        node_up_check
        do_stop

        # Make sure the RabbitMQ fresh
        clear_rabbit_mq

        do_start "$3" "$4"
        ;;

    ping)
        do_ping
        if [ "${PING_STATUS}" == "pong" ]; then
            echo "pong"
        else
            echo "Node '${NODE_NAME}' not responding to pings."
            exit 1
        fi
        ;;

    top)
        node_up_check
        do_top
        ;;

    stat)   # deprecated, use 'top' instead
        node_up_check
        do_top
        ;;

    status) # similar to 'top', but ... ?
        do_status
        ;;

    logrotate)
        do_logrotate
        ;;

    logs)
        do_logs "$3"
        ;;

    backup)
        node_down_check
        do_backup
        ;;

    clean)
        node_down_check
        do_clean
        ;;

    help)
        help
        ;;

    *)
        usage
        ;;
esac
popd > /dev/null

exit 0
