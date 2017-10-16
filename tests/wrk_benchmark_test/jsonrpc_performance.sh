#!/bin/bash
SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/../..)

tests/wrk_benchmark_test/jsonrpc_performance.sh
if [ $# == 0 ]; then
    config=${SOURCE_DIR}/tests/jsonrpc_performance/jsonrpc_performance/config_err_format.json
    analysis=0
else
    config=$1
    analysis=$2
fi
if [ $analysis == 0 ]; then
    ${SOURCE_DIR}/release/bin/jsonrpc_performance --config $config
else
    if [ $# -lt 4 ]; then
        echo "4 arguments are required"
        exit 1
    fi
    start_h=$3
    end_h=$4
    ${SOURCE_DIR}/release/bin/jsonrpc_performance --config $config  --analysis=true --start_h=$start_h --end_h=$end_h
fi
