#!/bin/bash
if [ $# == 0 ]; then
config="../jsonrpc_performance/config_err_format.json"
analysis=0
else
config=$1
analysis=$2
fi
if [ $analysis == 0 ]; then
    ../../admintool/release/bin/jsonrpc_performance --config $config
else
    if [ $# -lt 4 ]; then
        echo "4 arguments are required"
        exit 1
    fi
    start_h=$3
    end_h=$4
    ../../admintool/release/bin/jsonrpc_performance --config $config --analysis=true --start_h=$start_h --end_h=$end_h
fi