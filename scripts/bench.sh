#!/bin/bash

sudo() {
 set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo $*
    fi
    set +o noglob
}

if [[ `uname` == 'Darwin' ]]
then
    SCRIPT_PATH=`realpath $0`
    SOURCE_DIR=$(realpath $(dirname ${SCRIPT_PATH})/..)
else
    SCRIPT_PATH=`readlink -f $0`
    SOURCE_DIR=$(readlink -f $(dirname ${SCRIPT_PATH})/..)
fi

cd ${SOURCE_DIR}

CARGO_HOME=/opt/.cargo

echo "################################################################################"
echo "1) check whether docker support"
IMAGE="cryptape/cita-build"
sudo docker images 2>&1 | grep ${IMAGE} > /dev/null 2>&1
if [ $? == 0 ]; then
    echo "run in docker"
    # Only allocate tty if we detect one
    if [ -t 1 ]; then
        DOCKER_RUN_OPTIONS="-t"
    fi
    if [ -t 0 ]; then
        DOCKER_RUN_OPTIONS="${DOCKER_RUN_OPTIONS} -i"
    fi

    docker run --rm ${DOCKER_RUN_OPTIONS} \
           --volume ${SOURCE_DIR}:${SOURCE_DIR} \
           --volume ${HOME}/.cargo/registry:/root/.cargo/registry  \
           --volume ${HOME}/.cargo/git:/root/.cargo/git \
           --volume /etc/localtime:/etc/localtime \
           --workdir "$(pwd)" $IMAGE "${SCRIPT_PATH}"
    exit $?
else
    echo "run in native machine"
fi

set -e
echo "################################################################################"
echo "2) show git status and commit"
git status --short
git rev-parse HEAD

echo "################################################################################"
echo "3) setup & build & test"
echo "################################################################################"
echo "3.1) setup"
source ${CARGO_HOME}/env
scripts/config_rabbitmq.sh
# For native machine, skip this step.
# scripts/install_develop.sh

echo "################################################################################"
echo "3.2) build"
time make release

echo "################################################################################"
echo "3.3) benchmark in develop"
time make bench

echo "################################################################################"
echo "3.4) benchmark in deploy"
echo "################################################################################"
echo "3.4.1) chain_performance"
echo "################################################################################"
echo "performance test for create"
time bash tests/wrk_benchmark_test/chain_performance.sh 1 10000 0 0
echo "################################################################################"
echo "performance test for call"
time bash tests/wrk_benchmark_test/chain_performance.sh 2 10000 0 0
echo "################################################################################"
echo "performance test for store"
time bash tests/wrk_benchmark_test/chain_performance.sh 3 10000 0 0
