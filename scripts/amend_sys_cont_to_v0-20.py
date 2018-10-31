#!/usr/bin/env python3
# -*- coding:utf-8 -*-

"""
Amend the code and storage of system contract.
"""

import json
import functools
import subprocess
import logging
import time
import sys
import argparse
from jsonrpcclient.http_client import HTTPClient


LATEST_VERSION = 1
AMEND_ADDR = '0xffffffffffffffffffffffffffffffffff010002'
SYS_CONF = '0xffffffffffffffffffffffffffffffffff020000'
NEW = '0xffffffffffffffffffffffffffffffffff020012'
KEY = '0x30'
LOCAL = 'http://127.0.0.1:1337'


def send_tx(args):
    """
    Send a transfer transaction to a node

        python3 make_tx.py \
        --code "" \
        --to 0xffffffffffffffffffffffffffffffffff010002 \
        --no-newcrypto

        python3 send_tx.py

    """
    kwargs = {
        '--privkey': args.privkey,
        '--to': AMEND_ADDR,
        '--code': args.code,
        '--value': str(args.value),
        '--version': str(args.version),
        '--chain_id': str(args.chain_id),
    }
    args = functools.reduce(
        lambda lst, kv: lst + list(kv),
        kwargs.items(),
        [],
    )
    print(['python3', 'make_tx.py', *args, '--no-newcrypto'])
    subprocess.call(['python3', 'make_tx.py', *args, '--no-newcrypto'])
    subprocess.call(['python3', 'send_tx.py'])
    with open('../output/transaction/hash') as fobj:
        return fobj.read().strip()


def rpc_request(method, params, url=LOCAL):
    """ Send a jsonrpc request to default url. """
    client = HTTPClient(url)
    return client.request(method, params)


def get_receipt(tx_hash, url, retry=8):
    """ Get receipt of a transaction """
    while retry > 0:
        receipt = rpc_request('getTransactionReceipt', [tx_hash], url)
        if receipt is not None:
            return receipt
        time.sleep(4)
        retry -= 1


def amend_code(addr, code, args):
    """ Amend the code """
    if code:
        code = addr + code[2:]
        print('code:', code)
        args.code = code
        tx_hash = send_tx(args)
        receipt = get_receipt(tx_hash, args.url)
        if receipt['errorMessage']:
            logging.critical('receipt error')
            sys.exit(1)


def amend_storage(addr, key, val, args):
    """ Amend the storage: key and value """
    code = addr + key[2:].zfill(64) + val[2:].zfill(64)
    print('code:', code)
    args.code = code
    tx_hash = send_tx(args)
    receipt = get_receipt(tx_hash, args.url)
    if receipt['errorMessage']:
        logging.critical('receipt error')
        sys.exit(1)


def parse_arguments():
    """ parse the arguments: chain_id, version, privkey, url """
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--version",
        help="Tansaction version.",
        default=LATEST_VERSION,
        type=int)
    parser.add_argument(
        '--privkey',
        required=True,
        help='The admin private key.')
    parser.add_argument(
        '--chain_id',
        required=True,
        help='The id of the chain.')
    parser.add_argument(
        '--url',
        required=True,
        help='The url of the chain.')


    args = parser.parse_args()
    return args


def main():
    """ Load the genesis file and amend """
    args = parse_arguments()

    with open('../../genesis.json', 'r') as gene:
        genesis = json.load(gene)

    alloc = genesis['alloc']

    for addr in alloc:
        # amend storage
        args.value = 3
        storage = alloc[addr]['storage']
        for key in storage:
            if addr == NEW:
                amend_storage(addr, key, storage[key], args)
            elif addr == SYS_CONF and key == KEY:
                amend_storage(addr, key, storage[key], args)

        # amend code
        args.value = 2
        amend_code(addr, alloc[addr]['code'], args)


if __name__ == '__main__':
    main()
