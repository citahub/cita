#!/bin/bash
set +e
CUR_PATH=$(cd `dirname $0`; pwd)

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

stop_all
echo "###Stop CITA"
echo `date`
exit 0

