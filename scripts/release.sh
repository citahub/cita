#!/bin/bash

PROJECT_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
if [ -z ${PROJECT_DIR} ] ; then
    echo "failed to locate project directory"
fi
cd ${PROJECT_DIR}

if [ $# -ne 1 ] ; then
    echo "usage: $0 debug|release"
fi
type=$1

# 0) setup
rm    -rf target/install
mkdir -p  target/install/scripts/
mkdir -p  target/install/bin/

# 1) binary
cp -rf target/${type}/chain                                        target/install/bin/
cp -rf target/${type}/consensus_tendermint                         target/install/bin/
cp -rf target/${type}/consensus_poa                                target/install/bin/
cp -rf target/${type}/raft                                         target/install/bin/
cp -rf target/${type}/txpool                                       target/install/bin/
cp -rf target/${type}/jsonrpc                                      target/install/bin/
cp -rf target/${type}/auth                                         target/install/bin/
cp -rf target/${type}/network                                      target/install/bin/
cp -rf target/${type}/trans_evm                                    target/install/bin/
cp -rf target/${type}/create_key_addr                              target/install/bin/
cp -rf target/${type}/chain_performance                            target/install/bin/
cp -rf target/${type}/amqp_test                                         target/install/bin/
cp -rf target/${type}/jsonrpc_performance                          target/install/bin/
cp -rf target/${type}/latency                                      target/install/bin/
#strip                                                              target/install/bin/*

# 2) cita
cp -rf  scripts/cita           						            target/install/scripts/

# 3) contract
cp -rf scripts/contracts                                        target/install/scripts/

# 4) admintool
mkdir -p  target/install/scripts/admintool
cp -rf  scripts/admintool/*.py           						target/install/scripts/admintool/
cp -rf  scripts/admintool/*.md           						target/install/scripts/admintool/
cp -rf  scripts/admintool/*.sh           						target/install/scripts/admintool/
cp -rf  scripts/admintool/*.txt          						target/install/scripts/admintool/
cp -rf  scripts/admintool/*.json          						target/install/scripts/admintool/
ln -srf target/install/scripts/admintool/admintool.sh           target/install/bin/
ln -srf target/install/scripts/cita                             target/install/bin/

# 5) Dockerfile
cp -rf  scripts/Dockerfile-run         						target/install/scripts/
cp -rf  scripts/Dockerfile           						target/install/scripts/
cp -rf  scripts/build_docker.sh         					target/install/scripts/
