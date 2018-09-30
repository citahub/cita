#!/usr/bin/env python3
# -*- coding:utf-8 -*-
# pylint: disable=missing-docstring

import argparse
import collections
import yaml

DEFAULT_CONFIG = '''
Contracts:
- SysConfig:
  - delayBlockNumber: 1
  - checkPermission: false
  - checkSendTxPermission: false
  - checkCreateContractPermission: false
  - checkQuota: false
  - checkFeeBackPlatform: false
  - chainOwner: '0x0000000000000000000000000000000000000000'
  - chainName: test-chain
  - chainId: 1
  - operator: test-operator
  - website: https://www.example.com
  - blockInterval: 3000
  - economicalModel: 0
  - name: Nervos AppChain Test Token
  - symbol: NATT
  - avatar: https://avatars1.githubusercontent.com/u/35361817
- QuotaManager:
  - admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
- NodeManager:
  - nodes:
    - '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
  - stakes:
    - 0
- ChainManager:
  - parentChainId: 0
  - parentChainAuthorities: []
- Authorization:
  - superAdmin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
- Group:
  - parent: '0x0000000000000000000000000000000000000000'
  - name: rootGroup
  - accounts:
    - '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
- Admin:
  - admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
- VersionManager:
  - version: 0
'''


def dictlist_to_ordereddict(dictlist):
    """Convert a list of dict to an ordered dict."""
    odd = collections.OrderedDict()
    for dic in dictlist:
        for key, val in dic.items():
            odd[key] = val
    return odd


def ordereddict_to_dictlist(ordereddict):
    """Convert an ordered dict to a list of dict."""
    return [{key: value} for key, value in ordereddict.items()]


def conv_type_as_old(new, old, key1, key2, val=None):
    if val is None:
        val = new
    if isinstance(old, list):
        old_val = old[0] if old else None
        return [
            conv_type_as_old(element, old_val, key1, key2, val)
            for element in new.split(',')
        ]
    elif isinstance(old, bool):
        if new.lower() in ('true', 'false'):
            return new.lower() == 'true'
    elif isinstance(old, int):
        return int(new)
    elif isinstance(old, str):
        return str(new)
    elif old is None:
        try:
            _ = int(new)
        except ValueError:
            return str(new)
        else:
            return int(new)
    raise Exception('Type for {}.{}={} is not right'.format(key1, key2, val))


class InitializationData(object):
    def __init__(self, contracts_cfgs):
        self.contracts_cfgs = contracts_cfgs

    @classmethod
    def load_from_string(cls, cfg):
        data = yaml.load(cfg)
        contracts_cfgs = dictlist_to_ordereddict(data['Contracts'])
        for name, arguments in contracts_cfgs.items():
            contracts_cfgs[name] = dictlist_to_ordereddict(arguments)
        return cls(contracts_cfgs=contracts_cfgs)

    def update_by_kkv_dict(self, kkv_dict):
        if not kkv_dict:
            return
        for key1, val1 in kkv_dict.items():
            configs = self.contracts_cfgs.get(key1)
            if configs is None:
                raise Exception('There is no contract named {}'.format(key1))
            else:
                for key2, val2 in val1.items():
                    val2_old = configs.get(key2)
                    if val2_old is None:
                        raise Exception(
                            'There is no argument named {} for {}'.format(
                                key2, key1))
                    else:
                        configs[key2] = conv_type_as_old(
                            val2, val2_old, key1, key2)
                self.contracts_cfgs[key1] = configs

    def set_super_admin(self, super_admin):
        if not super_admin:
            return
        self.contracts_cfgs['QuotaManager']['admin'] = super_admin
        self.contracts_cfgs['Authorization']['superAdmin'] = super_admin
        self.contracts_cfgs['Group']['accounts'] = [super_admin]
        self.contracts_cfgs['Admin']['admin'] = super_admin

    def save_to_file(self, filepath):
        data = dict()
        contracts_cfgs = self.contracts_cfgs
        for name, arguments in contracts_cfgs.items():
            contracts_cfgs[name] = ordereddict_to_dictlist(arguments)
        data['Contracts'] = ordereddict_to_dictlist(contracts_cfgs)
        with open(filepath, 'w') as stream:
            yaml.dump(data, stream, default_flow_style=False)


class KeyKeyValueDict(dict):
    @staticmethod
    def str2tuple(kkv):
        kk_v = kkv.split('=')
        if len(kk_v) == 2 and kk_v[1]:
            k_k = kk_v[0].split('.')
            if len(k_k) == 2 and k_k[0] and k_k[1]:
                return (k_k[0], k_k[1], kk_v[1])
        raise Exception('input {} is not like Key.Key=Value'.format(kkv))

    def kkv_get(self, key1, key2):
        key2vals = self.get(key1)
        if key2vals is None:
            return None
        return key2vals.get(key2)

    def kkv_set(self, key1, key2, val):
        val_old = self.kkv_get(key1, key2)
        if val_old is None:
            if val is not None:
                self.kkv_update(key1, key2, val)
        else:
            raise Exception('{}.{} has been set twice: {} and {}'.format(
                key1, key2, val, val_old))

    def kkv_update(self, key1, key2, val):
        key2vals = self.get(key1)
        if key2vals is None:
            self.update({key1: {key2: val}})
        else:
            key2vals.update({key2: val})
            self.update({key1: key2vals})


class KeyKeyValueAction(argparse.Action):
    # pylint: disable=too-few-public-methods
    def __init__(self, option_strings, dest, **kwargs):
        super(KeyKeyValueAction, self).__init__(option_strings, dest, **kwargs)

    def __call__(self, parser, namespace, values, option_string=None):
        kkv = getattr(namespace, self.dest)
        if kkv is None:
            kkv = KeyKeyValueDict()
        for (key1, key2, val) in values:
            kkv.kkv_set(key1, key2, val)
        setattr(namespace, self.dest, kkv)


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--output', required=True, help='Path of the output file.')
    parser.add_argument('--super_admin', help='Address of super admin.')
    parser.add_argument(
        '--contract_arguments',
        nargs='+',
        type=KeyKeyValueDict.str2tuple,
        action=KeyKeyValueAction,
        metavar='Contract.Argument=Value',
        help='Update constructor arguments for system contract.'
        ' Can be specify more than once.')
    args = parser.parse_args()
    return dict(
        contract_arguments=args.contract_arguments,
        super_admin=args.super_admin,
        output=args.output,
    )


def core(output, super_admin, contract_arguments):
    init_data = InitializationData.load_from_string(DEFAULT_CONFIG)
    init_data.update_by_kkv_dict(contract_arguments)
    init_data.set_super_admin(super_admin)
    init_data.save_to_file(output)


if __name__ == '__main__':
    core(**parse_arguments())
