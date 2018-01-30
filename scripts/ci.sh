#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-build:latest"

SCRIPT_PATH=$(readlink -f $0)
SOURCE_DIR=$(readlink -f "$(dirname ${SCRIPT_PATH})/..")

DEFAULT_HASH="sha3hash"
DEFAULT_CRYPT="secp256k1"

SELECT_HASH="${DEFAULT_HASH}"
SELECT_CRYPT="${DEFAULT_CRYPT}"

exec 2>&1

function draw_title () {
    printf "############################################################################\n"
    printf "##### %-64s #####\n" "$*"
    printf "############################################################################\n"
}

function show_version () {
    draw_title "Show Version"
    echo "[Info ] Git status as follow:"
    git status --short | while read line; do
        echo "    ${line}"
    done
    echo "[Info ] Git version is [$(git rev-parse HEAD)]."
}

function test_if_has_docker () {
    local docker_bin=$(which docker)
    if [ -z "${docker_bin}" ]; then
        echo "[Warn ] There is no docker."
        return 1
    else
        echo "[Info ] The docker is [${docker_bin}]."
    fi
    local image_id=$(docker images --quiet "${DOCKER_IMAGE}")
    if [ -z "${image_id}" ]; then
        echo "[Warn ] There is no image [${DOCKER_IMAGE}]."
        return 1
    else
        echo "[Info ] The image id is [${image_id}]."
    fi
    return 0
}

function run_in_docker () {
    draw_title "Run in Docker"
    # Only allocate tty if we detect one
    if [ -t 1 ]; then
        DOCKER_RUN_OPTIONS="-t"
    fi
    if [ -t 0 ]; then
        DOCKER_RUN_OPTIONS="${DOCKER_RUN_OPTIONS} -i"
    fi
    docker run --rm ${DOCKER_RUN_OPTIONS} \
           --env RUN_IN_DOCKER=1  \
           --volume ${SOURCE_DIR}:${SOURCE_DIR} \
           --volume ${HOME}/.cargo/registry:/root/.cargo/registry  \
           --volume ${HOME}/.cargo/git:/root/.cargo/git \
           --workdir "${SOURCE_DIR}" "${DOCKER_IMAGE}" "${SCRIPT_PATH}"
}

function run_in_machine () {
    draw_title "Run in Machine"
    draw_title "    1) Setup"
    source ${HOME}/.cargo/env
    scripts/config_rabbitmq.sh
    # For native machine, skip this step.
    # scripts/install_develop.sh
    draw_title "    2) Format"
    time make fmt
    draw_title "    3) Build"
    draw_title "        3.1) Build for Debug"
    time make debug
    draw_title "        3.2) Check Cargo.lock After Build"
    if [ $(git status --short Cargo.lock | wc -l) -ne 0 ]; then
        echo "[Error] Please update Cargo.lock BEFORE commit."
        exit 1
    fi
    draw_title "    4) Unit Test"
    time make test
    draw_title "    5) Integrate Test"
    draw_title "        5.1) Basic Test (contract create/call, node start/stop)"
    time ./tests/integrate_test/cita_basic.sh
    draw_title "        5.2) Byzantine Test"
    time ./tests/integrate_test/cita_byzantinetest.sh
}

function replace_default_feature () {
    local workspacedir="${1}"
    local old_feature="${2}"
    local new_feature="${3}"
    if [ "${old_feature}" = "${new_feature}" ]; then
        return
    fi
    draw_title "Replace [${old_feature}] by [${new_feature}]"
    local before_feature='[ \t]*default[ \t]*=[ \t]*\[.*\"'
    local after_feature='\".*'
    find "${workspacedir}" -mindepth 2 -name "Cargo.toml" \
            | xargs grep -l "^${before_feature}${old_feature}${after_feature}" \
            | while read cargotoml; do
        if [ -f "${cargotoml}" ]; then
            echo "[Info ] Replace [${old_feature}] by [${new_feature}] for [${cargotoml}] ..."
            sed -i "s/\(${before_feature}\)${old_feature}\(${after_feature}\)\$/\1${new_feature}\2/" "${cargotoml}"
        else
            echo "[Error] [${cargotoml}] is not a file."
        fi
    done
}

function replace_algorithm () {
    replace_default_feature "${SOURCE_DIR}" "${DEFAULT_HASH}" "${SELECT_HASH}"
    replace_default_feature "${SOURCE_DIR}" "${DEFAULT_CRYPT}" "${SELECT_CRYPT}"
}

function restore_algorithm () {
    replace_default_feature "${SOURCE_DIR}" "${SELECT_HASH}" "${DEFAULT_HASH}"
    replace_default_feature "${SOURCE_DIR}" "${SELECT_CRYPT}" "${DEFAULT_CRYPT}"
}

function show_usage () {
    local script_name="$(basename $0)"
    echo "
    Usage:
        ${script_name} sha3hash secp256k1
        ${script_name} blake2bhash ed25519
        ${script_name} sm3hash sm2
"
}

function check_args () {
    draw_title "Check Command Line Arguments"
    local select_hash="$1"
    local select_crypt="$2"
    if [ -z "${select_hash}" ]; then
        echo "[Info ] Use the default hash algorithm."
    elif [ "${select_hash}" = "sha3hash" ] \
            || [ "${select_hash}" = "blake2bhash" ] \
            || [ "${select_hash}" = "sm3hash" ]; then
        SELECT_HASH="${select_hash}"
        echo "[Info ] Select the hash algorithm [${SELECT_HASH}]."
    else
        echo "[ERROR] Unknown algorithm [${select_hash}]."
        show_usage
        exit 1
    fi
    if [ -z "${select_crypt}" ]; then
        echo "[Info ] Use the default crypt algorithm."
    elif [ "${select_crypt}" = "secp256k1" ] \
            || [ "${select_crypt}" = "ed25519" ] \
            || [ "${select_crypt}" = "sm2" ]; then
        SELECT_CRYPT="${select_crypt}"
        echo "[Info ] Select the hash algorithm [${SELECT_CRYPT}]."
    else
        echo "[ERROR] Unknown algorithm [${select_crypt}]."
        show_usage
        exit 1
    fi
}

function main () {
    check_args "$@"
    cd  "${SOURCE_DIR}"
    show_version
    test_if_has_docker
    has_docker=$?
    replace_algorithm
    set -e -o pipefail
    if [ ${has_docker} -eq 0 ]; then
        run_in_docker
    else
        run_in_machine
    fi
    set +e
    restore_algorithm
}

main "$@"
