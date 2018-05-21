#!/usr/bin/env python3
# coding=utf-8

import argparse
from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint
from util import remove_hex_0x
from log import logger


def get_code(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getCode", params)
    except:
        return None

    return response


def contract_address_infile():
    with open("../output/transaction/contract_address") as addressfile:
        address = addressfile.read()
        return address


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("address", help="20 bytes ethereum compatiable address.")
    parser.add_argument("number", help="integer block number(Hex string)")

    args = parser.parse_args()

    address = args.address

    params = [remove_hex_0x(address), args.number]
    resp = get_code(params)
    if resp is not None:
        print(resp)


if __name__ == "__main__":
    main()
