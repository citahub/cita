import os
import copy
import sys
import random
from secp256k1 import PrivateKey, PublicKey
import rlp
from rlp.utils import decode_hex, encode_hex 
from utils import privtopub,privtoaddr

def mk_privkey(seed):
    return sha3(seed)

def mk_keys_addr():
    if len(sys.argv)==2:
        path = sys.argv[1]
    else:
        path = os.path.join(sys.argv[1],"node" + sys.argv[2])
    dump_path = os.path.join(path, "privkey")
    privkey = PrivateKey() 
    sec_key = privkey.serialize()
    f = open(dump_path, "w")
    f.write(sec_key)
    f.close()
    auth_path = os.path.join(sys.argv[1], "authorities")
    authority = encode_hex(privtoaddr(decode_hex(sec_key)))
    auth_file = open(auth_path, "a")
    auth_file.write("0x" + authority + "\n")
    auth_file.close()

mk_keys_addr()
