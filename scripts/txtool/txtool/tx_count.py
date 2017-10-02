#!/usr/bin/env python
# coding=utf-8

import argparse
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint

def get_transaction_count(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getTransactionCount", params)
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


def address_infile():
    with open("../output/accounts/address") as addressfile:
        address = addressfile.read()
        return address


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-a", "--address", help="20 bytes ethereum compatiable address.")
    parser.add_argument("number", help="block number")

    args = parser.parse_args()

    address = args.address
    if args.address is None:
        address = address_infile()

    params = [address, block_number(args.number)]
    resp = get_transaction_count(params)
    print int(resp, 16)

if __name__ == "__main__":
    main()
