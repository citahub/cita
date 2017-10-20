#!/bin/bash

SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

cd ${BINARY_DIR}/
rm -rf ${BINARY_DIR}/node*
rm -rf ${BINARY_DIR}/data
${BINARY_DIR}/bin/admintool.sh > /dev/null 2>&1

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
    ${BINARY_DIR}/bin/chain_performance --genesis ${BINARY_DIR}/node0/genesis.json                               \
                  --config ${BINARY_DIR}/node0/chain.json                                                        \
                  --method create --tx_num=$tx_num                                                               \
                  --flag_prof_start=$flag_prof_start --flag_prof_duration=$flag_prof_duration
elif [ $category == 2 ]; then
    ${BINARY_DIR}/bin/chain_performance --genesis ${BINARY_DIR}/node0/genesis.json                               \
                  --config ${BINARY_DIR}/node0/chain.json                                                        \
                  --method call --tx_num=$tx_num                                                                 \
                  --flag_prof_start=$flag_prof_start --flag_prof_duration=$flag_prof_duration
elif [ $category == 3 ]; then
    ${BINARY_DIR}/bin/chain_performance --genesis ${BINARY_DIR}/node0/genesis.json                               \
                  --config ${BINARY_DIR}/node0/chain.json                                                        \
                  --method store --tx_num=$tx_num                                                                \
                  --flag_prof_start=$flag_prof_start --flag_prof_duration=$flag_prof_duration
fi
