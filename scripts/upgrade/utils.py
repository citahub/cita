
import sys
import time
import logging
import functools
import subprocess
import json
from jsonrpcclient.http_client import HTTPClient

LOCAL = 'http://127.0.0.1:1337'

def set_code(args):
    kwargs = {
        '--admin-private-key': args.privkey,
        '--address': args.address,
        '--content': args.code,
        '--url': args.url,
        '--algorithm': args.algorithm
    }
    args = functools.reduce(
        lambda lst, kv: lst + list(kv),
        kwargs.items(),
        [],
    )
    
    print(['cita-cli', 'amend', 'code', *args])
    ret_bytes = subprocess.check_output(['cita-cli', 'amend', 'code', *args])
    ret_json = json.loads(ret_bytes)
    t_hash = ret_json['result']['hash']
    return t_hash

def set_kv(args):
    kwargs = {
        '--admin-private-key': args.privkey,
        '--address': args.address,
        '--url': args.url,
        '--algorithm': args.algorithm
    }
    args_list = functools.reduce(
        lambda lst, kv: lst + list(kv),
        kwargs.items(),
        [],
    )
    args_list.append("--kv")
    args_list.append(args.k)
    args_list.append(args.v)

    print(['cita-cli', 'amend', 'set-h256', *args_list])
    ret_bytes = subprocess.check_output(['cita-cli', 'amend', 'set-h256', *args_list])

    ret_json = json.loads(ret_bytes)
    t_hash = ret_json['result']['hash']
    return t_hash

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
            # code = addr + code[2:]
            print('code:', code)
            args.code = code
            args.address = addr
            tx_hash = set_code(args)
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
        args.k = key
        args.v = val
        tx_hash = set_kv(args)
        receipt = get_receipt(tx_hash, args.url)
        if receipt['errorMessage']:
            logging.critical('amend storage of %s[%s] error: %s', addr, key,
                             receipt['errorMessage'])
            sys.exit(1)
    except Exception as exception:
        logging.critical('amend storage of %s[%s] exception: %s', addr, key,
                         exception)
        sys.exit(1)
