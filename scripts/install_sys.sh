#!/bin/sh
set -e

# 1) install add-apt-repository
apt-get update -q
apt-get install -y software-properties-common

# 2) add libsodium repository if using trusty version; only for travis trusty build environment.
if [ $(lsb_release -s -c) = "trusty" ]; then
    add-apt-repository -y ppa:chris-lea/libsodium;
fi;

# 3) add ethereum repository
add-apt-repository -y ppa:ethereum/ethereum

# 4) install dependencies
apt-get update -q
# 4.1) deploy dependencies: synch with scripts/Dockerfile-run if modified.
apt-get install -y libstdc++6 libsodium18 rabbitmq-server libssl-dev libgoogle-perftools4 python-pip
# 4.2) development dependencies
apt-get install -y build-essential pkg-config                              \
        libsnappy-dev  libgoogle-perftools-dev   libsodium-dev libzmq3-dev \
		solc curl jq  google-perftools capnproto
