"""
Amend the code of [System contract](0xffffffffffffffffffffffffffffffffff020000).
"""

import argparse
import json
from utils import rpc_request, get_receipt, amend_code, send_tx, amend_storage

SYS_CONF = '0xffffffffffffffffffffffffffffffffff020000'
Cert_Revoke_Manager = '0xffffffffffffffffffffffffffffffffff020030'

def parse_arguments():
    """ parse the arguments: privkey, url """
    parser = argparse.ArgumentParser()
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
        if addr == SYS_CONF:
            # amend code
            args.value = 2
            amend_code(addr, alloc[addr]['code'], args)
            print(f'amend system config contract code successfully')
        if addr == Cert_Revoke_Manager:
            # amend code
            args.value = 2
            amend_code(addr, alloc[addr]['code'], args)

            # amend storage
            args.value = 3
            storage = alloc[addr]['storage']
            for key in storage:
                amend_storage(addr, key, storage[key], args)
            print(f'amend cert remoke contract code successfully')




if __name__ == '__main__':
    main()
