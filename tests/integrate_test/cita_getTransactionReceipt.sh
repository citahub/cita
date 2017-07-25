#!/bin/bash
if [ $# == 1 ]; then
    HASH=$1
    IP="127.0.0.1";
elif [ $# == 2 ]; then
    HASH=$1
    IP=$2;
else
    echo "args: hash ip (default localhost) "
    exit
fi
curl -s -X POST -d '{"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":['\"$HASH\"'],"id":2}' $IP:1337
