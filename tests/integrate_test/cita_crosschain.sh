#!/usr/bin/env bash

set -e -o pipefail

# Test private key & address
PKEY="1234567890123456789012345678901234567890123456789012345678901234"
PADDR="2e988a386a799f506693793c6a5af6b54dfaabfb"

# Chain Manager Contract
CMC_ADDR="ffffffffffffffffffffffffffffffffff020002"
CMC="scripts/contracts/src/system/chain_manager.sol"
CMC_ABI=

# Base dir for import contract files
CONTRACT_LIBS_DIR="scripts/contracts"

# Templates for some shell commands
JSONRPC_CALL='{"jsonrpc":"2.0","method":"call", "params":[{"to":"%s", "data":"%s"}, "pending"],"id":2}'
JSONRPC_BLOCKHEADER='{"jsonrpc":"2.0","method":"getBlockHeader","params":["0x%x"],"id":1}'
JSONRPC_STATEPROOF='{"jsonrpc":"2.0","method":"getStateProof","params":["0x%s","0x%s","0x%x"],"id":1}'

# Test contract file
CONTRACT_DEMO="scripts/contracts/tests/contracts/cross_chain_token.sol"
DEMO_ABI=

# Global variables which are set in functions
MAIN_CONTRACT_ADDR=
SIDE_CONTRACT_ADDR=

function title () {
    echo
    echo
    echo "################################################################################"
    echo "################################################################################"
    echo "################################################################################"
    echo
    echo "[$(date --iso-8601="seconds")] $@"
    echo
    echo "################################################################################"
    echo "################################################################################"
    echo "################################################################################"
    echo
    echo
}

function python_run () {
    local pycmd="$1;"
    shift 1
    while [ -n "$1" ]; do
        pycmd="${pycmd} $1;"
        shift 1
    done
    python3 -c "${pycmd}"
}

function func_encode () {
    local func="$1"
    python_run \
        "import sha3" \
        "keccak = sha3.keccak_256()" \
        "keccak.update('${func}'.encode('utf-8'))" \
        "print(keccak.hexdigest()[0:8])"
}

function map_key_encode () {
    local key="$1"
    local position="$2"
    python_run \
        "import sha3" \
        "import binascii" \
        "keccak = sha3.keccak_256()" \
        "keccak.update(binascii.unhexlify('${key}'))" \
        "keccak.update(binascii.unhexlify('${position}'))" \
        "print(keccak.hexdigest()[0:64])"
}

function abi_encode () {
    local abi="$1"
    local func="$2"
    local data="$3"
    python_run \
        "from ethereum.abi import ContractTranslator" \
        "import binascii" \
        "ct = ContractTranslator(b'''${abi}''')" \
        "tx = ct.encode('${func}', [${data}])" \
        "print(binascii.hexlify(tx).decode('utf-8'))"
}

function json_get () {
    #"outfmt = sys.argv[1].strip().split('.')[1:]" \
    local outfmt="$1"
    python_run \
        "import json" \
        "import sys" \
        "from functools import reduce" \
        "instr = sys.stdin.read().strip()" \
        "injson = json.loads(instr)" \
        "outfmt = \"${outfmt}\".strip().split('.')[1:]" \
        "print(reduce(lambda x, y: x[y], outfmt, injson))"
}

function txtool_run () {
    local chain=$1
    shift 1
    cd "${chain}tool/txtool"
    python3 "$@" 2>/dev/null
    cd ../..
}

function start_chain () {
    local chain=$1
    local size=$2
    title "Start chain [${chain}] ..."
    for ((id=0;id<${size};id++)); do
        bin/cita setup ${chain}chain/${id} && true
    done
    for ((id=0;id<${size};id++)); do
        bin/cita stop  ${chain}chain/${id}>/dev/null 2>&1 || true
        bin/cita start ${chain}chain/${id} trace>/dev/null 2>&1
    done
}

function stop_chain () {
    local chain=$1
    local size=$2
    title "Stop chain [${chain}] ..."
    for ((id=0;id<${size};id++)); do
        bin/cita stop ${chain}chain/${id}
    done
}

