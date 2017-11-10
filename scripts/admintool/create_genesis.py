#!/usr/bin/env python  
# coding=utf-8  

import argparse
import json
import os
import time
from os import path
import binascii

from ethereum.tools.tester import Chain
from ethereum.tools._solidity import (
    get_solidity,
    compile_file,
    solidity_get_contract_data,
)
from ethereum.abi import ContractTranslator

SOLIDITY_AVAILABLE = get_solidity() is not None
CONTRACTS_DIR = path.join(path.dirname(__file__), os.pardir, 'contracts/system')
CONTRACTS = {
    '0x00000000000000000000000000000000013241a2': {'file': 'node_manager.sol',
                                                   'name': 'NodeManager'},
    '0x00000000000000000000000000000000013241a3': {'file': 'quota_manager.sol',
                                                   'name': 'QuotaManager'},
    '0x00000000000000000000000000000000013241a4': {'file': 'permission_manager.sol',
                                                   'name': 'PermissionManager'}
}

def init_contracts(nodes):
    result = dict()
    tester_state = Chain()
    for address, contract in CONTRACTS.iteritems():
        contract_path = path.join(CONTRACTS_DIR, contract['file'])
        simple_compiled = compile_file(contract_path)
        simple_data = solidity_get_contract_data(
            simple_compiled,
            contract_path,
            contract['name'],
        )

        ct = ContractTranslator(simple_data['abi'])
        if (address == '0x00000000000000000000000000000000013241a3'):
            extra = (ct.encode_constructor_arguments([nodes[address]]) if nodes[address] else b'')
        else:
            extra = (ct.encode_constructor_arguments([nodes[address][0], nodes[address][1]]) if nodes[address] else b'')
        print(binascii.hexlify(simple_data['bin'] + extra))
        abi_address = tester_state.contract(simple_data['bin'] + extra)
        tester_state.mine()
        account = tester_state.chain.state.account_to_dict(abi_address)
        result[address] = {'code': account['code'], 'storage': account['storage'], 'nonce': account['nonce']}
    return result


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--authorities", help="Authorities nodes list file.")
    parser.add_argument(
        "--init_data", help="init with constructor_arguments.")

    args = parser.parse_args()
    init_path = os.path.join(args.init_data)
    auth_path = os.path.join(args.authorities)

    authorities = []
    with open(auth_path, "r") as f:
        for line in f:
            authorities.append(line.strip('\n'))

    init_data = dict()

    with open(init_path, "r") as f:
        init_data = json.load(f)

    for auth in authorities:
        init_data["0x00000000000000000000000000000000013241a2"][0].append(auth)

    data = dict()
    timestamp = int(time.time())
    data["prevhash"] = "0x0000000000000000000000000000000000000000000000000000000000000000"
    data["timestamp"] = timestamp

    print "init data", init_data
    alloc = init_contracts(init_data)
    data['alloc'] = alloc
    dump_path =  "genesis.json"
    with open(dump_path, "w") as f:
        json.dump(data, f, indent=4)

if __name__ == '__main__':
    main()
