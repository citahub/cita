#!/bin/bash
set -e

cd  ${WORKSPACE}
source ~/.cargo/env

echo "################################################################################"
# echo "0) test sm2/sm3"
# sed -i 's/\["secp256k1"\]/\["sm2"\]/g' share_libs/crypto/Cargo.toml
# sed -i 's/\["sha3hash"\]/\["sm3hash"\]/g' share_libs/util/Cargo.toml
make clean
make debug
make test
make bench

sed -i 's/\["sm2"\]/\["secp256k1"\]/g' share_libs/crypto/Cargo.toml
sed -i 's/\["sm3hash"\]/\["sha3hash"\]/g' share_libs/util/Cargo.toml

./tests/integrate_test/cita_basic.sh
./tests/integrate_test/cita_byzantinetest.sh


echo "################################################################################"
# echo "0) test ed25519/blake2b"
# sed -i 's/\["secp256k1"\]/\["ed25519"\]/g' share_libs/crypto/Cargo.toml
# sed -i 's/\["sha3hash"\]/\["blake2bhash"\]/g' share_libs/util/Cargo.toml
make clean
make debug
make test
make bench

sed -i 's/\["ed25519"\]/\["secp256k1"\]/g' share_libs/crypto/Cargo.toml
sed -i 's/\["blake2bhash"\]/\["sha3hash"\]/g' share_libs/util/Cargo.toml

./tests/integrate_test/cita_basic.sh
./tests/integrate_test/cita_byzantinetest.sh

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