function wait_chain_for_height () {
    local chain=$1
    local height=$2
    title "Waiting for chain [${chain}] ..."
    while true; do
        local height_now=$(txtool_run ${chain} block_number.py | tail -1)
        if [ "${height}" != "None" ] \
                && [ "${height_now}" != "None" ] \
                && [ "${height_now}" -gt "${height}" ]; then
            break
        fi
    done
}

function deploy_contract () {
    local chain="$1"
    local solfile="$2"
    local extra="$3"
    local code="$(solc --allow-paths "$(pwd)/${CONTRACT_LIBS_DIR}" \
        --bin "${solfile}" 2>/dev/null | tail -1)${extra}"
    txtool_run ${chain} make_tx.py --privkey "${PKEY}" --code "${code}"
    txtool_run ${chain} send_tx.py
    txtool_run ${chain} get_receipt.py --forever true
}

function send_contract () {
    local chain="$1"
    local addr="$2"
    local abi="$3"
    local func="$4"
    local input="$5"
    local code="$(abi_encode "${abi}" "${func}" "${input}")"
    txtool_run ${chain} make_tx.py --privkey "${PKEY}" \
        --to "0x${addr}" --code "0x${code}"
    txtool_run ${chain} send_tx.py
    txtool_run ${chain} get_receipt.py --forever true
}

function call_contract () {
    local chain="$1"
    local addr="$2"
    local code="$3"
    case ${chain} in
        main)
            port=11337;;
        side)
            port=21337;;
        ?)
            exit 1
            ;;
    esac
    curl -s -X POST -d "$(printf "${JSONRPC_CALL}" "0x${addr}" "0x${code}")" \
        127.0.0.1:${port} \
        | json_get .result | xargs -I {} echo {}
}

function get_block_header () {
    local chain="$1"
    local height="$2"
    case ${chain} in
        main)
            port=11337;;
        side)
            port=21337;;
        ?)
            exit 1
            ;;
    esac
    curl -s -X POST -d "$(printf "${JSONRPC_BLOCKHEADER}" "${height}")" \
        127.0.0.1:${port} \
        | json_get .result | xargs -I {} echo {}
}

function get_state_proof () {
    local chain="$1"
    local address="$2"
    local key="$3"
    local height="$4"
    case ${chain} in
        main)
            port=11337;;
        side)
            port=21337;;
        ?)
            exit 1
            ;;
    esac
    curl -s -X POST -d "$(printf "${JSONRPC_STATEPROOF}" "${address}" "${key}" "${height}")" \
        127.0.0.1:${port} \
        | json_get .result | cut -c 3-
}

function get_addr () {
    local chain="$1"
    txtool_run ${chain} get_receipt.py --forever true \
        | json_get .contractAddress | cut -c 3-
}

function get_tx () {
    local chain="$1"
    txtool_run ${chain} get_receipt.py --forever true \
        | json_get .transactionHash | cut -c 3-
}

function get_tx_block_number () {
    local chain="$1"
    local txhash="$2"
    txtool_run ${chain} get_receipt.py --tx=${txhash} --forever true \
        | json_get .blockNumber
}

function hex2dec () {
    local hex="$1"
    if [ "$(echo ${hex} | cut -c 1-2)" != "0x" ] || [ "${hex}" = "0x" ]; then
        echo "none"
    else
        python_run "print(int('${hex}', 16))"
    fi
}

function parse_addresses () {
    local addrs="$(echo $1 | cut -c 131-)"
    local start=1
    local stop=64
    while [ -n "${addrs}" ]; do
        echo ${addrs} | cut -c 25-64 | xargs -I {} echo "0x{}"
        addrs="$(echo ${addrs} | cut -c 65-)"
    done | sort
}

function call_demo_for_main () {
    local code="$1"
    call_contract main "${MAIN_CONTRACT_ADDR}" "${code}"
}

function call_demo_for_side () {
    local code="$1"
    call_contract side "${SIDE_CONTRACT_ADDR}" "${code}"
}

