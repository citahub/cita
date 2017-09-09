#!/usr/bin/env python
# coding=utf-8

import argparse
from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint


def build_params(sender, to, data, number):
    dictionary = {"from": "" if sender is None else sender,
                  "to": "" if to is None else to,
                  "data": data}
    return [dictionary, block_number(number)]


def block_number(number):
    result = 0
    if number.startswith('0x') or number.startswith('0X'):
        result = int(number[2:], 16)
    elif number == 'pending' or number == 'earliest' or number == 'latest':
        result = number
    else:
        result = int(number, 10)

    return result


def call(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_call", params)
    except:
        return None

    return response


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--sender", help="20 bytes ethereum compatiable address.")
    parser.add_argument("to", help="20 bytes ethereum compatiable address.")
    parser.add_argument("data", help="compiled solidity function.")
    parser.add_argument("number", help="block number", nargs='?')

    args = parser.parse_args()

    sender = args.sender
    if args.sender is None:
        sender = ""
    
    number = args.number
    if args.number is None:
        number = 'latest'

    params = build_params(sender, args.to, args.data, number)
    resp = call(params)
    if resp is not None:
        print resp


if __name__ == "__main__":
    main()
