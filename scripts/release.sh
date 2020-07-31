#!/bin/bash

if [[ $(uname) == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath "$(dirname "$(realpath "$0")")"/..)
else
    SOURCE_DIR=$(readlink -f "$(dirname "$(realpath "$0")")"/..)
fi

cd "${SOURCE_DIR}" || exit

if [ $# -ne 2 ] ; then
    echo "usage: $0 x86|aarch64 debug|release"
    exit 1
fi

arch=$1
type=$2
if [ "${arch}" == "x86" ]; then
    install_dir=target/install
fi
if [ "${arch}" == "aarch64" ]; then
    install_dir=target/aarch64_install
fi

# 0) setup
mkdir -p                                   ${install_dir}/scripts/
mkdir -p                                   ${install_dir}/bin/
mkdir -p                                   ${install_dir}/resource/


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
        cita-relayer-parser \
        ; do
    if [ "${arch}" == "x86" ]; then
        cp -rf "target/${type}/${binary}" ${install_dir}/bin/
    fi
    if [ "${arch}" == "aarch64" ]; then
        cp -rf "target/aarch64-unknown-linux-gnu/${type}/${binary}" ${install_dir}/bin/
    fi
done

# 2) cita
cp -rf scripts/cita.sh                      ${install_dir}/bin/cita

# 3) contract
cp -rf scripts/contracts                   ${install_dir}/scripts/

# 4) config tool
cp -rf  scripts/config_tool                ${install_dir}/scripts/
cp -f   scripts/create_cita_config.py      ${install_dir}/scripts/

# 5) txtool
cp -rf scripts/txtool                      ${install_dir}/scripts/

# 6) docker env
cp -f  env.sh                              ${install_dir}/bin/cita-env
cp -f  scripts/cita_config.sh              ${install_dir}/bin/cita-config

# 7) amend info of system contract
cp -f scripts/amend_system_contracts.sh    ${install_dir}/scripts/
cp -f scripts/amend_system_contracts.py    ${install_dir}/scripts/

exit 0
