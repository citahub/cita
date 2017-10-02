#!/usr/bin/env python
# coding=utf-8

from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint


def peer_count():
    try:
        url = endpoint()
        response = HTTPClient(url).request("net_peerCount", "")
    except:
        return None

    return response


def main():
    count = peer_count()
    if count is not None:
        print int(count, 16)
    else:
        print "Please check CITA is on."

if __name__ == "__main__":
    main()