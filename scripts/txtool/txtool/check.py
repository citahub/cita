#!/usr/bin/env python3
# coding=utf-8

# TODO List all the acceptable params
# TODO Handle passed params


from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
from log import logger
# '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":1}'


def check_cita_status():
    result_status = False
    try:
        url = endpoint()
        response = HTTPClient(url).request("cita_blockNumber", [])
        result_status = response > 0
    except:
        result_status = False
    finally:
        return result_status


if __name__ == '__main__':
    if check_cita_status():
        print("CITA is on.")
    else:
        print("CITA is not working.")
