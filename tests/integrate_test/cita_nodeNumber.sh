#!/bin/bash

IP=$1
PORT=$2
if [ ! -n "$IP" ]; then
    IP="127.0.0.1"
fi
if [ ! -n "$PORT" ]; then
    PORT=1337
fi
curl -s -X POST -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":2}' $IP:$PORT | jq ".result"