#!/bin/bash
set -e

cd  ${WORKSPACE}
source ~/.cargo/env

echo "################################################################################"
echo "1) clean"
make clean

# For native machine, skip this step.
# echo "################################################################################"
# echo "2) setup"
# sudo make setup1
# make setup2

echo "################################################################################"
echo "3) format"
make fmt

echo "################################################################################"
echo "4) build"
make debug

echo "################################################################################"
echo "5) unit test"
make test

echo "################################################################################"
echo "6) bench"
make bench

echo "################################################################################"
echo "7) integrate test"
./tests/integrate_test/cita_basic.sh

echo "################################################################################"
echo "8) byzantine test"
./tests/integrate_test/cita_byzantinetest.sh

echo "################################################################################"
echo "9) archive result"
mkdir -p ${WORKSPACE}/../archive/${BUILD_ID}
mv ${WORKSPACE}/target  ${WORKSPACE}/../archive/${BUILD_ID}
