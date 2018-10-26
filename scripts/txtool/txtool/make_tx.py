#!/usr/bin/env python3
# coding=utf-8

from __future__ import print_function, absolute_import
import argparse
import binascii
import random
import string
from pathlib import Path
from blockchain_pb2 import Transaction, SignedTransaction, UnverifiedTransaction, Crypto
from util import hex2bytes, run_command, remove_hex_0x, recover_pub
from secp256k1 import PrivateKey
from ethereum.utils import sha3
import pysodium
from generate_account import generate
from block_number import block_number
from url_util import endpoint
from log import logger
from jsonrpcclient.http_client import HTTPClient

LATEST_VERSION = 1

accounts_path = Path("../output/transaction")
if not accounts_path.is_dir():
    command = 'mkdir -p ../output/transaction'.split()
    for line in run_command(command):
        logger.debug(line)


def save_deploy(code):
    with open("../output/transaction/deploycode", "w+") as deployfile:
        deployfile.write(code.decode('utf-8'))


def bin_code():
    with open("../output/compiled/bytecode", 'r') as binfile:
        bytecode = binfile.read()
        return bytecode


def private_key():
    with open("../output/accounts/privkey", 'r') as privfile:
        privkey = privfile.read()
        return privkey


def get_sender(privkey=None, newcrypto=False):
    generate(privkey, newcrypto)
    return _sender_from_file()


def _sender_from_file():
    with open("../output/accounts/address", 'r') as addressfile:
        address = addressfile.read()
        return address


def get_nonce(size=6):
    """Get a random string."""
    return (''.join(
        random.choice(string.ascii_uppercase + string.digits)
        for _ in range(size)))


def get_chainid():
    params = ['latest']
    chainid = 0

    try:
        url = endpoint()
        logger.debug(url)
        response = HTTPClient(url).request("getMetaData", params)
        chainid = response['chainId']
        logger.debug(response)
    except:
        chainid = 0

    logger.debug("final chainId is {}".format(chainid))
    return chainid

def get_chainid_v1():
    params = ['latest']
    chainid = 0

    try:
        url = endpoint()
        logger.debug(url)
        response = HTTPClient(url).request("getMetaData", params)
        chainid = response['chainIdV1']
        logger.debug(response)
    except:
        chainid = "0"

    # padding to 32 bytes
    chainid = int(chainid)
    logger.debug("final chainId is {}".format(chainid))
    return chainid


def generate_deploy_data(current_height,
                         bytecode,
                         value,
                         quota,
                         privatekey,
                         receiver=None,
                         newcrypto=False,
                         version=LATEST_VERSION):
    if newcrypto:
        data = _blake2b_ed25519_deploy_data(current_height, bytecode, value, quota,
                                            privatekey, version, receiver)
    else:
        data = _sha3_secp256k1_deploy_data(current_height, bytecode, value, quota,
                                           privatekey, version, receiver)

    return data


def _blake2b_ed25519_deploy_data(current_height,
                                 bytecode,
                                 value,
                                 quota,
                                 privatekey,
                                 version,
                                 receiver=None):
    sender = get_sender(private_key, True)
    logger.debug(sender)
    nonce = get_nonce()
    logger.debug("nonce is {}".format(nonce))

    tx = Transaction()
    tx.valid_until_block = current_height + 88
    tx.nonce = nonce
    tx.version = version
    if version == 0:
        chainid = get_chainid()
        logger.debug("chainid is {}".format(chainid))
        tx.chain_id = chainid
    elif version == 1:
        chainid = get_chainid_v1()
        logger.debug("chainid_v1 is {}".format(chainid))
        tx.chain_id_v1 = chainid.to_bytes(32, byteorder='big')
    else:
        logger.error("unexpected version {}".format(version))
    if receiver is not None:
        if version == 0:
            tx.to = receiver
        elif version == 1:
            tx.to_v1 = hex2bytes(receiver)
        else:
            logger.error("unexpected version {}".format(version))
    tx.data = hex2bytes(bytecode)
    tx.value = value.to_bytes(32, byteorder='big')
    tx.quota = quota

    message = _blake2b(tx.SerializeToString())
    logger.debug("blake2b msg")
    sig = pysodium.crypto_sign_detached(message, hex2bytes(privatekey))
    logger.debug("sig {}".format(binascii.b2a_hex(sig)))

    pubkey = pysodium.crypto_sign_sk_to_pk(hex2bytes(privatekey))
    logger.debug("pubkey is {}".format(binascii.b2a_hex(pubkey)))
    signature = binascii.hexlify(sig[:]) + binascii.hexlify(pubkey[:])
    logger.debug("signature is {}".format(signature))

    unverify_tx = UnverifiedTransaction()
    unverify_tx.transaction.CopyFrom(tx)
    unverify_tx.signature = hex2bytes(signature)
    unverify_tx.crypto = Crypto.Value('SECP')

    logger.info("unverify_tx is {}".format(
        binascii.hexlify(unverify_tx.SerializeToString())))
    return binascii.hexlify(unverify_tx.SerializeToString())


