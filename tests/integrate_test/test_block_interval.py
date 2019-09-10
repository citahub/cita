#!/usr/bin/env python3

import argparse
import time
from txtool_utils import send_tx, get_receipt, get_metadata


def main():
    print(f'a) Test default block interval...')
    metadata = get_metadata()
    assert metadata['blockInterval'] == 3000

    print(f'b) Send transaction to change block interval...')
    # send tx to change the block interval
    admin_privkey = '0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6'
    system_contract_address = '0xffffffffffffffffffffffffffffffffff020000'
    code = 'de7aa05d000000000000000000000000000000000000000000000000000000003b9aca00'  # setBlockInterval(1000000000)

    opts = parse_arguments()
    version = opts.version

    tx_hash = send_tx(
        admin_privkey, to=system_contract_address, code=code, version=version)
    time.sleep(5)       # ensure tx execution.
    receipt = get_receipt(tx_hash)
    assert receipt['errorMessage'] == None, "Transaction execution failed"

    metadata = get_metadata()
    assert metadata['blockInterval'] == 1000000000, "Block interval not changed"


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", help="Tansaction version.", type=int)

    return parser.parse_args()


if __name__ == '__main__':
    main()
