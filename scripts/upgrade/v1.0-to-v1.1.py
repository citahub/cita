"""
Amend the code of [System contract](0xffffffffffffffffffffffffffffffffff020000).
"""

import argparse
import json
from utils import rpc_request, get_receipt, amend_code, send_tx

SYS_CONF = '0xffffffffffffffffffffffffffffffffff020000'

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
        # amend code
        args.value = 2
        if addr == SYS_CONF:
            amend_code(addr, alloc[addr]['code'], args)
            print(f'amend code successfully')


if __name__ == '__main__':
    main()
