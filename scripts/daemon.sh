#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-run:ubuntu-18.04-20180523"

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
DOCKER_HOME=/opt
WORKDIR=${DOCKER_HOME}/cita-run

docker ps | grep ${CONTAINER_NAME} > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "docker container ${CONTAINER_NAME} is already running"
else
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1
    docker run -d \
           --net=host \
           --volume ${RELEASE_DIR}:${WORKDIR} \
           --volume ${LOCALTIME_PATH}:/etc/localtime \
           --env USER_ID=`id -u $USER` \
           --workdir ${WORKDIR} \
           --name ${CONTAINER_NAME} ${DOCKER_IMAGE} \
           /bin/bash -c "while true;do sleep 100;done"
    sleep 20
fi

docker exec -d ${CONTAINER_NAME} /usr/bin/gosu user "$@"
