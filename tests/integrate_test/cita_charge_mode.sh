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
    --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
    --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
    --contract_arguments "SysConfig.checkFeeBackPlatform=true" \
    --contract_arguments "SysConfig.economicalModel=1" \
    --contract_arguments "VersionManager.version=0" \
    --contract_arguments "SysConfig.chainOwner=0x36a60d575b0dee0423abb6a57dbc6ca60bf47545" > /dev/null 2>&1
echo "DONE"

################################################################################
echo -n "3) start nodes  ...  "
for i in {0..3} ; do
    ./bin/cita setup node/$i > /dev/null
done
for i in {0..3} ; do
    ./bin/cita start node/$i trace > /dev/null &
done
echo "DONE"

################################################################################
echo -n "4) check alive  ...  "
timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                              echo "failed to check_height_growth 0: ${timeout}"
                                              exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "5) Run fee back tests ... "
cd ./scripts/txtool/txtool
python3 ${SOURCE_DIR}/tests/integrate_test/test_fee_back.py --version=0
cd ../../..
echo "DONE"

################################################################################
echo -n "6) Run charge mode tests ...  "
echo ""

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
        ${NODE_0_PRIVKEY} \
        --version=0
cd ../../..
echo "DONE"

################################################################################
echo -n "7) Update to chainIDV1 ... "
cd ./scripts/txtool/txtool
python3 ${SOURCE_DIR}/tests/integrate_test/update_version.py --version=0
cd ../../..
echo "DONE"

################################################################################
echo -n "8) check alive  ...  "
timeout=$(check_height_growth_normal 0 60) || (echo "FAILED"
                                              echo "failed to check_height_growth 0: ${timeout}"
                                              exit 1)
echo "${timeout}s DONE"

################################################################################
echo -n "9) Run fee back tests in v1 ... "
cd ./scripts/txtool/txtool
python3 ${SOURCE_DIR}/tests/integrate_test/test_fee_back.py --version=1
cd ../../..
echo "DONE"

################################################################################
echo -n "10) Run charge mode tests in v1 ...  "
echo ""

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
        ${NODE_0_PRIVKEY} \
        --version 1
cd ../../..
echo "DONE"

################################################################################
echo -n "11) stop nodes  ...  "
for i in {0..3} ; do
    ./bin/cita stop node/$i > /dev/null
done
echo "DONE"

################################################################################
echo -n "11) cleanup ... "
cleanup
echo "DONE"
