#!/usr/bin/env python  
# coding=utf-8  

import argparse
import json
import os
import time
from os import path

from ethereum.tools.tester import Chain
from ethereum.tools._solidity import (
   get_solidity,
   compile_file,
   solidity_get_contract_data,
)
from ethereum.abi import ContractTranslator

SOLIDITY_AVAILABLE = get_solidity() is not None
CONTRACTS_DIR = path.join(path.dirname(__file__), os.pardir, 'contracts')
CONTRACTS = {
   '0x00000000000000000000000000000000013241a2': {'file': 'node_manager.sol',
                                                  'name': 'NodeManager'}
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
        extra = (ct.encode_constructor_arguments([nodes]) if nodes else b'')
        abi_address = tester_state.contract(simple_data['bin'] + extra)
        tester_state.mine()
        account = tester_state.chain.state.account_to_dict(abi_address)
        result[address] = {'code': account['code'], 'storage': account['storage'], 'nonce': account['nonce']}
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--authorities", help="Authorities nodes list file.")

    args = parser.parse_args()
    auth_path = os.path.join(args.authorities)

    authorities = []
    with open(auth_path, "r") as authority_file:
        for line in authority_file:
            authorities.append(line.strip('\n'))
    data = dict()
    timestamp = int(time.time())
    data["prevhash"] = "0x0000000000000000000000000000000000000000000000000000000000000000"
    data["timestamp"] = timestamp

    print "authorities", authorities
    alloc = init_contracts(authorities)
    data['alloc'] = alloc
    dump_path = os.path.join(path.dirname(__file__), "genesis.json")
    with open(dump_path, "w") as f:
        json.dump(data, f, indent=4)


if __name__ == '__main__':
    main()
