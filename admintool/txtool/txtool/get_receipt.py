#!/usr/bin/env python
# coding=utf-8

from __future__ import print_function
import argparse
from pathlib import Path
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
from util import findDict, remove_hex_0x
from util import run_command
import simplejson

def save_topcis(topics):
    if topics is not None:
        topicsfile = open("../output/transaction/topics", "w+")
        simplejson.dump(topics, topicsfile)
        topicsfile.close()


def save_contract_address(contract_address):
    if contract_address is not None:
        addressfile = open("../output/transaction/contract_address", "w+")
        addressfile.write(contract_address)
        addressfile.close()


def get_transaction_hash():
    with open("../output/transaction/hash", 'r') as hashfile:
        tx_hash = hashfile.read()
        return tx_hash


def get_receipt_by(tx_hash):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getTransactionReceipt", tx_hash)
    except:
        return None

    return response

def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--tx", help="Transaction hash with or without 0x prefix.")
    opts = parser.parse_args()

    return opts.tx

def main():
    compile_path = Path("../output/transaction")
    if not compile_path.is_dir():
        command = 'mkdir -p ../output/transaction'.split()
        for line in run_command(command):
            print(line)

    tx_hash = parse_arguments()
    if tx_hash is None:
        tx_hash = get_transaction_hash()

    receipt = get_receipt_by(remove_hex_0x(tx_hash))
    print(str(receipt))
    print("保存topics到../output/transaction/topics")
    topics = _log_topics(receipt)
    save_topcis(topics)

    contract_address = findDict(receipt, 'contractAddress')
    save_contract_address(contract_address)


def _log_topics(receipt):
    result_list = []
    logs = findDict(receipt, 'logs')
    for log in logs:
        topics = findDict(log, 'topics')
        result_list = list(set(result_list + topics))

    return result_list

if __name__ == "__main__":
    main()
