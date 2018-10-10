#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-build:ubuntu-18.04-20181009"
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
CARGO_HOME=/opt/.cargo
WORKDIR=/opt/cita
USER_ID=`id -u $USER`
USER_NAME="user"

if [ "${USER_ID}" = "0" ]; then
    USER_NAME="root"
fi

mkdir -p ${HOME}/.docker_cargo/git
mkdir -p ${HOME}/.docker_cargo/registry

docker ps | grep ${CONTAINER_NAME} > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "docker container ${CONTAINER_NAME} is already running"
else
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1

    # test network and set init cmd
    timeout=3
    target=www.google.com
    ret_code=`curl -I -s --connect-timeout $timeout $target -w %{http_code} | tail -n1`
    if [ "x$ret_code" = "x200" ]; then
        INIT_CMD="while true;do sleep 100;done"
    else
        INIT_CMD="echo -e '[source.crates-io]\nregistry = \"https://github.com/rust-lang/crates.io-index\"\nreplace-with = \"ustc\"\n[source.ustc]\nregistry = \"https://mirrors.ustc.edu.cn/crates.io-index\"' | sudo tee /opt/.cargo/config;while true;do sleep 100;done"
    fi

    docker run -d \
           --volume ${SOURCE_DIR}:${WORKDIR} \
           --volume ${HOME}/.docker_cargo/registry:${CARGO_HOME}/registry \
           --volume ${HOME}/.docker_cargo/git:${CARGO_HOME}/git \
           --volume ${LOCALTIME_PATH}:/etc/localtime \
           --env USER_ID=${USER_ID} \
           --workdir ${WORKDIR} \
           --name ${CONTAINER_NAME} ${DOCKER_IMAGE} \
           /bin/bash -c "${INIT_CMD}"
    # Wait entrypoint.sh to finish
    sleep 3
fi

test -t 1 && USE_TTY="-t"

if [ $# -gt 0 ]; then
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} /usr/bin/gosu ${USER_NAME} "$@"
else
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} \
        /bin/bash -c "stty cols $(tput cols) rows $(tput lines) && /usr/bin/gosu ${USER_NAME} /bin/bash"
fi
