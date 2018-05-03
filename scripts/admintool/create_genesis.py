#!/usr/bin/env python
# coding=utf-8

import argparse
import json
import os
import time
from os import path
import binascii
import hashlib
import sys
import random

from ethereum.tools.tester import (Chain, get_env)
from ethereum.tools._solidity import (
    get_solidity,
    compile_file,
    solidity_get_contract_data,
)
from ethereum.abi import ContractTranslator

assert get_solidity() is not None, 'Solidity not found!'

CONTRACTS_DIR = path.join(path.dirname(__file__), os.pardir, 'contracts')
# Remove 'admintool'
COMMON_DIR = path.join(path.dirname(__file__)[:-10], 'contracts/common')

CONTRACTS = {
    '0x00000000000000000000000000000000013241a2': {
        'file': 'system/node_manager.sol',
        'name': 'NodeManager'
    },
    '0x00000000000000000000000000000000013241a3': {
        'file': 'system/quota_manager.sol',
        'name': 'QuotaManager'
    },
    '0x0000000000000000000000000000000031415926': {
        'file': 'system/sys_config.sol',
        'name': 'SysConfig'
    },
    '0x00000000000000000000000000000000000000ce': {
        'file': 'system/chain_manager.sol',
        'name': 'ChainManager'
    },
    '0x00000000000000000000000000000000013241b2': {
        'file': 'permission_management/permission_management.sol',
        'name': 'PermissionManagement'
    },
    '0x00000000000000000000000000000000013241b3': {
        'file': 'permission_management/permission_creator.sol',
        'name': 'PermissionCreator'
    },
    '0x00000000000000000000000000000000013241b4': {
        'file': 'permission_management/authorization.sol',
        'name': 'Authorization'
    },
    '0x00000000000000000000000000000000013241b5': {
        'file': 'permission_management/permission.sol',
        'name': 'Permission'
    },
    '0xe9e2593c7d1db5ee843c143e9cb52b8d996b2380': {
        'file': 'permission_management/role_creator.sol',
        'name': 'RoleCreator'
    },
    '0xe3b5ddb80addb513b5c981e27bb030a86a8821ee': {
        'file': 'permission_management/role_management.sol',
        'name': 'RoleManagement'
    },
    '0x00000000000000000000000000000000013241b6': {
        'file': 'user_management/group.sol',
        'name': 'Group'
    },
    '0x00000000000000000000000000000000013241c2': {
        'file': 'user_management/group_management.sol',
        'name': 'GroupManagement'
    },
    '0x00000000000000000000000000000000013241c3': {
        'file': 'user_management/group_creator.sol',
        'name': 'GroupCreator'
    }
}


def init_contracts(nodes, args):
    result = dict()
    env = get_env(None)
    env.config['BLOCK_GAS_LIMIT'] = 471238800
    tester_state = Chain(env=env)

    for address, contract in CONTRACTS.iteritems():
        contract_path = path.join(CONTRACTS_DIR, contract['file'])
        simple_compiled = compile_file(
            contract_path,
            combined='bin,abi,userdoc,devdoc,hashes',
            extra_args='common=%s' % COMMON_DIR)
        simple_data = solidity_get_contract_data(
            simple_compiled,
            contract_path,
            contract['name'],
        )

        # Save the userdoc of contract
        userdoc_path = contract['name'] + "-userdoc.json"
        with open(userdoc_path, "w") as f:
            json.dump(simple_data['userdoc'], f, indent=4)

        # Save devdoc of contract
        devdoc_path = contract['name'] + "-devdoc.json"
        with open(devdoc_path, "w") as f:
            json.dump(simple_data['devdoc'], f, indent=4)

        # Save hashes of contract function
        hashes_path = contract['name'] + "-hashes.json"
        with open(hashes_path, "w") as f:
            json.dump(simple_data['hashes'], f, indent=4)

        if '' == simple_data['bin']:
            sys.exit()

        ct = ContractTranslator(simple_data['abi'])

        if address == '0x00000000000000000000000000000000013241a3' or address == '0x00000000000000000000000000000000013241b4':
            extra = (ct.encode_constructor_arguments([nodes[address]])
                     if nodes[address] else b'')
        elif address == '0x0000000000000000000000000000000031415926':
            if address in nodes:
                params = nodes[address]
                # Current chain id:
                #   - 3  bit prefix (0b000 means testnet)
                #   - 29 bit id is a random number in range [0, 2**29]
                params[4] = args.chain_id or random.randint(0, 2**(32 - 3))
                print '[chain-id]: {}'.format(params[4])
                print 'params: {}'.format(params)
                with open(args.chain_id_file, 'w') as f:
                    f.write('{}\n'.format(params[4]))
                extra = ct.encode_constructor_arguments(params)
            else:
                extra = b''
        elif address == '0x00000000000000000000000000000000013241b6':
            extra = (ct.encode_constructor_arguments(nodes[address][:3])
                     if nodes[address] else b'')
        elif address == '0x00000000000000000000000000000000013241a2':
            extra = (ct.encode_constructor_arguments(nodes[address][:2])
                     if nodes[address] else b'')
        elif address == '0x00000000000000000000000000000000013241b5':
            for addr, permission in nodes[address].iteritems():
                new_funcs = []
                for func in permission[2]:
                    new_func = ''
                    for i in range(0, len(func), 2):
                        new_func += chr((int(func[i:i + 2], 16)))
                    new_funcs.append(new_func)

                extra_common = (ct.encode_constructor_arguments(
                    [permission[0], permission[1], new_funcs]))

                if addr == '0x00000000000000000000000000000000013241b5':
                    extra = extra_common
                else:
                    abi_address = tester_state.contract(
                        simple_data['bin'] + extra_common,
                        language='evm',
                        startgas=30000000)
                    tester_state.mine()
                    account = tester_state.chain.state.account_to_dict(
                        abi_address)
                    result[addr] = {
                        'code': account['code'],
                        'storage': account['storage'],
                        'nonce': account['nonce']
                    }
        elif address == '0x00000000000000000000000000000000000000ce' and nodes[address]:
            current_chain_id = args.current_chain_id if args.current_chain_id else nodes[
                address][0]
            parent_chain_id = args.parent_chain_id \
                if args.parent_chain_id else nodes[address][1]
            parent_chain_nodes = args.parent_chain_nodes.split(',') \
                if args.parent_chain_nodes else nodes[address][2]
            extra = (ct.encode_constructor_arguments(
                [current_chain_id, parent_chain_id, parent_chain_nodes]))

        else:
            extra = ''

        print("contract:\n", contract['name'])
        print("extra:\n", binascii.hexlify(extra))
        abi_address = tester_state.contract(
            simple_data['bin'] + extra, language='evm', startgas=30000000)
        tester_state.mine()
        account = tester_state.chain.state.account_to_dict(abi_address)
        result[address] = {
            'code': account['code'],
            'storage': account['storage'],
            'nonce': account['nonce']
        }

    return result


