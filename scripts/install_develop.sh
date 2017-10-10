#!/bin/sh
set -e

sudo (){
    cmd=$*
    if [ "$(whoami)" = "root" ]; then
        ${cmd}
    else
        /usr/bin/sudo ${cmd}
    fi
}

# 1) install add-apt-repository
sudo apt-get update -q
sudo apt-get install -y software-properties-common

# 2) add repositores
# 2.1) add libsodium repository if using trusty version; only for travis trusty build environment.
if [ $(lsb_release -s -c) = "trusty" ]; then
    sudo add-apt-repository -y ppa:chris-lea/libsodium;
fi;
# 2.2) add ethereum repository
sudo add-apt-repository -y ppa:ethereum/ethereum

# 3) install develop dependencies
sudo apt-get update -q
sudo apt-get install -y build-essential pkg-config rabbitmq-server python-pip solc curl jq  google-perftools capnproto \
        libsnappy-dev  libgoogle-perftools-dev   libsodium* libzmq3-dev \
        libssl-dev libgoogle-perftools-dev

# 4) install python package
sudo pip install --user ethereum==2.0.4 pysodium

# 5) extra
# 5.1) libgmssl
wget https://github.com/cryptape/GmSSL/releases/download/v1.0/libgmssl.so.1.0.0.gz
gzip -d libgmssl.so.1.0.0.gz
sudo mv libgmssl.so.1.0.0 /usr/local/lib/
sudo ln -srf /usr/local/lib/libgmssl.so.1.0.0 /usr/local/lib/libgmssl.so

# 6) install rust&rustfmt
# 6.1) rust
which cargo || (curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2017-08-04)
. ${HOME}/.cargo/env

# 6.2) rustfmt
which rustfmt|| cargo install --force --vers 0.9.0 rustfmt
