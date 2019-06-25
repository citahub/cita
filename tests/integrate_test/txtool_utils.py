#!/usr/bin/env python3

import functools
import subprocess
import time
from jsonrpcclient.http_client import HTTPClient

LATEST_VERSION = 2


def send_tx(privkey,
            code="",
            to="",
            version=LATEST_VERSION):
    """
    Step 1: make tx

    python3 make_tx.py --privkey "$$" --to "$$" --code "$$" -- version "$$"

    Step 2: Send tx

    python3 send_tx.py
    """

    kwargs = {
        '--privkey': privkey,
        '--to': to,
        '--code': code,
        '--version': str(version),
    }
    args = functools.reduce(
        lambda lst, kv: lst + list(kv),
        kwargs.items(),
        [],
    )
    print(['python3', 'make_tx.py', *args])
    subprocess.call(['python3', 'make_tx.py', *args, '--no-newcrypto'])
    subprocess.call(['python3', 'send_tx.py'])
    with open('../output/transaction/hash') as fobj:
        return fobj.read().strip()


def get_receipt(tx_hash, retry=8):
    """ Get receipt of a transaction """
    while retry > 0:
        receipt = rpc_request('getTransactionReceipt', [tx_hash])
        if receipt is not None:
            return receipt
        time.sleep(4)
        retry -= 1


def rpc_request(method, params):
    """ Send a jsonrpc request to default url. """
    client = HTTPClient('http://127.0.0.1:1337')
    return client.request(method, params)


def get_balance(addr):
    """ Get the balance of an address """
    return int(rpc_request('getBalance', [addr, 'pending']), 16)