def _sha3_secp256k1_deploy_data(current_height,
                                bytecode,
                                value,
                                quota,
                                privatekey,
                                version,
                                receiver=None):
    sender = get_sender(privatekey, False)
    if privatekey is None:
        temp = private_key()
        privkey = PrivateKey(hex2bytes(temp))
    else:
        privkey = PrivateKey(hex2bytes(privatekey))

    logger.debug(sender)
    nonce = get_nonce()
    logger.debug("nonce is {}".format(nonce))

    tx = Transaction()
    tx.valid_until_block = current_height + 88
    tx.nonce = nonce
    tx.version = version
    if version == 0:
        chainid = get_chainid()
        logger.debug("chainid is {}".format(chainid))
        tx.chain_id = chainid
    elif version == 1:
        chainid = get_chainid_v1()
        logger.debug("chainid_v1 is {}".format(chainid))
        tx.chain_id_v1 = chainid.to_bytes(32, byteorder='big')
    else:
        logger.error("unexpected version {}".format(version))
    if receiver is not None:
        if version == 0:
            tx.to = receiver
        elif version == 1:
            tx.to_v1 = hex2bytes(receiver)
        else:
            logger.error("unexpected version {}".format(version))
    tx.data = hex2bytes(bytecode)
    tx.value = value.to_bytes(32, byteorder='big')
    tx.quota = quota

    message = sha3(tx.SerializeToString())

    logger.debug("hash message: {}")
    sign_recover = privkey.ecdsa_sign_recoverable(message, raw=True)
    sig = privkey.ecdsa_recoverable_serialize(sign_recover)

    signature = binascii.hexlify(sig[0]) + binascii.hexlify(
        bytes(bytearray([sig[1]])))

    unverify_tx = UnverifiedTransaction()
    unverify_tx.transaction.CopyFrom(tx)
    unverify_tx.signature = hex2bytes(signature)
    unverify_tx.crypto = Crypto.Value('SECP')

    logger.info("unverify_tx is {}".format(
        binascii.hexlify(unverify_tx.SerializeToString())))
    return binascii.hexlify(unverify_tx.SerializeToString())


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--code", help="Compiled contract bytecode.")
    parser.add_argument("--value", type=int, default=0, help="The value to send.")
    parser.add_argument("--quota", type=int, default=1000000, help="The quota(gas limit).")
    parser.add_argument(
        "--privkey", help="private key genearted by secp256k1 alogrithm.")
    parser.add_argument("--to", help="transaction to")
    parser.add_argument(
        '--newcrypto',
        dest='newcrypto',
        action='store_true',
        help="Use ed25519 and blake2b.")
    parser.add_argument(
        '--no-newcrypto',
        dest='newcrypto',
        action='store_false',
        help="Use ecdsa and sha3.")
    parser.add_argument(
        "--version", help="Tansaction version.", default=1, type=int)
    parser.add_argument("--chain_id", default=0, type=int)
    parser.set_defaults(newcrypto=False)

    opts = parser.parse_args()
    return opts


def _params_or_default():
    opts = parse_arguments()
    bytecode = opts.code
    value = opts.value
    quota = opts.quota
    privkey = opts.privkey
    receiver = opts.to
    version = opts.version

    if bytecode is None:
        bytecode = bin_code()

    return (bytecode, value, quota, privkey, receiver, version)


def _blake2b(seed):
    hashed = pysodium.crypto_generichash_blake2b_salt_personal(
        seed, key="CryptapeCryptape")
    return hashed


def main():
    blake2b_ed25519 = parse_arguments().newcrypto
    logger.debug(blake2b_ed25519)
    bytecode, value, quota, privkey, receiver, version = _params_or_default()
    current_height = int(block_number(), 16)
    data = generate_deploy_data(
        current_height, remove_hex_0x(bytecode), value, quota,
        remove_hex_0x(privkey), remove_hex_0x(receiver), blake2b_ed25519, version)
    logger.info("save deploy code to ../output/transaction/deploycode")
    save_deploy(data)


if __name__ == '__main__':
    main()
