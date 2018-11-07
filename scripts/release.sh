#!/bin/bash

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $(realpath $0))/..)
else
    SOURCE_DIR=$(readlink -f $(dirname $(realpath $0))/..)
fi

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
        create_key_addr \
        cita-relayer-parser \
        snapshot_tool \
        consensus-mock \
        chain-executor-mock \
        box_executor \
        bft-wal \
        ; do
    cp -rf "target/${type}/${binary}" target/install/bin/
done
#strip                                     target/install/bin/*

# 2) cita
cp -rf  scripts/cita                       target/install/bin/

# 3) contract
cp -rf scripts/contracts                   target/install/scripts/

# 4) config tool
cp -rf  scripts/config_tool                target/install/scripts/
cp -f   scripts/create_cita_config.py      target/install/scripts/

# 5) txtool
cp -rf scripts/txtool                      target/install/scripts/

# 6) docker env
cp -f scripts/env.sh                       target/install/
cp -f scripts/daemon.sh                    target/install/

# 7) amend info of system contract
cp -f scripts/amend_sys_cont_to_v0-20.py              target/install/scripts/
cp -f scripts/amend_sys_cont_to_v0-20.sh              target/install/scripts/
