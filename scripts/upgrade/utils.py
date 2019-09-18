
import sys
import time
import logging
import functools
import subprocess
from jsonrpcclient.http_client import HTTPClient

LOCAL = 'http://127.0.0.1:1337'
AMEND_ADDR = '0xffffffffffffffffffffffffffffffffff010002'

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

def amend_storage(addr, key, val, args):
    """ Amend the storage: key and value """
    try:
        code = addr + key[2:].zfill(64) + val[2:].zfill(64)
        print('code:', code)
        args.code = code
        tx_hash = send_tx(args)
        receipt = get_receipt(tx_hash, args.url)
        if receipt['errorMessage']:
            logging.critical('amend storage of %s[%s] error: %s', addr, key,
                             receipt['errorMessage'])
            sys.exit(1)
    except Exception as exception:
        logging.critical('amend storage of %s[%s] exception: %s', addr, key,
                         exception)
        sys.exit(1)
