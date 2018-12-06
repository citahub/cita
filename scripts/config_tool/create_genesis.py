#!/usr/bin/env python3
# -*- coding:utf-8 -*-
# pylint: disable=missing-docstring

import argparse
import binascii
import json
import os
import sys
import time

from ethereum.abi import ContractTranslator
import sha3
import yaml

from create_init_data import dictlist_to_ordereddict


DEFAULT_PREVHASH = '0x{:064x}'.format(0)
BLOCK_GAS_LIMIT = 471238800


def disable_import_warning():
    """This is a temporary method.
    We do NOT need bitcoin. We want to decrease the size of docker.
    So, just filter out the unnecessary warning.
    """

    import builtins
    from types import ModuleType

    class DummyModule(ModuleType):
        def __getattr__(self, key):
            return None
        __all__ = []

    def filterimport(name, globals=None, locals=None, fromlist=(), level=0):
        if name == 'bitcoin':
            return DummyModule(name)
        return realimport(name, globals, locals, fromlist, level)

    realimport, builtins.__import__ = builtins.__import__, filterimport


disable_import_warning()
import ethereum.tools.tester as eth_tester
import ethereum.tools._solidity as solidity


def replaceLogRecord():
    """This is a temporary method.
    We will remove pyethereum in the near future.
    """

    import logging
    import re

    def makeRecord(self, name, level, fn, lno, msg, args, exc_info,
                   func=None, extra=None, sinfo=None):
        name = re.sub(r'(^|[^a-zA-Z])eth([^a-zA-Z]|$)', r'\1cita\2', name)
        rv = logging._logRecordFactory(
            name, level, fn, lno, msg, args, exc_info, func, sinfo)
        if extra is not None:
            for key in extra:
                if (key in ["message", "asctime"]) or (key in rv.__dict__):
                    raise KeyError("Attempt to overwrite %r in LogRecord" % key)
                rv.__dict__[key] = extra[key]
        return rv

    def getMessage(self):
        msg = str(self.msg)
        if self.args:
            msg = msg % self.args
        msg = re.sub(r'(^|[^a-zA-Z])eth([^a-zA-Z]|$)', r'\1cita\2', msg)
        msg = re.sub(r'(^|[^a-zA-Z])gas([^a-zA-Z]|$)', r'\1quota\2', msg)
        return msg

    logging.Logger.makeRecord = makeRecord
    logging.LogRecord.getMessage = getMessage


def function_encode(func_sign):
    keccak = sha3.keccak_256()
    keccak.update(func_sign.encode('utf-8'))
    return binascii.unhexlify(keccak.hexdigest()[0:8])


