#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-run:latest"

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

docker ps | grep ${CONTAINER_NAME} > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "docker container ${CONTAINER_NAME} is already running"
else
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1
    docker run -d --net=host --volume ${RELEASE_DIR}:${RELEASE_DIR} \
        --volume /etc/localtime:/etc/localtime \
        --workdir "${RELEASE_DIR}" --name ${CONTAINER_NAME} ${DOCKER_IMAGE} \
        /bin/bash -c "while true;do sleep 100;done"
    sleep 20
fi

CMD="$@"
if [ "${CMD}" = "" ]; then
    CMD="bash"
fi

test -t 1 && USE_TTY="-t"
docker exec -i ${USE_TTY} ${CONTAINER_NAME} ${CMD}