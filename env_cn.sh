#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-build:ubuntu-18.04-20180523"
if [[ `uname` == 'Darwin' ]]
then
    cp /etc/localtime $PWD/localtime
    LOCALTIME_PATH="$PWD/localtime"
else
    LOCALTIME_PATH="/etc/localtime"
fi

docker_bin=$(which docker)
if [ -z "${docker_bin}" ]; then
    echo "Command not found, install docker first."
    exit 1
else
    docker version > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "Run docker version failed, Maybe docker service not running or current user not in docker user group."
        exit 2
    fi
fi

SOURCE_DIR=`pwd`
CONTAINER_NAME="cita_build${SOURCE_DIR//\//_}"
DOCKER_HOME=/opt
WORKDIR=${DOCKER_HOME}/cita

mkdir -p ${HOME}/.docker_cargo/git
mkdir -p ${HOME}/.docker_cargo/registry

docker ps | grep ${CONTAINER_NAME} > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "docker container ${CONTAINER_NAME} is already running"
else
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1
    docker run -d \
           --volume ${SOURCE_DIR}:${WORKDIR} \
           --volume ${HOME}/.docker_cargo/registry:${DOCKER_HOME}/.cargo/registry \
           --volume ${HOME}/.docker_cargo/git:${DOCKER_HOME}/.cargo/git \
           --volume ${LOCALTIME_PATH}:/etc/localtime \
           --env USER_ID=`id -u $USER` \
           --workdir ${WORKDIR} \
	         --name ${CONTAINER_NAME} ${DOCKER_IMAGE} \
           /bin/bash -c "echo -e '[source.crates-io]\nregistry = \"https://github.com/rust-lang/crates.io-index\"\nreplace-with = \"ustc\"\n[source.ustc]\nregistry = \"https://mirrors.ustc.edu.cn/crates.io-index\"'>~/.cargo/config;while true;do sleep 100;done"
    sleep 20
fi

test -t 1 && USE_TTY="-t"

if [ $# -gt 0 ]; then
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} /usr/bin/gosu user "$@"
else
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} \
        /bin/bash -c "stty cols $(tput cols) rows $(tput lines) && bash"
fi
