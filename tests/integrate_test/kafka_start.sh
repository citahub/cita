#!/bin/bash

if [ $# -ne 1 ]; then
    echo "usage: $0 INSTALL_PATH"
    exit 1
fi

if [ -z "$KAFKA_HOME" ]; then
    echo "Please set KAFKA_HOME environment var first"
    exit 1
fi

install_path=$1
kafka_pid=.kafka_pid
for i in {0..3} ; do
    node_path=$install_path/node$i
    nohup $KAFKA_HOME/bin/zookeeper-server-start.sh $node_path/zookeeper.properties > /tmp/zookeeper$i.log 2>&1 &
    echo $! >> $node_path/$kafka_pid
done

sleep 5

for i in {0..3}; do
    node_path=$install_path/node$i
    nohup $KAFKA_HOME/bin/kafka-server-start.sh $node_path/kafka.properties > /tmp/kafka$i.log 2>&1 &
    echo $! >> $node_path/$kafka_pid
done
