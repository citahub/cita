#!/usr/bin/env python3

"""
Test case of fee back to operator in charge economical mode.
"""

import functools
import subprocess
import time
import argparse
from jsonrpcclient.http_client import HTTPClient

LATEST_VERSION = 1

def send_tx(privkey, code="", version=LATEST_VERSION):
    """
    Send a transaction

    python3 make_tx.py 
    --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6" 
    --code ""   

    python3 send_tx.py 
    """

    version_manager = "ffffffffffffffffffffffffffffffffff020011"

    kwargs = {
        '--privkey': privkey,
        '--to': version_manager,
        '--code': code,
        '--version': str(version),
    }
    args = functools.reduce(
        lambda lst, kv: lst + list(kv),
        kwargs.items(),
        [],
    )
    print(['python3', 'make_tx.py', *args])
    subprocess.call(['python3', 'make_tx.py', *args])
    subprocess.call(['python3', 'send_tx.py'])
    with open('../output/transaction/hash') as fobj:
        return fobj.read().strip()

def get_balance(addr):
    """ Get the balance of an address """
    return int(rpc_request('getBalance', [addr, 'pending']), 16)

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

def main():
    """ Run the test. """
    admin_privkey = '0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6'
    code = '0x62ddb8e10000000000000000000000000000000000000000000000000000000000000001'

    opts = parse_arguments()
    version = opts.version

    tx_hash = send_tx(admin_privkey, code, version)
    get_receipt(tx_hash)

    print('>>> Test fee back successfully!')

def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--version", help="Tansaction version.", default=LATEST_VERSION, type=int)

    opts = parser.parse_args()
    
    return opts

if __name__ == '__main__':
    main()
