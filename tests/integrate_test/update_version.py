#!/usr/bin/env python3
"""
Test case of fee back to operator in charge economical mode.
"""

import argparse
from txtool_utils import get_receipt, rpc_request, send_tx, get_balance


def main():
    """ Run the test. """
    admin_privkey = '0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6'
    code = '0x62ddb8e10000000000000000000000000000000000000000000000000000000000000001'
    version_manager = "ffffffffffffffffffffffffffffffffff020011"

    opts = parse_arguments()
    version = opts.version

    tx_hash = send_tx(
        admin_privkey, code=code, to=version_manager, version=version)
    get_receipt(tx_hash)
    print('>>> Update version successfully!')


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", help="Tansaction version.", type=int)

    return parser.parse_args()


if __name__ == '__main__':
    main()
