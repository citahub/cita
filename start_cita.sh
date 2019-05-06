#!/bin/bash

echo "Launch CITA"

rm -rf  target/install/test-chain
# compile
./env.sh make debug
#docker rm -f $(docker ps -a -q -f name=run)
cd target/install
echo "Compile finish"

# start CITA
bin/cita create --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"

bin/cita setup test-chain/0
bin/cita setup test-chain/1
bin/cita setup test-chain/2
bin/cita setup test-chain/3

echo "Boot CITA"
bin/cita start test-chain/0
bin/cita start test-chain/1
bin/cita start test-chain/2
bin/cita start test-chain/3

echo "Success"

