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

    kwargs = {
        '--privkey': privkey,
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
    operator_address = '0x36a60d575b0dee0423abb6a57dbc6ca60bf47545'
    code = '0x606060405260008055341561001357600080fd5b60f2806100216000396000f3006060604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f1460585780636d4ce63c14606a578063d826f88f146090575b600080fd5b3415606257600080fd5b606860a2565b005b3415607457600080fd5b607a60b4565b6040518082815260200191505060405180910390f35b3415609a57600080fd5b60a060bd565b005b60016000808282540192505081905550565b60008054905090565b600080819055505600a165627a7a72305820906dc3fa7444ee6bea2e59c94fe33064e84166909760c82401f65dfecbd307d50029'

    opts = parse_arguments()
    version = opts.version

    operator_balance_old = get_balance(operator_address)
    tx_hash = send_tx(admin_privkey, code, version)
    receipt = get_receipt(tx_hash)
    operator_balance_new = get_balance(operator_address)
    print('[operator.address]:{}'.format(operator_address))
    print('[operator.balance old]:{}'.format(operator_balance_old))
    print('[operator.balance]:{}'.format(operator_balance_new))
    print('[quotaUsed]:{}'.format(receipt['quotaUsed']))
    assert operator_balance_new - operator_balance_old == int(receipt['quotaUsed'], 16)

    print('>>> Test fee back successfully!')

def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--version", help="Tansaction version.", default=LATEST_VERSION, type=int)

    opts = parser.parse_args()
    
    return opts

if __name__ == '__main__':
    main()
