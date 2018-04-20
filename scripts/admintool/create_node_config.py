#!/usr/bin/env python
# coding=utf-8

import toml
import os
import copy
import sys


def main():
    path = os.path.join(sys.argv[1], "node" + sys.argv[3])
    secret_path = os.path.join(path, "privkey")
    with open(secret_path, "r") as secret_key:
        signer = secret_key.read()

    params = dict(signer=signer)
    dump_path = os.path.join(path, "consensus.toml")
    with open(dump_path, "w") as f:
        toml.dump(params, f)
    f.close()


if __name__ == '__main__':
    main()
