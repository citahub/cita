#!/bin/bash

SOURCE_DIR=$(realpath $(dirname $(realpath $0))/..)
cd ${SOURCE_DIR}

if [ $# -ne 1 ] ; then
    echo "usage: $0 debug|release"
    exit 1
fi
type=$1

# 0) setup
mkdir -p                                   target/install/scripts/
mkdir -p                                   target/install/bin/
mkdir -p                                   target/install/resource/

# 1) binary
for binary in \
        cita-auth \
        cita-bft \
        cita-chain \
        cita-executor \
        cita-forever \
        cita-jsonrpc \
        cita-network \
        \
        amqp_test \
        benchmark_ws \
        chain_performance \
        jsonrpc_performance \
        latency \
        trans_evm \
        \
        create_key_addr \
        cita-relayer-parser \
        snapshot_tool \
        ; do
    cp -rf "target/${type}/${binary}" target/install/bin/
done
#strip                                     target/install/bin/*

# 2) cita
cp -rf  scripts/cita                       target/install/bin/

# 3) contract
cp -rf scripts/contracts                   target/install/scripts/

# 4) admintool
mkdir -p                                   target/install/scripts/admintool
cp -rf  scripts/admintool/*.py             target/install/scripts/admintool/
cp -rf  scripts/admintool/*.md             target/install/scripts/admintool/
cp -rf  scripts/admintool/*.sh             target/install/scripts/admintool/
cp -rf  scripts/admintool/*.txt            target/install/scripts/admintool/
cp -rf  scripts/admintool/*.json           target/install/scripts/admintool/
cp -rf  scripts/admintool/*.toml           target/install/scripts/admintool/
if [[ `uname` == 'Darwin' ]]
then
    gln -srf target/install/scripts/admintool/admintool.sh target/install/bin/
else
    ln -srf target/install/scripts/admintool/admintool.sh target/install/bin/
fi


# 6) txtool
cp -rf scripts/txtool                      target/install/scripts/

# 7) docker env
cp -f scripts/env.sh                       target/install/
cp -f scripts/daemon.sh                    target/install/