class GenesisData(object):
    # pylint: disable=too-many-instance-attributes,too-many-arguments
    def __init__(
            self, contracts_dir, contracts_docs_dir, init_data_file,
            timestamp, prevhash):
        self.timestamp = int(time.time() * 1000) if not timestamp else timestamp
        self.prevhash = DEFAULT_PREVHASH if not prevhash else prevhash

        self.contracts_dir = os.path.join(contracts_dir, 'src')
        self.contracts_docs_dir = contracts_docs_dir
        self.contracts_common_dir = os.path.join(self.contracts_dir, 'common')
        self.contracts_lib_dir = os.path.join(self.contracts_dir, 'lib')
        self.contracts_interfaces_dir = os.path.join(self.contracts_dir, 'interfaces')
        contracts_list_file = os.path.join(contracts_dir, 'contracts.yml')
        self.load_contracts_list(contracts_list_file)
        self.load_contracts_args(init_data_file)

        self.init_chain_tester()

        self.accounts = dict()

    def load_contracts_list(self, contracts_list_file):
        """From file to load the list of contracts."""
        with open(contracts_list_file, 'r') as stream:
            contracts_list = yaml.load(stream)
        contracts_list['NormalContracts'] = dictlist_to_ordereddict(
            contracts_list['NormalContracts'])
        contracts_list['PermissionContracts']['basic'] \
            = dictlist_to_ordereddict(
                contracts_list['PermissionContracts']['basic'])
        contracts_list['PermissionContracts']['contracts'] \
            = dictlist_to_ordereddict(
                contracts_list['PermissionContracts']['contracts'])
        self.contracts_list = contracts_list

    def load_contracts_args(self, init_data_file):
        """From file to load arguments for contracts."""
        with open(init_data_file, 'r') as stream:
            data = yaml.load(stream)
        contracts_args = dictlist_to_ordereddict(data['Contracts'])
        for name, arguments in contracts_args.items():
            contracts_args[name] = dictlist_to_ordereddict(arguments)
        self.contracts_args = contracts_args

    def init_chain_tester(self):
        """Init a chain tester."""
        chain_env = eth_tester.get_env(None)
        chain_env.config['BLOCK_GAS_LIMIT'] = BLOCK_GAS_LIMIT
        self.chain_tester = eth_tester.Chain(env=chain_env)

    def compile_to_data(self, name, path):
        """Compile a solidity file and return the result data."""

        import logging

        compiled = solidity.compile_file(
            path,
            combined='bin,abi,userdoc,devdoc,hashes',
            extra_args='common={} lib={} interfaces={}'.format(
                self.contracts_common_dir,
                self.contracts_lib_dir,
                self.contracts_interfaces_dir))
        data = solidity.solidity_get_contract_data(compiled, path, name)
        if not data['bin']:
            logging.critical('The bin of contract %r is empty. Please check it!', name)
            sys.exit(1)
        return data

    def write_docs(self, name, data):
        """Save userdoc, devdoc and hashes of contract function."""
        if self.contracts_docs_dir:
            for doc_type in ('userdoc', 'devdoc', 'hashes'):
                doc_file = os.path.join(self.contracts_docs_dir,
                                        '{}-{}.json'.format(name, doc_type))
                with open(doc_file, 'w') as stream:
                    json.dump(data[doc_type], stream, separators=(',', ': '), indent=4)

    def mine_contract_on_chain_tester(self, addr, code):
        """Mine in test chain to get data of a contract."""
        addr_in_tester = self.chain_tester.contract(
            code, language='evm', startgas=30000000)
        self.chain_tester.mine()
        account_in_tester = self.chain_tester \
              .chain.state.account_to_dict(addr_in_tester)
        self.accounts[addr] = {
            key: val
            for (key, val) in filter(
                lambda keyval: keyval[0] in ('code', 'storage', 'nonce'),
                account_in_tester.items(),
            )
        }

    def init_normal_contracts(self):
        """Compile normal contracts from files and construct by arguments.
        """
        flags = [
            'checkCallPermission',
            'checkSendTxPermission',
            'checkCreateContractPermission',
            'checkQuota',
            'checkFeeBackPlatform',
            'autoExec'
        ]
        ncinfo = self.contracts_list['NormalContracts']
        for name, info in ncinfo.items():
            addr = info['address']
            path = os.path.join(self.contracts_dir, info['file'])
            data = self.compile_to_data(name, path)
            self.write_docs(name, data)
            ctt = ContractTranslator(data['abi'])
            args = self.contracts_args.get(name)
            if name == 'SysConfig':
                args['flags'] = []
                for flag in flags:
                    args['flags'].append(args[flag])
                    args.pop(flag)
            extra = b'' if not args else ctt.encode_constructor_arguments(
                [arg for arg in args.values()])
            self.mine_contract_on_chain_tester(addr, data['bin'] + extra)

    def init_permission_contracts(self):
        ncinfo = self.contracts_list['NormalContracts']
        pcinfo = self.contracts_list['PermissionContracts']
        path = os.path.join(self.contracts_dir, pcinfo['file'])
        data = self.compile_to_data('Permission', path)
        self.write_docs('Permission', data)
        for name, info in pcinfo['basic'].items():
            addr = info['address']
            conts = [addr]
            funcs = [binascii.unhexlify('00000000')]
            ctt = ContractTranslator(data['abi'])
            extra = ctt.encode_constructor_arguments([name, conts, funcs])
            self.mine_contract_on_chain_tester(addr, data['bin'] + extra)
        for name, info in pcinfo['contracts'].items():
            addr = info['address']
            conts = [ncinfo[cont]['address'] for cont in info['contracts']]
            funcs = [function_encode(func) for func in info['functions']]
            ctt = ContractTranslator(data['abi'])
            extra = ctt.encode_constructor_arguments([name, conts, funcs])
            self.mine_contract_on_chain_tester(addr, data['bin'] + extra)

    def set_account_value(self, address, value):
        for addr in address:
            self.accounts[addr] = {
                'code': '',
                'storage': {},
                'nonce': '1',
                'value': value,
            }


    def save_to_file(self, filepath):
        with open(filepath, 'w') as stream:
            json.dump(
                dict(
                    timestamp=self.timestamp,
                    prevhash=self.prevhash,
                    alloc=self.accounts,
                ),
                stream,
                separators=(',', ': '),
                indent=4)


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--contracts_dir', required=True, help='The directory of contracts.')
    parser.add_argument(
        '--contracts_docs_dir',
        help='The directory of generated documents for contracts.'
        ' If did not be specified, no documents will be generated.')
    parser.add_argument(
        '--init_data_file',
        required=True,
        help='Path of the file for initialization data of contracts.')
    parser.add_argument(
        '--output', required=True, help='Path of the output file.')
    parser.add_argument(
        '--timestamp', type=int, help='Specify a timestamp to use.')
    parser.add_argument('--prevhash', help='Prevhash of genesis.')
    args = parser.parse_args()
    return dict(
        contracts_dir=args.contracts_dir,
        contracts_docs_dir=args.contracts_docs_dir,
        init_data_file=args.init_data_file,
        output=args.output,
        timestamp=args.timestamp,
        prevhash=args.prevhash,
    )


def core(contracts_dir, contracts_docs_dir, init_data_file, output, timestamp,
         prevhash):
    # pylint: disable=too-many-arguments
    replaceLogRecord()
    if solidity.get_solidity() is None:
        print('Solidity not found!')
        sys.exit(1)
    if contracts_docs_dir:
        contracts_docs_dir = os.path.abspath(contracts_docs_dir)
    genesis_data = GenesisData(
        os.path.abspath(contracts_dir),
        contracts_docs_dir,
        os.path.abspath(init_data_file),
        timestamp,
        prevhash,
    )
    with open(init_data_file, 'r') as stream:
        data = yaml.load(stream)
    address = data['Contracts'][2]['NodeManager'][0]['nodes']
    super_admin = data['Contracts'][6]['Admin'][0]['admin']
    address.append(super_admin)
    value = '0xffffffffffffffffffffffffff'
    genesis_data.init_normal_contracts()
    genesis_data.init_permission_contracts()
    genesis_data.set_account_value(address, value)
    genesis_data.save_to_file(output)


if __name__ == '__main__':
    core(**parse_arguments())
