#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-run:ubuntu-18.04-20181009"
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

RELEASE_DIR=`pwd`
CONTAINER_NAME="cita_run${RELEASE_DIR//\//_}"
WORKDIR=/opt/cita-run
USER_ID=`id -u $USER`
USER_NAME="user"

if [ "${USER_ID}" = "0" ]; then
    USER_NAME="root"
fi

docker ps | grep ${CONTAINER_NAME} > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1
    docker run -d \
           --net=host \
           --volume ${RELEASE_DIR}:${WORKDIR} \
           --volume ${LOCALTIME_PATH}:/etc/localtime \
           --env USER_ID=${USER_ID} \
           --workdir ${WORKDIR} \
           --name ${CONTAINER_NAME} ${DOCKER_IMAGE} \
           /bin/bash -c "while true;do sleep 100;done"
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
