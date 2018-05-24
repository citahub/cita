#!/usr/bin/env python3
# coding=utf-8

from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint
import binascii

def block_number():
    try:
        url = endpoint()
        response = HTTPClient(url).request("cita_blockNumber", [])
    except:
        return None

    return response


def main():
    number = block_number()
    if number:
        print(int(number, 16))
    else:
        print("None")


if __name__ == "__main__":
    main()
