#!/bin/bash

readonly SOURCE_DIR="$(dirname "$(readlink -f "$0")")"
readonly COMMIT_ID=$(git rev-parse --short HEAD)
readonly CONTAINER_NAME="cita_build_${COMMIT_ID}"
readonly DOCKER_IMAGE='cita/cita-build:ubuntu-18.04-20191128'
readonly PUB_KEY_PATH="${HOME}/.ssh/id_rsa"

# Patch from crates.io.
readonly CARGO_HOME='/opt/.cargo'
readonly DOCKER_CARGO="${HOME}/.docker_cargo"
mkdir -p "${DOCKER_CARGO}/git"
mkdir -p "${DOCKER_CARGO}/registry"

# Docker Arguments
readonly WORKDIR='/opt/cita'
cp '/etc/localtime' "${SOURCE_DIR}/localtime"
readonly LOCALTIME_PATH="${SOURCE_DIR}/localtime"

# Ssh Agent
eval $(ssh-agent)
ssh-add

# Run Docker
echo "Start docker container ${CONTAINER_NAME} ..."
docker run --rm --init \
    --volume "${SOURCE_DIR}:${WORKDIR}" \
    --volume "${DOCKER_CARGO}/git:${CARGO_HOME}/git" \
    --volume "${DOCKER_CARGO}/registry:${CARGO_HOME}/registry" \
    --volume "${LOCALTIME_PATH}:/etc/localtime" \
    --volume "${PUB_KEY_PATH}:${PUB_KEY_PATH}" \
    --volume $(readlink -f $SSH_AUTH_SOCK):/ssh-agent \
    --env SSH_AUTH_SOCK=/ssh-agent \
    --workdir "${WORKDIR}" \
    --name "${CONTAINER_NAME}" \
    "${DOCKER_IMAGE}" \
    /bin/bash -c "$@"
