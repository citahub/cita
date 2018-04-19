#!/usr/bin/env python
# coding=utf-8

import toml
import os
import copy
import sys


def main():
    path = os.path.join(sys.argv[1], "node" + sys.argv[3])
    is_test = sys.argv[4] == "true"
    secret_path = os.path.join(path, "privkey")
    with open(secret_path, "r") as secret_key:
        signer = secret_key.read()
    auth_path = os.path.join(sys.argv[1], "authorities")

    authorities = []
    with open(auth_path, "r") as authority_file:
        for authority in authority_file:
            authorities.append(authority.strip('\n'))

    params = dict(is_test=is_test, signer=signer)
    dump_path = os.path.join(path, "consensus.toml")
    with open(dump_path, "w") as f:
        toml.dump(params, f)
    f.close()


if __name__ == '__main__':
    main()
