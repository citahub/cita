#!/bin/bash
#
# Environments' scripts.

# OS
if [[ $(uname) == 'Darwin' ]]; then
    SYSTEM_NET="bridge"
    SOURCE_DIR="$(dirname "$(realpath "$0")")"
else
    SYSTEM_NET="host"
    SOURCE_DIR="$(dirname "$(readlink -f "$0")")"
fi

# Source Directory
test -f "$SOURCE_DIR"/Cargo.toml
ret="$?"
if [[ "$ret" -eq 0 ]]; then
    CONTAINER_NAME="cita_build_container"
    DOCKER_IMAGE="cita/cita-build:ubuntu-18.04-20190304"
else
    CONTAINER_NAME="cita_run_container"
    DOCKER_IMAGE="cita/cita-run:ubuntu-18.04-20181009"
    SOURCE_DIR="$(dirname "$SOURCE_DIR")"
fi

# Patch from crates.io.
CARGO_HOME=/opt/.cargo
DOCKER_CARGO="$HOME"/.docker_cargo
mkdir -p "$DOCKER_CARGO"/git
mkdir -p "$DOCKER_CARGO"/registry

# Docker Port
EXPOSE="1337:1337"
if [[ "$3" == "port" ]]; then
    EXPOSE="${*:4}"
    [[ "$EXPOSE" = "" ]] && EXPOSE="1337:1337"
    docker container stop $CONTAINER_NAME > /dev/null 2>&1
    echo -e "\033[0;32mExpose ports: ${*}\033[0m"
fi

# Docker Arguments
WORKDIR=/opt/cita
USER_ID="$(id -u "$USER")"
USER_NAME="user"

cp /etc/localtime "$SOURCE_DIR"/localtime
LOCALTIME_PATH="$SOURCE_DIR"/localtime
[[ "$USER_ID" = "0" ]] && USER_NAME="root"

INIT_CMD="sleep infinity"

# Run Docker
docker ps | grep "${CONTAINER_NAME}" > /dev/null 2>&1
ret="$?"
if [[ "$ret" -ne 0 ]]; then
    echo "Start docker container $CONTAINER_NAME ..."
    docker rm "$CONTAINER_NAME" > /dev/null 2>&1
    docker run -d \
           --net="$SYSTEM_NET" \
           --volume "$SOURCE_DIR":"$WORKDIR" \
           --volume "$DOCKER_CARGO"/git:"$CARGO_HOME"/git \
           --volume "$DOCKER_CARGO"/registry:"$CARGO_HOME"/registry \
           --volume "$LOCALTIME_PATH":/etc/localtime \
           --env USER_ID="$USER_ID" \
           --workdir "$WORKDIR" \
           --name "$CONTAINER_NAME" \
           -p $EXPOSE "$DOCKER_IMAGE" \
           /bin/bash -c "$INIT_CMD"
    # Wait entrypoint.sh to finish
    sleep 3
fi

# Run commands through docker container
# $0=`realpath`, $1="bin/cita", $2="bebop",
# $3="command/--daemon", $4="config/command"
test -t 1 && USE_TTY="-t"
[[ "$3" == "start" ]] || [[ "$3" == "restart" ]] && set "${@:1:2}" "--daemon" "${@:3}"

if [[ "$3" == "--daemon" ]]; then
    set "${@:1:2}" "${@:4}"
    docker exec -d ${CONTAINER_NAME} /bin/bash -c "/usr/bin/gosu ${USER_NAME} $* >/dev/null 2>&1"
elif [[ $# -gt 0 ]]; then
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} /usr/bin/gosu ${USER_NAME} "$@"
else
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} \
        /bin/bash -c "stty cols $(tput cols) rows $(tput lines) && /usr/bin/gosu ${USER_NAME} /bin/bash"
fi
