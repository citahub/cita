#!/usr/bin/env python
# coding=utf-8

from __future__ import print_function
import sys
import binascii
from pathlib import Path
from util import which, run_command
from ecdsa import SigningKey, SECP256k1
import sha3


# 检查是否安装了openssl
openssl_installed = which("openssl")
if not openssl_installed:
    sys.exit("Openssl not installed.Check txtool/README.md and prerequest_sudo.sh for more infomation.")

accounts_path = Path("../output/accounts")
if not accounts_path.is_dir():
    command = 'mkdir -p ../output/accounts'.split()
    for line in run_command(command):
        print(line)


def save_privkey(privkey):
    print("私钥{}".format(privkey))
    print("生成私钥保存在output/accounts/privkey中")
    privkey_file = open("../output/accounts/privkey", "w+")
    privkey_file.write(privkey)
    privkey_file.close()


def save_pubkey(pubkey):
    print("公钥{}".format(pubkey))
    print("生成公钥保存在output/accounts/pubkey中")
    pubkey_file = open("../output/accounts/pubkey", "w+")
    pubkey_file.write(pubkey)
    pubkey_file.close()

def save_address(address):
    address_file = open("../output/accounts/address", "w+")
    address_file.write(address)
    address_file.close()


def main():
    keccak = sha3.keccak_256()
    priv = SigningKey.generate(curve=SECP256k1)
    pub = priv.get_verifying_key().to_string()

    keccak.update(pub)
    address = keccak.hexdigest()[24:]

    save_privkey(binascii.hexlify(priv.to_string()))
    save_pubkey(binascii.hexlify(pub))
    save_address(address)


if __name__ == "__main__":
    main()
