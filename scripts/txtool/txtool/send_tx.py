#!/usr/bin/env python3
# coding=utf-8

from __future__ import print_function
import argparse
from pathlib import Path
from log import logger
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
from util import findDict, run_command


def save_transaction_hash(tx_hash):
    deployfile = open("../output/transaction/hash", "w+")
    deployfile.write(tx_hash)
    deployfile.close()


def get_deploy_code():
    with open("../output/transaction/deploycode", 'r') as deployfile:
        code = deployfile.read()
        return code


def send_transaction(params):
    try:
        url = endpoint()
        response = HTTPClient(url).request("cita_sendRawTransaction", params)
    except Exception as e:
        logger.error(e)
        return None

    return response


def send_txs(params):
    try:
        for item in params:
            response = send_transaction(item)
    except:
        return None

    return response


def parse_arguments():
    args = None
    parser = argparse.ArgumentParser()
    parser.add_argument('--codes', nargs="+", help="send transaction params.")
    opts = parser.parse_args()
    if opts.codes:
        args = opts.codes

    return args


def main():
    compile_path = Path("../output/transaction")
    if not compile_path.is_dir():
        command = 'mkdir -p ../output/transaction'.split()
        for line in run_command(command):
            logger.debug(line)

    params = parse_arguments()
    if params is None:
        params = get_deploy_code()
        resp = send_transaction(params)
    elif isinstance(params, list) and len(params) > 1:
        resp = send_txs(params)
    else:
        resp = send_transaction(params)

    logger.info("transaction hash is stored in../output/transaction/hash")
    if resp is not None:
        tx_hash = findDict(resp, 'hash')
        save_transaction_hash(tx_hash)


if __name__ == "__main__":
    main()
