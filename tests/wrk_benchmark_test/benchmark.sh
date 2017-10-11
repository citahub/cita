#!/bin/bash
category=1
if [ $# == 0 ]; then
    config="config_create.json";
elif [ $# == 1 ]; then
    config=$1;
else
    echo "args: config_file"
    exit
fi

SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
BINARY_DIR=${SOURCE_DIR}/target/install

${BINARY_DIR}/bin/trans_evm --config $config
