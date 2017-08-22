#!/usr/bin/env python
# coding=utf-8


import os
import sys
from secp256k1 import PrivateKey
from rlp.utils import decode_hex, encode_hex
from utils import privtoaddr


def main():
    if len(sys.argv) == 2:
        path = sys.argv[1]
    else:
        path = os.path.join(sys.argv[1], "node" + sys.argv[2])
    dump_path = os.path.join(path, "privkey")
    privkey = PrivateKey()
    sec_key = privkey.serialize()
    with open(dump_path, "w") as f:
        f.write(sec_key)

    auth_path = os.path.join(sys.argv[1], "authorities")
    authority = encode_hex(privtoaddr(decode_hex(sec_key)))
    with open(auth_path, "a") as auth_file:
        auth_file.write("0x" + authority + "\n")


if __name__ == '__main__':
    main()
