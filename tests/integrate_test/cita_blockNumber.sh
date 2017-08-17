#!/bin/bash
#echo "args: ip port"
IP=$1
PORT=$2
if [ ! -n "$IP" ]; then
    IP="127.0.0.1"
fi
if [ ! -n "$PORT" ]; then
    PORT=1337
fi
h=$(curl -s -X POST -d '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":2}' $IP:$PORT | jq ".result" | sed 's/\"//g')
echo $((h))
