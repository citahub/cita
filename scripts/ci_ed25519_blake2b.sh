#!/bin/bash

IMAGE="cryptape/cita-build"
docker images 2> /dev/null | grep $IMAGE > /dev/null 2>&1
if [ $? == 0 ]; then
    # Only allocate tty if we detect one
    if [ -t 1 ]; then
        DOCKER_RUN_OPTIONS="-t"
    fi
    if [ -t 0 ]; then
        DOCKER_RUN_OPTIONS="$DOCKER_RUN_OPTIONS -i"
    fi

    echo "Found docker image $IMAGE"
    echo "Will run ci in docker"

    SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
    cd  ${SOURCE_DIR}
    git status
    git rev-parse HEAD

    SCRIPT_PATH=`readlink -f $0`
    docker run --rm $DOCKER_RUN_OPTIONS --env RUN_IN_DOCKER=1  -v $(pwd):$(pwd) -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git -w "$(pwd)" $IMAGE "$SCRIPT_PATH"
else
    if [ ! $RUN_IN_DOCKER ]; then
        SOURCE_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
        cd  ${SOURCE_DIR}
        git status
        git rev-parse HEAD
    fi

    set -e

    source ~/.cargo/env

    echo "Switch to ed25519 and blake2bhash"
    sed -i 's/\["secp256k1"\]/\["ed25519"\]/g' share_libs/crypto/Cargo.toml
    sed -i 's/\["sha3hash"\]/\["blake2bhash"\]/g' share_libs/util/Cargo.toml

    echo "################################################################################"
    echo "1) setup"
    scripts/config_rabbitmq.sh
    # For native machine, skip this step.
    # scripts/install_develop.sh

    echo "################################################################################"
    echo "2) format"
    time make fmt

    echo "################################################################################"
    echo "3) build"
    time make debug

    echo "################################################################################"
    echo "4) unit test"
    time make test

    echo "################################################################################"
    echo "5) integrate test"
    echo "5.1) basic test(contract create/call, node start/stop)"
    time ./tests/integrate_test/cita_basic.sh
    echo "5.2) byzantine test"
    time ./tests/integrate_test/cita_byzantinetest.sh

    sed -i 's/\["ed25519"\]/\["secp256k1"\]/g' share_libs/crypto/Cargo.toml
    sed -i 's/\["blake2bhash"\]/\["sha3hash"\]/g' share_libs/util/Cargo.toml
fi