function assert_equal () {
    local left="$1"
    local right="$2"
    local errmsg="$3"
    if [ "${left}" = "${right}" ]; then
        : # Test is passed!
    else
        echo "[ERROR] <${left}> != <${right}>: ${errmsg}."
        exit 1
    fi
}

function test_demo_contract () {
    local data=
    local code="$(func_encode 'getChainId()')"
    local main_chain_id_hex=$(call_contract main "${CMC_ADDR}" "${code}")
    local main_chain_id=$(hex2dec ${main_chain_id_hex})
    local side_chain_id_hex=$(call_contract side "${CMC_ADDR}" "${code}")
    local side_chain_id=$(hex2dec ${side_chain_id_hex})
    local main_tokens=50000
    local side_tokens=30000
    local crosschain_tokens=1234
    local crosschain_tokens_bytes=$( \
        printf "binascii.unhexlify('%064x')" ${crosschain_tokens})

    echo "main_chain_id=[${main_chain_id}]"
    echo "side_chain_id=[${side_chain_id}]"

    title "Check all authorities."
    local data=$(printf "%064x" "${side_chain_id}")
    local code="$(func_encode 'getAuthorities(uint256)')${data}"
    assert_equal \
        "$(parse_addresses \
            $(call_contract main "${CMC_ADDR}" "${code}") \
                | xargs -I {} printf {})" \
        "$(cat sidechain/template/authorities.list \
            | sort | xargs -I {} printf {})" \
        "The authorities is not right for side chain."
    local data=$(printf "%064x" "${main_chain_id}")
    local code="$(func_encode 'getAuthorities(uint256)')${data}"
    assert_equal \
        "$(parse_addresses \
            $(call_contract side "${CMC_ADDR}" "${code}") \
                | xargs -I {} printf {})" \
        "$(cat mainchain/template/authorities.list \
            | sort | xargs -I {} printf {})" \
        "The authorities is not right for main chain."

    title "Deploy contract for both main chain and side chain."
    deploy_contract main "${CONTRACT_DEMO}" \
        "$(printf "%064x" ${main_tokens})"
    deploy_contract side "${CONTRACT_DEMO}" \
        "$(printf "%064x" ${side_tokens})"
    MAIN_CONTRACT_ADDR=$(get_addr main)
    SIDE_CONTRACT_ADDR=$(get_addr side)
    echo "Demo contract for main at [${MAIN_CONTRACT_ADDR}]."
    echo "Demo contract for side at [${SIDE_CONTRACT_ADDR}]."

    title "Check from_chain_id for both chains."
    code="$(func_encode "getFromChainId()")"
    assert_equal "${main_chain_id}" \
        "$(hex2dec $(call_demo_for_main "${code}"))" \
        "The from_chain_id is not right for main chain."
    assert_equal "${side_chain_id}" \
        "$(hex2dec $(call_demo_for_side "${code}"))" \
        "The from_chain_id is not right for side chain."

    title "Check tokens for both chains."
    data=$(printf "%64s" "${PADDR}" | tr ' ' '0')
    code="$(func_encode 'getBalance(address)')${data}"
    assert_equal ${main_tokens} \
        "$(hex2dec $(call_demo_for_main "${code}"))" \
        "The tokens is not right for main chain."
    assert_equal ${side_tokens} \
        "$(hex2dec $(call_demo_for_side "${code}"))" \
        "The tokens is not right for side chain."

    title "Send tokens from main chain."
    DEMO_ABI=$(solc --allow-paths "$(pwd)/${CONTRACT_LIBS_DIR}" \
            --combined-json abi ${CONTRACT_DEMO} \
        | sed "s@${CONTRACT_DEMO}:@@g" \
        | json_get '.contracts.MyToken.abi')
    send_contract main "${MAIN_CONTRACT_ADDR}" "${DEMO_ABI}" \
        "sendToSideChain" \
        "${side_chain_id}, '${SIDE_CONTRACT_ADDR}', ${crosschain_tokens_bytes}"
    local maintx=$(get_tx main)

    title "Waiting for proof."
    local height_now=$(txtool_run main block_number.py | tail -1)
    wait_chain_for_height main $((height_now+3))

    title "Send tokens to side chain."
    cat > relayer-parser.json <<EOF
{
    "private_key": "0x1111111111111111111111111111111111111111111111111111111111111111",
    "chains": [
        {
            "id": "${main_chain_id_hex}",
            "servers": [
                { "url": "http://127.0.0.1:11337", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11338", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11339", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11340", "timeout": { "secs": 30, "nanos": 0 } }
            ]
        },
        {
            "id": "${side_chain_id_hex}",
            "servers": [
                { "url": "http://127.0.0.1:21337", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21338", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21339", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21340", "timeout": { "secs": 30, "nanos": 0 } }
            ]
        }
    ]
}
EOF
    local sidetx=$(./bin/cita-relayer-parser \
        -c ${main_chain_id_hex} -t ${maintx} \
        -f relayer-parser.json)
    rm relayer-parser.json

    title "Waiting for receipt for ${sidetx}."
    txtool_run side get_receipt.py --tx=${sidetx} --forever true

    local tx_block_number=$(hex2dec $(get_tx_block_number side ${sidetx}))
    title "Got tx_block_number ${tx_block_number}"
    # 3 is position of balanceOf in contract scripts/contracts/tests/contracts/cross_chain_token.sol
    local state_proof=$(get_state_proof side ${SIDE_CONTRACT_ADDR} $(map_key_encode "000000000000000000000000${PADDR}" "0000000000000000000000000000000000000000000000000000000000000003") ${tx_block_number})
    title "Got state_proof ${state_proof}"

    title "Check balance for both chains after crosschain transaction."
    data=$(printf "%64s" "${PADDR}" | tr ' ' '0')
    code="$(func_encode 'getBalance(address)')${data}"
    assert_equal $((main_tokens-crosschain_tokens)) \
        "$(hex2dec $(call_demo_for_main "${code}"))" \
        "The balance is not right for main chain."
    assert_equal $((side_tokens+crosschain_tokens)) \
        "$(hex2dec $(call_demo_for_side "${code}"))" \
        "The balance is not right for side chain."

    title "Check sync block header number."
    local data=$(printf "%064x" "${side_chain_id}")
    code="$(func_encode 'getExpectedBlockNumber(uint256)')${data}"
    assert_equal 0 \
        "$(hex2dec $(call_contract main "${CMC_ADDR}" "${code}"))" \
        "The block number of side chain in main chain is wrong."

    title "Get side chain block header bytes."
    local side_header_0=$(get_block_header side 0 | cut -c 3-)
    local side_header_1=$(get_block_header side 1 | cut -c 3-)
    local side_header_2=$(get_block_header side 2 | cut -c 3-)
    local side_header_3=$(get_block_header side 3 | cut -c 3-)
    local main_header_3=$(get_block_header main 3 | cut -c 3-)

    title "Sync side chain block header bytes to main chain."
    send_contract main "${CMC_ADDR}" "${CMC_ABI}" "verifyBlockHeader" \
        "${side_chain_id}, binascii.unhexlify('${side_header_0}')"
    send_contract main "${CMC_ADDR}" "${CMC_ABI}" "verifyBlockHeader" \
        "${side_chain_id}, binascii.unhexlify('${side_header_1}')"
    send_contract main "${CMC_ADDR}" "${CMC_ABI}" "verifyBlockHeader" \
        "${side_chain_id}, binascii.unhexlify('${side_header_3}')"
    send_contract main "${CMC_ADDR}" "${CMC_ABI}" "verifyBlockHeader" \
        "${side_chain_id}, binascii.unhexlify('${side_header_2}')"
    send_contract main "${CMC_ADDR}" "${CMC_ABI}" "verifyBlockHeader" \
        "${side_chain_id}, binascii.unhexlify('${main_header_3}')"

    title "Check sync block header number after sync."
    local data=$(printf "%064x" "${side_chain_id}")
    code="$(func_encode 'getExpectedBlockNumber(uint256)')${data}"
    assert_equal 3 \
        "$(hex2dec $(call_contract main "${CMC_ADDR}" "${code}"))" \
        "The block number of side chain in main chain is wrong."

    local max_block_number=$[tx_block_number+1]
    title "Relay block header until ${max_block_number}."
    for ((i=3;i<=${max_block_number};i++))
    do
        local side_header=$(get_block_header side ${i} | cut -c 3-)
        send_contract main "${CMC_ADDR}" "${CMC_ABI}" "verifyBlockHeader" \
            "${side_chain_id}, binascii.unhexlify('${side_header}')"
    done

    title "verify state proof"
    # 96 is offset of state proof in call args
    data=$(printf "%064x%064x%064x%064x" "${side_chain_id}" "${tx_block_number}" "96" "$[${#state_proof}/2]")
    code="$(func_encode 'verifyState(uint256,uint64,bytes)')${data}${state_proof}"
    local result=$(call_contract main "${CMC_ADDR}" "${code}")
    title "verify result ${result}"
    # result has 0x prefix
    assert_equal $((side_tokens+crosschain_tokens)) \
        "$(hex2dec "0x${result:130:194}")" \
        "The balance is not right for state proof."
}

function main () {

    title "Test is starting ..."

    local code=
    local main_chain_id=3
    local side_chain_id=4

    cd target/install

    title "Clean data ..."
    rm -rf mainchain sidechain maintool sidetool

    title "Create tools ..."
    cp -r scripts/txtool maintool
    cp -r scripts/txtool sidetool
    sed -i 's/port=1337/port=11337/g' maintool/txtool/config/setting.cfg
    sed -i 's/port=1337/port=21337/g' sidetool/txtool/config/setting.cfg

    title "Create main chain configs ..."
    ./scripts/create_cita_config.py create --chain_name mainchain \
        --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
        --nodes "127.0.0.1:14000,127.0.0.1:14001,127.0.0.1:14002,127.0.0.1:14003" \
        --jsonrpc_port 11337 --ws_port 14337 --grpc_port 15000 \
        --contract_arguments "SysConfig.chainId=${main_chain_id}"

    start_chain main 4

    title "Create side chain keys ..."
    for ((id=0;id<4;id++)); do
        bin/create_key_addr secret${id} address${id}
    done
    local side_auths=$(ls address[0-4] | sort | xargs -I {} cat {} \
        | tr '\n' ',' | rev | cut -c 2- | rev)
    rm address[0-4]
    local main_auths=$(cat mainchain/template/authorities.list \
        | xargs -I {} printf "%s," "{}" | rev | cut -c 2- | rev)
    ./scripts/create_cita_config.py create --chain_name sidechain \
        --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
        --authorities "${side_auths}" \
        --jsonrpc_port 21337 --ws_port 24337 --grpc_port 25000 \
        --contract_arguments "SysConfig.chainId=${side_chain_id}" \
            "ChainManager.parentChainId=${main_chain_id}" \
            "ChainManager.parentChainAuthorities=${main_auths}"

    wait_chain_for_height main 3

    title "Register side chain ..."
    CMC_ABI=$(solc --allow-paths "$(pwd)/${CONTRACT_LIBS_DIR}" \
        --combined-json abi ${CMC} 2>/dev/null \
        | sed "s@${CMC}:@@g" \
        | json_get '.contracts.ChainManager.abi')

    send_contract main "${CMC_ADDR}" "${CMC_ABI}" \
        "newSideChain" "${side_chain_id}, [${side_auths}]"

    title "Create side chain configs ..."
    for ((id=0;id<4;id++)); do
        ./scripts/create_cita_config.py append \
            --chain_name sidechain \
            --node "127.0.0.1:$((24000+${id}))" \
            --signer "$(cat secret${id})"
        rm -f secret${id}
    done

    start_chain side 4

    title "Enable side chain ..."
    send_contract main "${CMC_ADDR}" "${CMC_ABI}" \
        "enableSideChain" "${side_chain_id}"

    title "Test the demo contract ..."
    test_demo_contract

    title "Clean test data."
    stop_chain side 4
    stop_chain main 4
    rm -rf mainchain sidechain maintool sidetool

    cd ../..

    title "Test for crosschain is DONE."
}

main "$@"
