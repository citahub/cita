#!/usr/bin/env python  
# coding=utf-8  
  
import json  
import os  
import copy  
import sys
  
def make_json():  
    path = os.path.join(sys.argv[1], "node"+sys.argv[3])  
    #if os.path.split(path)[1] != "data":  
    #    pathVar.set("The path are incorrect, please choose again!")  
    #    return 1  
    duration = int(sys.argv[4])
    is_test = sys.argv[5]=="true"
    block_tx_limit = int(sys.argv[6])
    tx_filter_size = int(sys.argv[7])
    secret_path = os.path.join(path, "privkey")
    secret_key = open(secret_path, "r")
    signer = secret_key.read()
    data = dict() 
    auth_path = os.path.join(sys.argv[1], "authorities")
    authority_file = open(auth_path, "r")
    authorities = []
    for authority in authority_file:
        authorities.append(authority.strip('\n'))

    params = dict(authorities=authorities, duration=duration, is_test=is_test, signer=signer,
                    block_tx_limit=block_tx_limit, tx_filter_size=tx_filter_size)
    name =  sys.argv[2]
    if name == "tendermint":
        tendermint = dict(params=params)
        engine = dict(Tendermint=tendermint)
    else:
        authorityround = dict(params=params)
        engine = dict(AuthorityRound=authorityround) 
    
    data["name"] = name
    data["engine"] = engine 
    dump_path = os.path.join(path, "consensus.json")  
    f = open(dump_path, "w")  
    json.dump(data, f, indent=4)  
    f.close()

make_json()  
