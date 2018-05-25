#!/usr/bin/env python3
# coding=utf-8

import argparse
from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint
from util import findDict


def block_by_hash(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("cita_getBlockByHash", params)
    except:
        return None

    return response


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("hash", help="block hash as param")
    parser.add_argument('--detail', dest='detail', action='store_true')
    parser.add_argument('--no-detail', dest='detail', action='store_false')
    parser.set_defaults(detail=True)
    args = parser.parse_args()

    params = [args.hash, args.detail]
    resp = block_by_hash(params)


if __name__ == "__main__":
    main()
