#!/usr/bin/env bash

DOCKER_IMAGE="cita/cita-build:ubuntu-18.04-20181009"

if [[ `uname` == 'Darwin' ]]
then
    SCRIPT_PATH=$(realpath $0)
    SOURCE_DIR=$(realpath "$(dirname ${SCRIPT_PATH})/..")
else
    SCRIPT_PATH=$(readlink -f $0)
    SOURCE_DIR=$(readlink -f "$(dirname ${SCRIPT_PATH})/..")
fi

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
    local cargo_home=/opt/.cargo
    local workdir=/opt/cita
    docker run --rm ${DOCKER_RUN_OPTIONS} \
           --volume ${SOURCE_DIR}:${workdir} \
           --volume ${HOME}/.docker_cargo/registry:${cargo_home}/registry \
           --volume ${HOME}/.docker_cargo/git:${cargo_home}/git \
           --volume /etc/localtime:/etc/localtime \
           --env USER_ID=`id -u $USER` \
           --workdir ${workdir} \
           "${DOCKER_IMAGE}" ./scripts/ci.sh
}

function run_in_machine () {
    draw_title "Run in Machine"
    draw_title "    1) Setup"
    local cargo_home=${CARGO_HOME}
    if [ ! -d "${cargo_home}" ]; then
        cargo_home=${HOME}/.cargo
    fi
    source ${cargo_home}/env
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
    draw_title "        5.2.1) Byzantine Test (EconomicalModel = Quota)"
    time ./tests/integrate_test/cita_byzantinetest.sh quota
    draw_title "        5.2.2) Byzantine Test (EconomicalModel = Charge)"
    time ./tests/integrate_test/cita_byzantinetest.sh charge
    if [ "${DEFAULT_HASH}" = "${SELECT_HASH}" ] \
            && [ "${DEFAULT_CRYPT}" = "${SELECT_CRYPT}" ]; then
        draw_title "        5.3.1) JSONRPC schema mock test (EconomicalModel = Quota)"
        time ./tests/integrate_test/cita_jsonrpc_schema_mock.sh quota
    # TODO: We should add it back later
        # draw_title "        5.3.2) JSONRPC schema mock test (EconomicalModel = Charge)"
        # time ./tests/integrate_test/cita_jsonrpc_schema_mock.sh charge
        draw_title "        5.4) Crosschain transaction test"
        time ./tests/integrate_test/cita_crosschain.sh
        draw_title "        5.5) EconomicalModel = Charge transfer value tests"
        time ./tests/integrate_test/cita_charge_mode.sh
    else
        echo "[Info ] Skip JSONRPC schema mock test."
        echo "[Info ] Skip Crosschain transaction test."
    fi
}

function replace_algorithm () {
    ./scripts/replace_default_feature.sh "${SOURCE_DIR}" "${DEFAULT_HASH}" "${SELECT_HASH}"
    ./scripts/replace_default_feature.sh "${SOURCE_DIR}" "${DEFAULT_CRYPT}" "${SELECT_CRYPT}"
}

function restore_algorithm () {
    ./scripts/replace_default_feature.sh "${SOURCE_DIR}" "${SELECT_HASH}" "${DEFAULT_HASH}"
    ./scripts/replace_default_feature.sh "${SOURCE_DIR}" "${SELECT_CRYPT}" "${DEFAULT_CRYPT}"
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
