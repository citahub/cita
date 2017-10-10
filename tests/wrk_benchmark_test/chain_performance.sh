#!/bin/bash

PROJECT_DIR=$(readlink -f $(dirname $(readlink -f $0))/../..)

if [ $# == 0 ]; then
    category=1
    tx_num=300
    flag_prof_start=0
    flag_prof_duration=10
elif [ $# -lt 4 ]; then
    echo "4 arguments are required"
elif [ $# == 4 ]; then
    category=$1
    tx_num=$2
    flag_prof_start=$3
    flag_prof_duration=$4
fi
if [ $category == 1 ]; then
    ${PROJECT_DIR}/target/release/chain_performance --genesis ${PROJECT_DIR}/tests/wrk_benchmark_test/genesis.json \
                  --config ${PROJECT_DIR}/tests/wrk_benchmark_test/chain.json                                      \
                  --method create --tx_num=$tx_num                                                                 \
                  --flag_prof_start=$flag_prof_start --flag_prof_duration=$flag_prof_duration
elif [ $category == 2 ]; then
    ${PROJECT_DIR}/target/release/chain_performance --genesis ${PROJECT_DIR}/tests/wrk_benchmark_test/genesis.json \
                  --config ${PROJECT_DIR}/tests/wrk_benchmark_test/chain.json                                      \
                  --method call --tx_num=$tx_num                                                                   \
                  --flag_prof_start=$flag_prof_start --flag_prof_duration=$flag_prof_duration
elif [ $category == 3 ]; then
    ${PROJECT_DIR}/target/release/chain_performance --genesis ${PROJECT_DIR}/tests/wrk_benchmark_test/genesis.json \
                  --config ${PROJECT_DIR}/tests/wrk_benchmark_test/chain.json                                      \
                  --method store --tx_num=$tx_num                                                                  \
                  --flag_prof_start=$flag_prof_start --flag_prof_duration=$flag_prof_duration
fi
