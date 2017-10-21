#!/bin/bash

if [ $# -ne 1 ]; then
    echo "usage: $0 INSTALL_PATH"
    exit 1
fi

install_path=$1
kafka_pid=.kafka_pid

stop_kafka() {
    id=$1
    file=$install_path/node$id/$kafka_pid
    if [ -e $file ]; then
        for pid in $(cat $file); do
            kill -9 $pid || true
        done
        rm -f $file
    fi
}

stop_all_kafka() {
    for id in {0..3}; do
        stop_kafka $id
    done
}

stop_all_kafka
