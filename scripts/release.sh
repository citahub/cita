#!/bin/bash

if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$(realpath "$0")")"/..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$(realpath "$0")")"/..)
fi

cd "${SOURCE_DIR}" || exit

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
        create-key-addr \
        create-genesis \
        ; do
    cp -rf "target/${type}/${binary}" target/install/bin/
done
#strip                                     target/install/bin/*

# 2) cita
cp -rf scripts/cita.sh                      target/install/bin/cita

# 3) contract
cp -rf scripts/contracts                   target/install/scripts/

# 4) config tool
cp -rf  scripts/config_tool                target/install/scripts/
cp -f   scripts/create_cita_config.py      target/install/scripts/

# 5) txtool
cp -rf scripts/txtool                      target/install/scripts/

# 6) docker env
cp -f  env.sh                              target/install/bin/cita-env
cp -f  scripts/cita_config.sh              target/install/bin/cita-config

# 7) amend info of system contract
cp -f scripts/amend_system_contracts.sh    target/install/scripts/
cp -f scripts/amend_system_contracts.py    target/install/scripts/

exit 0
