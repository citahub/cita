#!/usr/bin/env python
# coding=utf-8


import os
import sys
import random
import pysodium
import binascii

def main():
    if len(sys.argv) == 2:
        path = sys.argv[1]
    else:
        path = os.path.join(sys.argv[1], "node" + sys.argv[2])
    dump_path = os.path.join(path, "privkey")
    pk, sk = pysodium.crypto_sign_keypair()
    f = open(dump_path, "w")
    f.write(binascii.b2a_hex(sk))
    f.close()
    auth_path = os.path.join(sys.argv[1], "authorities")
    authority = binascii.b2a_hex(pysodium.crypto_generichash_blake2b_salt_personal(pk, key = "CryptapeCryptape")[12:])
    auth_file = open(auth_path, "a")
    auth_file.write("0x" + authority + "\n")
    auth_file.close()

if __name__ == '__main__':
    main()
