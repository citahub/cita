#!/bin/bash
category=1
if [ $# == 0 ]; then
    config="config_create.json";
elif [ $# == 1 ]; then
    config=$1;
elif [ $# == 2 ]; then
    config="$1";
    category=$2
else
    echo "args: ip (default localhost)"
    exit
fi
if [ $category == 1 ]; then
../../admintool/release/bin/trans_evm --config $config
else
../../admintool/release/bin/trans_evm --config $config
fi