def main():
    parser = argparse.ArgumentParser()

    parser.add_argument("--genesis_file", help="The genesis.json file.")
    parser.add_argument("--chain_id_file", help="The chain_id file.")
    parser.add_argument(
        "--chain_id", type=int, default=0, help="Specify the chain_id to use.")
    parser.add_argument(
        "--timestamp",
        type=int,
        default=0,
        help="Specify the timestamp to use.")
    parser.add_argument("--authorities", help="Authorities nodes list file.")
    parser.add_argument("--init_data", help="init with constructor_arguments.")
    parser.add_argument("--resource", help="chain resource folder.")
    parser.add_argument("--permission", help="init the permission.")
    parser.add_argument(
        "--current_chain_id",
        type=int,
        default=0,
        help="current chain id for chain management, a unique id")
    parser.add_argument(
        "--parent_chain_id",
        type=int,
        default=0,
        help="the unique id of the parent chain")
    parser.add_argument(
        "--parent_chain_nodes", help="the parent chain's consensus nodes")

    args = parser.parse_args()
    init_path = os.path.join(args.init_data)
    auth_path = os.path.join(args.authorities)
    res_path = os.path.join(args.resource)
    per_path = os.path.join(args.permission)

    authorities = []
    with open(auth_path, "r") as f:
        for line in f:
            authorities.append(line.strip('\n'))

    init_data = dict()
    permission_data = dict()

    with open(init_path, "r") as f:
        init_data = json.load(f)

    with open(per_path, "r") as f:
        permission_data = json.load(f)

    init_data.update(permission_data)

    for auth in authorities:
        init_data["0x00000000000000000000000000000000013241a2"][0].append(auth)

    data = dict()
    timestamp = int(time.time() if not args.timestamp else args.timestamp)
    if os.path.exists(res_path) and os.path.isdir(res_path):
        #file list make sure same order when calc hash
        file_list = ""
        res_path_len = len(res_path)
        md5obj = hashlib.md5()
        for root, dirs, files in os.walk(res_path, True):
            for name in files:
                filepath = os.path.join(root, name)
                with open(filepath, 'rb') as f:
                    md5obj.update(f.read())
                    file_list += filepath[res_path_len:] + "\n"
        res_hash = md5obj.hexdigest()

        file_list_path = os.path.join(res_path, "file_list")
        with open(file_list_path, 'w') as f:
            f.write(file_list)
        data["prevhash"] = "0x00000000000000000000000000000000" + res_hash
    else:
        data[
            "prevhash"] = "0x0000000000000000000000000000000000000000000000000000000000000000"
    data["timestamp"] = timestamp

    print("init data\n", json.dumps(init_data, indent=4))
    alloc = init_contracts(init_data, args)
    data['alloc'] = alloc
    with open(args.genesis_file, "w") as fil:
        json.dump(data, fil, indent=4)


if __name__ == '__main__':
    main()
