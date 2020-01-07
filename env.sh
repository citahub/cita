#!/bin/bash
#
# Environments' scripts.

# OS
if [[ "$(uname)" == 'Darwin' ]]; then
    readonly SYSTEM_NET='bridge'
    SOURCE_DIR="$(dirname "$(realpath "$0")")"
else
    readonly SYSTEM_NET="host"
    SOURCE_DIR="$(dirname "$(readlink -f "$0")")"
fi

readonly CONTAINER_NAME_HASH=`echo ${SOURCE_DIR} | md5sum | cut -d " " -f 1`
if test -f "${SOURCE_DIR}/Cargo.toml"; then
    readonly CONTAINER_NAME="cita_build${CONTAINER_NAME_HASH}"
    readonly DOCKER_IMAGE='cita/cita-build:ubuntu-18.04-20191128'
else
    readonly CONTAINER_NAME="cita_run${CONTAINER_NAME_HASH}"
    readonly DOCKER_IMAGE='cita/cita-run:ubuntu-18.04-20191128'
    readonly SOURCE_DIR="$(dirname "$SOURCE_DIR")"
fi

# Patch from crates.io.
readonly CARGO_HOME='/opt/.cargo'
readonly DOCKER_CARGO="${HOME}/.docker_cargo"
mkdir -p "${DOCKER_CARGO}/git"
mkdir -p "${DOCKER_CARGO}/registry"

# Docker Port
EXPOSE='1337:1337'
if [[ "$3" == "port" ]]; then
    EXPOSE=( "${@:4}" )
    [[ "${EXPOSE[*]}" == "" ]] && EXPOSE=('1337:1337')
    docker container stop "${CONTAINER_NAME}" > /dev/null 2>&1
    echo -e "\033[0;32mExpose ports: ${EXPOSE[*]} \033[0m"
fi

# Expose parameter for docker needs something like "-p 1337:1337 -p 1338:1338", but not "-p 1337:1337 1338:1338"
EXPOSE_PARAM=()
for port in "${EXPOSE[@]}"; do
    EXPOSE_PARAM+=(-p "$port")
done

# Docker Arguments
USER_ID="$(id -u "$USER")"
readonly WORKDIR='/opt/cita'
USER_NAME='user'

cp '/etc/localtime' "${SOURCE_DIR}/localtime"
readonly LOCALTIME_PATH="${SOURCE_DIR}/localtime"
[[ "${USER_ID}" = '0' ]] && USER_NAME='root'

# test network and set init cmd
timeout=3
target=www.google.com
ret_code=`curl -I -s --connect-timeout $timeout $target -w %{http_code} | tail -n1`
if [ "x$ret_code" = "x200" ]; then
    readonly INIT_CMD="sleep infinity"
else
    readonly INIT_CMD="echo -e '[source.crates-io]\nreplace-with = \"rustcc\"\n[source.rustcc]\nregistry = \"https://code.aliyun.com/rustcc/crates.io-index.git\"' | sudo tee /opt/.cargo/config;sleep infinity"
fi

PUB_KEY_PATH="${HOME}/.ssh/id_rsa"
# Run Docker
if ! docker ps | grep "${CONTAINER_NAME}" > '/dev/null' 2>&1; then
    echo "Start docker container ${CONTAINER_NAME} ..."
    docker rm "${CONTAINER_NAME}" > '/dev/null' 2>&1

    eval $(ssh-agent)
    ssh-add
    docker run -d --init \
           --net="${SYSTEM_NET}" \
           --volume "${SOURCE_DIR}:${WORKDIR}" \
           --volume "${DOCKER_CARGO}/git:${CARGO_HOME}/git" \
           --volume "${DOCKER_CARGO}/registry:${CARGO_HOME}/registry" \
           --volume "${LOCALTIME_PATH}:/etc/localtime" \
           --volume "${PUB_KEY_PATH}:${PUB_KEY_PATH}" \
           --volume $(readlink -f $SSH_AUTH_SOCK):/ssh-agent \
           --env SSH_AUTH_SOCK=/ssh-agent \
           --env "USER_ID=${USER_ID}" \
           --workdir "${WORKDIR}" \
           --name "${CONTAINER_NAME}" \
           "${EXPOSE_PARAM[@]}" "${DOCKER_IMAGE}" \
           /bin/bash -c "${INIT_CMD}"
    # Wait entrypoint.sh to finish
    sleep 3
fi

# If running "cita port" command, need to exit, means the command have finished.
if [[ "$3" == "port" ]]; then
    exit 0
fi

# Run commands through docker container
# $0=`realpath`, $1="bin/cita", $2="bebop",
# $3="command/--daemon", $4="config/command"
test -t 1 && USE_TTY='-t'
[[ "$3" == 'start' ]] || [[ "$3" == 'restart' ]] && set "${@:1:2}" '--daemon' "${@:3}"

if [[ "$3" == '--daemon' ]]; then
    set "${@:1:2}" "${@:4}"
    # ATTENTION:
    # (docker commands) (parameters) /bin/bash -c (full-string docker arguments || sepreate them on by one)
    # `bin/bash -c` is local commands.
    docker exec -d "${CONTAINER_NAME}" /bin/bash -c "/usr/sbin/gosu ${USER_NAME} ${*} >/dev/null 2>&1"
elif [[ $# -gt 0 ]]; then
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} /bin/bash -c "/usr/sbin/gosu ${USER_NAME} ${*}"
else
    docker exec -i ${USE_TTY} ${CONTAINER_NAME} \
        /bin/bash -c "stty cols $(tput cols) rows $(tput lines) && /usr/sbin/gosu ${USER_NAME} /bin/bash"
fi
