#!/usr/bin/env python3
# coding=utf-8

from __future__ import print_function
import argparse
from pathlib import Path
from log import logger
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
from util import run_command


def get_transaction_hash():
    with open("../output/transaction/hash", 'r') as hashfile:
        tx_hash = hashfile.read()
        return tx_hash


def transaction_by_hash(tx_hash):
    try:
        url = endpoint()
        response = HTTPClient(url).request("getTransaction", tx_hash)
    except:
        return None

    return response


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--tx", help="Transaction hash with or without 0x prefix.")
    opts = parser.parse_args()

    return opts.tx


def main():
    compile_path = Path("../output/transaction")
    if not compile_path.is_dir():
        command = 'mkdir -p ../output/transaction'.split()
        for line in run_command(command):
            logger.debug(line)

    tx_hash = parse_arguments()
    if tx_hash is None:
        tx_hash = get_transaction_hash()

    transaction = transaction_by_hash(tx_hash)
    print(transaction)


if __name__ == "__main__":
    main()
