#!/usr/bin/env python
# coding=utf-8

from __future__ import print_function
from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
import simplejson


def get_topics():
    with open("../output/transaction/topics", 'r') as topicfile:
        topics = simplejson.load(topicfile)
        return topics


def get_logs(topics):
    try:
        url = endpoint()
        response = HTTPClient(url).request("eth_getLogs", topics=topics, fromBlock=0)
    except:
        return None

    return response


def main():
    topics = get_topics()
    print(topics)
    resp = get_logs(topics)


if __name__ == "__main__":
    main()
