#!/usr/bin/env bash

if [ `uname` == 'Darwin' ]; then
    SYSTEM_NET="bridge"
    SOURCE_DIR="$(dirname $(realpath $0))"
else
    SYSTEM_NET="host"
    SOURCE_DIR="$(dirname $(readlink -f $0))"
fi

cp /etc/localtime ${SOURCE_DIR}/localtime
LOCALTIME_PATH="${SOURCE_DIR}/localtime"

test -f "${SOURCE_DIR}/CODE_OF_CONDUCT.md"
if [ $? -eq 0 ]; then
    CONTAINER_NAME="cita_build_container"
    DOCKER_IMAGE="cita/cita-build:ubuntu-18.04-20190304"
else
    CONTAINER_NAME="cita_run_container"
    DOCKER_IMAGE="cita/cita-run:ubuntu-18.04-20181009"
    SOURCE_DIR="$(dirname $SOURCE_DIR)"
fi

WORKDIR=/opt/cita
USER_ID=`id -u $USER`
USER_NAME="user"

[[ "${USER_ID}" = "0" ]] && USER_NAME="root"

# Init contanier's cargo, for logs.
CARGO_HOME=/opt/.cargo
DOCKER_CARGO=${HOME}/.docker_cargo
mkdir -p ${DOCKER_CARGO}/git
mkdir -p ${DOCKER_CARGO}/registry

timeout=3
target=www.google.com
ret_code=`curl -I -s --connect-timeout $timeout $target -w %{http_code} | tail -n1`
if [ "x$ret_code" != "x200" ]; then
    INIT_CMD="echo -e '[source.crates-io]\nregistry = \"https://github.com/rust-lang/crates.io-index\"\nreplace-with = \"ustc\"\n[source.ustc]\nregistry = \"https://mirrors.ustc.edu.cn/crates.io-index\"' | sudo tee /opt/.cargo/config;"
fi
INIT_CMD="${INIT_CMD} while true; do sleep 100;done"

# Port condition
EXPOSE="1337:1337"
if [ "$3" == "port" ]; then
    EXPOSE=${@:4}
    [[ "${EXPOSE}" = "" ]] && EXPOSE="1337:1337"
    docker container stop $CONTAINER_NAME > /dev/null 2>&1
    echo -e "\033[0;32mExpose ports: ${@}\033[0m"
fi

# Run container
docker ps | grep ${CONTAINER_NAME} > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1
    docker run -d \
           --net=${SYSTEM_NET} \
           --volume ${SOURCE_DIR}:${WORKDIR} \
           --volume ${DOCKER_CARGO}/git:${CARGO_HOME}/git \
           --volume ${DOCKER_CARGO}/registry:${CARGO_HOME}/registry \
           --volume ${LOCALTIME_PATH}:/etc/localtime \
           --env USER_ID=${USER_ID} \
           --workdir ${WORKDIR} \
           --name ${CONTAINER_NAME} \
           -p $EXPOSE ${DOCKER_IMAGE} \
           /bin/bash -c "${INIT_CMD}"
    # Wait entrypoint.sh to finish
    sleep 3
fi

test -t 1 && USE_TTY="-t"

# Start nodes outside container directly
# Runing Commands through `cita` command:
# $0=`realpath`, $1="bin/cita", $2="bebop",
# $3="command/--daemon", $4="config/command"
# Most OS delete $0 default, some linux not.
[[ "$3" == "start" ]] && set "${@:1:2}" "--daemon" "${@:3}"

# Condition `daemon` to run daemon.
if [ "$3" == "--daemon"  ]; then
    set "${@:1:2}" "${@:4}"
    docker exec -d ${CONTAINER_NAME} /usr/bin/gosu ${USER_NAME} "$@"
elif [ $# -gt 0 ]; then
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} /usr/bin/gosu ${USER_NAME} "$@"
else
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} \
        /bin/bash -c "stty cols $(tput cols) rows $(tput lines) && /usr/bin/gosu ${USER_NAME} /bin/bash"
fi
