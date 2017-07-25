#!/usr/bin/env python  
# coding=utf-8  
  
import json  
import os  
import copy  
import sys
import time
import rlp
from rlp.utils import decode_hex, encode_hex 
from utils import privtopub
  
def make_json():
    path = sys.argv[3]
    if len(sys.argv)==5:
        pubkey = sys.argv[4]
        if pubkey[0:2] != "0x" and pubkey[0:2] != "0X":
            pubkey = "0x"+pubkey
    else:
        secret_path = os.path.join(path, "privkey")
        secret_key = open(secret_path, "r")
        sec_key = secret_key.read()
        pubkey = "0x"+encode_hex(privtopub(decode_hex(sec_key)))[2:]
    crypto = sys.argv[2]
    identifier = sys.argv[1]
    data = dict()
    admin = dict(pubkey=pubkey,crypto=crypto,identifier=identifier)
    timestamp = int(time.time())
    data["prevhash"] = "0x0000000000000000000000000000000000000000000000000000000000000000"
    data["admin"] = admin
    data["timestamp"] = timestamp
    dump_path = os.path.join(path, "genesis.json")
    f = open(dump_path, "w")
    json.dump(data, f, indent=4)
    f.close()

make_json()
