#!/bin/bash
set -e


PROJECT_DIR=$(readlink -f $(dirname $(readlink -f $0))/..)
cd  ${PROJECT_DIR}
source ~/.cargo/env

# For native machine, skip this step.
# echo "################################################################################"
# echo "1) setup"
# scripts/install_develop.sh

echo "################################################################################"
echo "2) format"
time make fmt

echo "################################################################################"
echo "3) build"
time make debug

echo "################################################################################"
echo "4) unit test"
time make test

echo "################################################################################"
echo "5) integrate test"
echo "5.1) basic test(contract create/call, node start/stop)"
time ./tests/integrate_test/cita_basic.sh
echo "5.2) byzantine test"
time ./tests/integrate_test/cita_byzantinetest.sh

echo "################################################################################"
echo "6) archive result"
mkdir -p ${WORKSPACE}/../archive/${BUILD_ID}
mv ${WORKSPACE}/target  ${WORKSPACE}/../archive/${BUILD_ID}
