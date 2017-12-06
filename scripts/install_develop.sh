#!/bin/bash
set -e

sudo(){
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo -H $*
    fi
    set +o noglob
}

# 1) install add-apt-repository
sudo apt-get update -q
sudo apt-get install -y software-properties-common

# 2) add repositores
# 2.1) add libsodium repository if using trusty version; only for travis trusty build environment.
if [ $(lsb_release -s -c) = "trusty" ]; then
    sudo add-apt-repository -y ppa:chris-lea/libsodium;
fi;

# 3) install develop dependencies
sudo apt-get update -q
sudo apt-get install -y build-essential pkg-config rabbitmq-server python-pip curl jq  google-perftools capnproto wget git \
     libsnappy-dev libgoogle-perftools-dev libsodium* libzmq3-dev libssl-dev cmake

# 3.1) install solc
wget https://github.com/ethereum/solidity/releases/download/v0.4.19/solidity_0.4.19.tar.gz
tar -xf solidity_0.4.19.tar.gz
./solidity_0.4.19/scripts/install_deps.sh
./solidity_0.4.19/scripts/build.sh

# 4) install python package
umask 022
sudo pip install ethereum==2.0.4 pysodium

# 5) extra
# 5.1) libgmssl
wget https://github.com/cryptape/GmSSL/releases/download/v1.0/libgmssl.so.1.0.0.gz
gzip -d libgmssl.so.1.0.0.gz
sudo mv libgmssl.so.1.0.0 /usr/local/lib/
sudo ln -srf /usr/local/lib/libgmssl.so.1.0.0 /usr/local/lib/libgmssl.so

# 6) install rust&rustfmt
# 6.1) rust
which cargo || (curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2017-12-05)
. ${HOME}/.cargo/env

# 6.2) rustfmt
which rustfmt|| cargo install --force --vers 0.2.17 rustfmt-nightly
