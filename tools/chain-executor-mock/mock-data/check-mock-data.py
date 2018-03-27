#!/usr/bin/env python

"""
  Check mock data from a yaml file by jsonrpc call.

  NOTE:
    * The yaml (mock-data.yaml) for generate mock data should be the same file
      for check mock data
"""

import argparse

import yaml
from jsonrpcclient.http_client import HTTPClient


DEFAULT_URL = 'http://localhost:1337'


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-l', '--url',
                        default=DEFAULT_URL, help=u'JSONRPC server url')
    parser.add_argument('-m', '--mock-data',
                        required=True,
                        help=u'YAML format mock data (mock-data.yaml)')
    return parser.parse_args()


def main():
    args = parse_args()
    with open(args.mock_data) as f:
        data = yaml.load(f)
    client = HTTPClient(args.url)

    assert int(client.request('net_peerCount'), 16) == 0
    assert int(client.request('cita_blockNumber'), 16) == len(data['blocks'])
    print('JSONRPC call check all OK!')


if __name__ == '__main__':
    main()
