#!/bin/bash
if [ $# == 1 ]; then
    HEIGHT=$1
    IP="127.0.0.1";
elif [ $# == 2 ]; then
    HEIGHT=$1
    IP=$2;
else
    echo "args: height ip (default localhost) "
    exit
fi
curl -s -X POST -d '{"jsonrpc":"2.0","method":"getBlockByNumber","params":["'$HEIGHT'",false],"id":2}' $IP:1337 | jq ".result.body.transactions | length"
