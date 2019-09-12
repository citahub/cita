#!/usr/bin/env python3
# -*- coding:utf-8 -*-
"""
Amend the code of system contract.
"""

import argparse
import functools
import json
import logging
import subprocess
import sys
import time
from jsonrpcclient.http_client import HTTPClient

LATEST_VERSION = 2
AMEND_ADDR = '0xffffffffffffffffffffffffffffffffff010002'
SYS_CONF = '0xffffffffffffffffffffffffffffffffff020000'
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
    try:
        if code:
            code = addr + code[2:]
            print('code:', code)
            args.code = code
            tx_hash = send_tx(args)
            receipt = get_receipt(tx_hash, args.url)
            if receipt['errorMessage']:
                logging.critical('amend code of %s error: %s', addr,
                                 receipt['errorMessage'])
                sys.exit(1)
    except Exception as exception:
        logging.critical('amend code of %s exception: %s', addr, exception)
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
        '--privkey', required=True, help='The admin private key.')
    parser.add_argument('--url', required=True, help='The url of the chain.')

    args = parser.parse_args()
    return args


def main():
    """ Load the genesis file and amend """
    args = parse_arguments()

    with open('../../genesis.json', 'r') as gene:
        genesis = json.load(gene)

    alloc = genesis['alloc']
    for addr in alloc:
        # amend code
        args.value = 2
        if addr == SYS_CONF:
            amend_code(addr, alloc[addr]['code'], args)
            print(f'amend code successfully')


if __name__ == '__main__':
    main()
