#!/usr/bin/env python3
# coding=utf-8

from __future__ import print_function
import sys
import binascii
import argparse
from pathlib import Path
from util import which, run_command, hex2bytes
from ecdsa import SigningKey, SECP256k1
import sha3
import pysodium
from log import logger

# check openssl
openssl_installed = which("openssl")
if not openssl_installed:
    sys.exit("Openssl not installed.Check txtool/README.md and prerequest_sudo.sh for more infomation.")

accounts_path = Path("../output/accounts")
if not accounts_path.is_dir():
    command = 'mkdir -p ../output/accounts'.split()
    for line in run_command(command):
        logger.debug(line)


def save_privkey(privkey):
    logger.debug("private key {}".format(privkey))
    logger.info("the private key stores in output/accounts/privkey")
    with open("../output/accounts/privkey", "w+") as privkey_file:
        privkey_file.write(privkey)


def save_pubkey(pubkey):
    logger.debug("public key {}".format(pubkey))
    logger.info("the public key is stored in output/accounts/pubkey")
    with open("../output/accounts/pubkey", "w+") as pubkey_file:
        pubkey_file.write(pubkey)


def save_address(address):
    with open("../output/accounts/address", "w+") as address_file:
        address_file.write(address)


def generate(privkey=None, newcrypto=False):
    if newcrypto:
        _generate_new(privkey)
    else:
        _generate_old(privkey)


def _generate_old(privkey=None):
    keccak = sha3.keccak_256()
    if privkey is None:
        priv = SigningKey.generate(curve=SECP256k1)
    else:
        priv = SigningKey.from_string(hex2bytes(privkey), curve=SECP256k1)

    pub = priv.get_verifying_key().to_string()

    keccak.update(pub)
    address = keccak.hexdigest()[24:]

    save_privkey(binascii.hexlify(priv.to_string()))
    save_pubkey(binascii.hexlify(pub))
    save_address(address)


def _generate_new(privkey=None):
    pk, sk = pysodium.crypto_sign_keypair()
    save_privkey(binascii.b2a_hex(sk))

    save_pubkey(binascii.b2a_hex(pk))

    address = binascii.b2a_hex(pysodium.crypto_generichash_blake2b_salt_personal(pk, key = "CryptapeCryptape")[12:])
    save_address(address)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--newcrypto', dest='newcrypto', action='store_true')
    parser.add_argument('--no-newcrypto', dest='newcrypto', action='store_false')
    parser.set_defaults(newcrypto=False)
    args = parser.parse_args()
    generate(None, args.newcrypto)


if __name__ == "__main__":
    main()
