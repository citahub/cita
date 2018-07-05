#!/bin/bash
set -e

if [[ `uname` == 'Darwin' ]]
then
    SOURCE_DIR=$(realpath $(dirname $0)/../..)
else
    SOURCE_DIR=$(readlink -f $(dirname $0)/../..)
fi
BINARY_DIR=${SOURCE_DIR}/target/install

################################################################################
echo -n "0) prepare  ...  "
. ${SOURCE_DIR}/tests/integrate_test/util.sh
cd ${BINARY_DIR}
echo "DONE"

################################################################################
echo -n "1) cleanup   ...  "
cleanup
echo "DONE"

################################################################################
echo -n "2) generate config  ...  "
./scripts/create_cita_config.py \
    create \
    --chain_name "node" \
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
    --contract_arguments "SysConfig.economicalModel=1" > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) start nodes  ...  "
for i in {0..3} ; do
    ./bin/cita setup node/$i  > /dev/null
done
for i in {0..3} ; do
    ./bin/cita start node/$i trace > /dev/null &
done
echo "DONE"

################################################################################
echo -n "4) Run charge mode tests ...  "
echo ""
echo "    Install python packages for tools ..."
sudo pip3 install -r ${SOURCE_DIR}/scripts/txtool/requirements.txt

NODE_0_PRIVKEY=`cat ./node/0/privkey`
NODE_1_PRIVKEY=`cat ./node/1/privkey`
NODE_2_PRIVKEY=`cat ./node/2/privkey`
NODE_3_PRIVKEY=`cat ./node/3/privkey`
cd ./scripts/txtool/txtool
python3 ${SOURCE_DIR}/tests/integrate_test/test_charge_mode.py \
        --miner-privkeys \
        ${NODE_0_PRIVKEY} \
        ${NODE_0_PRIVKEY} \
        ${NODE_0_PRIVKEY} \
        ${NODE_0_PRIVKEY}
cd ../../..
echo "DONE"

################################################################################
echo -n "5) stop nodes  ...  "
for i in {0..3} ; do
    ./bin/cita stop node/$i  > /dev/null
done
echo "DONE"

################################################################################
echo -n "6) cleanup ... "
cleanup
echo "DONE"
