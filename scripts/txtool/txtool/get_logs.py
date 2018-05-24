#!/usr/bin/env python3
# coding=utf-8

from __future__ import print_function
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
import argparse
import simplejson


def get_topics():
    with open("../output/transaction/topics", 'r') as topicfile:
        topics = simplejson.load(topicfile)
        return topics


def get_logs(topics, from_block, to_block):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getLogs", [{"topics":topics, "fromBlock":from_block, "toBlock":to_block}])
    except:
        return None

    return response

def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fromBlock", default="0")
    parser.add_argument("--toBlock", default="latest")
    opts = parser.parse_args()

    return opts.fromBlock, opts.toBlock

def main():
    from_block, to_block = parse_arguments()
    topics = get_topics()
    logger.debug(topics)
    resp = get_logs(topics, from_block, to_block)
    print(resp)


if __name__ == "__main__":
    main()
