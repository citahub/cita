#!/bin/bash
set -e
#echo "args: ip port"
IP=$1
PORT=$2
if [ ! -n "$IP" ]; then
    IP="127.0.0.1"
fi
if [ ! -n "$PORT" ]; then
    PORT=1337
fi
response=$(curl -s -X POST -d '{"jsonrpc":"2.0","method":"blockNumber","params":[],"id":2}' $IP:$PORT)
if [ $? -ne 0 ]; then
    exit 1
fi

height=$(echo ${response}|jq ".result"|sed 's/\"//g')

if [ "$height" == null ]; then
   exit 1
fi

echo $((height))
