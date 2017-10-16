#!/bin/bash
set -e
sudo() {
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo $*
    fi
    set +o noglob
}

BINARY_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
cd ${BINARY_DIR}
if [ "$1" = "all" ] ; then
    cd scripts
    # build image for deploy environment
    sudo docker build -f Dockerfile-run   -t cryptape/cita-run .
    cd ..
fi

sudo docker build --tag cryptape/play --file scripts/Dockerfile .
