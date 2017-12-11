#!/bin/bash

SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
cd ${SOURCE_DIR}

if [ $# -ne 1 ] ; then
    echo "usage: $0 debug|release"
    exit 1
fi
type=$1

# 0) setup
mkdir -p                                   target/install/scripts/
mkdir -p                                   target/install/bin/

# 1) binary
cp -rf target/${type}/chain                target/install/bin/cita-chain
cp -rf target/${type}/consensus_tendermint target/install/bin/cita-consensus
cp -rf target/${type}/jsonrpc              target/install/bin/cita-jsonrpc
cp -rf target/${type}/auth                 target/install/bin/cita-auth
cp -rf target/${type}/network              target/install/bin/cita-network
cp -rf target/${type}/trans_evm            target/install/bin/
cp -rf target/${type}/create_key_addr      target/install/bin/
cp -rf target/${type}/chain_performance    target/install/bin/
cp -rf target/${type}/amqp_test            target/install/bin/
cp -rf target/${type}/jsonrpc_performance  target/install/bin/
cp -rf target/${type}/latency              target/install/bin/
cp -rf target/${type}/benchmark_ws         target/install/bin/
cp -rf target/${type}/monitor              target/install/bin/cita-monitor
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
ln -srf target/install/scripts/admintool/admintool.sh target/install/bin/

# 5) Dockerfile
cp -rf  scripts/Dockerfile-run             target/install/scripts/
cp -rf  scripts/Dockerfile                 target/install/scripts/
cp -rf  scripts/install_runtime.sh         target/install/scripts/
cp -rf  scripts/build_image_from_binary.sh target/install/scripts/
cp -rf  scripts/docker-compose.yaml        target/install/scripts/

# 6) txtool
cp -rf scripts/txtool                      target/install/scripts/
