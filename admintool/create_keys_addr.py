#!/usr/bin/env python
# coding=utf-8


import os
import sys
<<<<<<< c4f6c3c74e62821430fb3164ee26ced3a0eaf368
from secp256k1 import PrivateKey
from rlp.utils import decode_hex, encode_hex
from utils import privtoaddr

=======
import random
import pysodium
import binascii
>>>>>>> blake2b and ed25519

def main():
    if len(sys.argv) == 2:
        path = sys.argv[1]
    else:
        path = os.path.join(sys.argv[1], "node" + sys.argv[2])
    dump_path = os.path.join(path, "privkey")
<<<<<<< c4f6c3c74e62821430fb3164ee26ced3a0eaf368
    privkey = PrivateKey()
    sec_key = privkey.serialize()
    with open(dump_path, "w") as f:
        f.write(sec_key)

    auth_path = os.path.join(sys.argv[1], "authorities")
    authority = encode_hex(privtoaddr(decode_hex(sec_key)))
    with open(auth_path, "a") as auth_file:
        auth_file.write("0x" + authority + "\n")

=======
    pk, sk = pysodium.crypto_sign_keypair()
    f = open(dump_path, "w")
    f.write(binascii.b2a_hex(sk))
    f.close()
    auth_path = os.path.join(sys.argv[1], "authorities")
    authority = binascii.b2a_hex(pysodium.crypto_generichash_blake2b_salt_personal(pk, key = "CryptapeCryptape")[12:])
    auth_file = open(auth_path, "a")
    auth_file.write("0x" + authority + "\n")
    auth_file.close()
>>>>>>> blake2b and ed25519

if __name__ == '__main__':
    main()
