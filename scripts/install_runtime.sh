#!/bin/bash
set -e -o pipefail

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

# 2) add repositories
# 2.1) add libsodium repository if using trusty version; only for travis trusty build environment.
if [ $(lsb_release -s -c) = "trusty" ]; then
    sudo add-apt-repository -y ppa:chris-lea/libsodium
fi;

# 3) install runtime dependencies
sudo apt-get update -q
sudo apt-get install -y libstdc++6 rabbitmq-server libssl-dev libgoogle-perftools4 python-pip wget \
                        libsodium* libz3-dev cmake libz3-dev libboost-all-dev

# 3.1) install solc
wget https://github.com/ethereum/solidity/releases/download/v0.4.19/solc-static-linux
chmod +x solc-static-linux
sudo mv solc-static-linux /usr/local/bin/solc

# 4) install python package
umask 022
sudo pip install ethereum==2.2.0 pysodium

# 5) extra
# 5.1) libgmssl
wget https://github.com/cryptape/GmSSL/releases/download/v1.0/libgmssl.so.1.0.0.gz
gzip -d libgmssl.so.1.0.0.gz
sudo mv libgmssl.so.1.0.0 /usr/lib/
sudo ln -srf /usr/lib/libgmssl.so.1.0.0 /usr/lib/libgmssl.so
