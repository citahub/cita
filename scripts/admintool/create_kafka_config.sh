#!/bin/bash
# create_kafka_config.sh ID PATH

if [ $# -ne 2 ]; then
    echo "usage: $0 ID NODE_PATH"
    exit 1
fi

id=$1
node_path=$2
kafka_port=`expr 9092 + $id`
zk_port=`expr 2181 + $id`
file=$node_path/kafka.properties
kafka_log_path=$node_path/kafka_log/
rm -rf $file
echo "broker.id=$id" > $file
echo "listeners=PLAINTEXT://localhost:$kafka_port" >> $file
echo "num.network.threads=16" >> $file
echo "num.io.threads=16" >> $file
echo "socket.send.buffer.bytes=1000000" >> $file
echo "socket.receive.buffer.bytes=1000000" >> $file
echo "socket.request.max.bytes=104857600" >> $file
echo "log.dirs=$kafka_log_path" >> $file
echo "num.partitions=1" >> $file
echo "num.recovery.threads.per.data.dir=1" >> $file
echo "log.retention.hours=168" >> $file
echo "log.segment.bytes=1073741824" >> $file
echo "log.retention.check.interval.ms=600000" >> $file
echo "zookeeper.connect=localhost:$zk_port" >> $file
echo "zookeeper.connection.timeout.ms=6000" >> $file
