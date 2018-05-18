#!/usr/bin/env python3
# coding=utf-8

import toml
import os
import copy
import sys


def main():
    node_path = sys.argv[1]

    secret_path = os.path.join(node_path, "privkey")
    with open(secret_path, "r") as secret_key:
        signer = secret_key.read().strip()

    params = dict(signer=signer)
    dump_path = os.path.join(node_path, "consensus.toml")
    with open(dump_path, "w") as fil:
        toml.dump(params, fil)
    fil.close()


if __name__ == '__main__':
    main()
