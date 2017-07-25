#!/bin/bash

cd  ${WORKSPACE}
source ~/.cargo/env

echo "1) clean"
make clean

echo "2) setup"
make setup

echo "3) build"
make debug

echo "4) unit test"
make test

echo "5) bench"
make bench

echo "6) integrate test"
./tests/integrate_test/cita_basic.sh

echo "7) byzantine test"
./tests/integrate_test/cita_byzantinetest.sh

echo "8) archive result"
mkdir -p ${WORKSPACE}/../archive/${BUILD_ID}
mv ${WORKSPACE}/target  ${WORKSPACE}/../archive/${BUILD_ID}
