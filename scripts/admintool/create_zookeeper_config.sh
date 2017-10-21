#!/bin/bash
# create_zookeeper_config.sh ID PATH

if [ $# -ne 2 ]; then
    echo "usage: $0 ID NODE_PATH"
    exit 1
fi

id=$1
node_path=$2
port=`expr 2181 + $id`
file=$node_path/zookeeper.properties
zk_log_path=$node_path/zookeeper_log
echo "dataDir=$zk_log_path" > $file
echo "clientPort=$port" >> $file
echo "maxClientCnxns=0" >> $file
