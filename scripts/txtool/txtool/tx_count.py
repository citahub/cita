#!/usr/bin/env python3
# coding=utf-8

import argparse
from log import logger
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint



def get_tx_count(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getTransactionCount", params)
        logger.debug(response)
    except:
        return None

    return response


def address_infile():
    with open("../output/accounts/address") as addressfile:
        address = addressfile.read()
        return address


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-a", "--address", help="20 bytes ethereum compatiable address.")
    parser.add_argument("number", help="integer block number(hex string)")

    args = parser.parse_args()

    address = args.address
    if args.address is None:
        address = address_infile()

    params = [address, args.number]
    logger.debug(params)
    resp = get_tx_count(params)
    if resp is None:
        print('None')
    else:
        print(int(resp, 16))


if __name__ == "__main__":
    main()
