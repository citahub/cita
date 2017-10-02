#!/usr/bin/env python
# coding=utf-8

import argparse
from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint
from util import remove_hex_0x

def get_code(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getCode", params)
    except:
        return None

    return response


def block_number(number):
    result = 0
    if number.startswith('0x') or number.startswith('0X'):
        result = int(number[2:], 16)
    elif number == 'pending' or number == 'earliest' or number == 'latest':
        result = number
    else:
        result = int(number, 10)

    return result


def contract_address_infile():
    with open("../output/transaction/contract_address") as addressfile:
        address = addressfile.read()
        return address


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("address", help="20 bytes ethereum compatiable address.")
    parser.add_argument("number", help="block number")

    args = parser.parse_args()

    address = args.address

    params = [remove_hex_0x(address), block_number(args.number)]
    resp = get_code(params)
    if resp is not None:
        print resp

if __name__ == "__main__":
    main()