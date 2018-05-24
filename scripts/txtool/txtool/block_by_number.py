#!/usr/bin/env python3
# coding=utf-8

import argparse
from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint
from util import findDict, remove_hex_0x


def block_by_number(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("cita_getBlockByNumber", params)
    except:
        return None

    return response


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("number", help="block number as param")
    parser.add_argument('--detail', dest='detail', action='store_true')
    parser.add_argument('--no-detail', dest='detail', action='store_false')
    parser.set_defaults(detail=True)
    args = parser.parse_args()

    params = [args.number, args.detail]
    resp = block_by_number(params)


if __name__ == "__main__":
    main()
