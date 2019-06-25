#!/usr/bin/env python3
"""
Test case of checkCreateContractPermission in sysconfig
"""

import argparse
from txtool_utils import get_receipt, rpc_request, send_tx


def main():
    user_privkey = '0xb3964278651fd8a24fa00aeef2a831fb7574f25a2be2ee9e951d0226ee01dd5b'
    code = '0x608060405234801561001057600080fd5b5060df8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c146078575b600080fd5b348015605957600080fd5b5060766004803603810190808035906020019092919050505060a0565b005b348015608357600080fd5b50608a60aa565b6040518082815260200191505060405180910390f35b8060008190555050565b600080549050905600a165627a7a723058205aed214856a5c433292a354261c9eb88eed1396c83dabbe105bde142e49838ac0029'

    opts = parse_arguments()
    version = opts.version

    # send create contract tx
    tx_hash = send_tx(user_privkey, code=code, version=version)
    # get_receipt
    receipt = get_receipt(tx_hash)
    assert receipt['errorMessage'] == 'No contract permission.'
    print(">>> Test create contract permission denied successfully!")


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", help="Tansaction version.", type=int)

    return parser.parse_args()


if __name__ == '__main__':
    main()
