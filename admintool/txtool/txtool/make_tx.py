#!/usr/bin/env python
# coding=utf-8

from __future__ import print_function
import argparse
import binascii
from pathlib import Path
from transaction_pb2 import Transaction, SignedTransaction, UnverifiedTransaction, Crypto
from util import hex2bytes, run_command, remove_hex_0x, recover_pub
from secp256k1 import PrivateKey
from ethereum.utils import sha3
from tx_count import get_transaction_count
import pysodium

accounts_path = Path("../output/transaction")
if not accounts_path.is_dir():
    command = 'mkdir -p ../output/transaction'.split()
    for line in run_command(command):
        print(line)


def save_deploy(code):
    with open("../output/transaction/deploycode", "w+") as deployfile:
        deployfile.write(code)


def bin_code():
    with open("../output/compiled/bytecode", 'r') as binfile:
        bytecode = binfile.read()
        return bytecode


def private_key():
    with open("../output/accounts/privkey", 'r') as privfile:
        privkey = privfile.read()
        return privkey


def get_sender():
    with open("../output/accounts/address", 'r') as addressfile:
        address = addressfile.read()
        return address


def get_nonce(sender):
    """Get nonce of sender at latest block."""
    nonce = get_transaction_count([sender, 'latest'])
    if nonce is not None:
        nonce = int(nonce, 16)
    else:
        nonce = 0

    print(str(nonce))
    return str(nonce)


def generate_deploy_data(bytecode, privatekey, receiver=None, newcrypto=False):
    if newcrypto:
        data = _blake2b_ed25519_deploy_data(bytecode, privatekey, receiver)
    else:
        data = _sha3_secp256k1_deploy_data(bytecode, privatekey, receiver)

    return data


def _blake2b_ed25519_deploy_data(bytecode, privatekey, receiver=None):
    sender = get_sender()
    print(sender)
    nonce = get_nonce(sender)
    print("nonce is {}".format(nonce))

    tx = Transaction()
    tx.valid_until_block = 4294967296
    tx.nonce = nonce
    if receiver is not None:
        tx.to = receiver
    tx.data = hex2bytes(bytecode)

    message = _blake2b(tx.SerializeToString())
    print("msg is {}".format(message))
    sig = pysodium.crypto_sign_detached(message, hex2bytes(privatekey))
    print("sig {}".format(binascii.b2a_hex(sig)))
    
    pubkey = pysodium.crypto_sign_sk_to_pk(hex2bytes(privatekey))
    print("pubkey is {}".format(binascii.b2a_hex(pubkey)))
    signature = binascii.hexlify(
        sig[:]) + binascii.hexlify(pubkey[:])
    print("signature is {}".format(signature))

    unverify_tx = UnverifiedTransaction()
    unverify_tx.transaction.CopyFrom(tx)
    unverify_tx.signature = hex2bytes(signature)
    unverify_tx.crypto = Crypto.Value('SECP')

    signed_transaction = SignedTransaction()
    signed_transaction.transaction_with_sig.CopyFrom(unverify_tx)
    signed_transaction.tx_hash = _blake2b(unverify_tx.SerializeToString())
    signed_transaction.signer = binascii.b2a_hex(pubkey)
    # print(binascii.hexlify(pub))

    return binascii.hexlify(signed_transaction.SerializeToString())


def _sha3_secp256k1_deploy_data(bytecode, privatekey, receiver=None):
    privkey = PrivateKey(hex2bytes(privatekey))
    sender = get_sender()
    print(sender)
    nonce = get_nonce(sender)
    print("nonce is {}".format(nonce))

    tx = Transaction()
    tx.valid_until_block = 4294967296
    tx.nonce = nonce
    if receiver is not None:
        tx.to = receiver
    tx.data = hex2bytes(bytecode)

    message = sha3(tx.SerializeToString())

    sign_recover = privkey.ecdsa_sign_recoverable(message, raw=True)
    sig = privkey.ecdsa_recoverable_serialize(sign_recover)

    signature = binascii.hexlify(
        sig[0]) + binascii.hexlify(bytes(bytearray([sig[1]])))

    unverify_tx = UnverifiedTransaction()
    unverify_tx.transaction.CopyFrom(tx)
    unverify_tx.signature = hex2bytes(signature)
    unverify_tx.crypto = Crypto.Value('SECP')

    signed_transaction = SignedTransaction()
    signed_transaction.transaction_with_sig.CopyFrom(unverify_tx)
    signed_transaction.tx_hash = sha3(unverify_tx.SerializeToString())
    pub = recover_pub(hex2bytes(privatekey))
    signed_transaction.signer = pub
    # print(binascii.hexlify(pub))

    return binascii.hexlify(signed_transaction.SerializeToString())


def parse_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument("--bytecode", help="Compiled contract bytecode.")
    parser.add_argument(
        "--privkey", help="private key genearted by secp256k1 alogrithm.")
    parser.add_argument("--receiver", help="transaction to")
    parser.add_argument('--newcrypto', dest='newcrypto',
                        action='store_true', help="Use ed25519 and blake2b.")
    parser.add_argument('--no-newcrypto', dest='newcrypto',
                        action='store_false', help="Use ecdsa and sha3.")
    parser.set_defaults(newcrypto=False)

    opts = parser.parse_args()
    return opts


def _params_or_default():
    opts = parse_arguments()
    bytecode = opts.bytecode
    privkey = opts.privkey
    receiver = opts.receiver

    if bytecode is None:
        bytecode = bin_code()

    if privkey is None:
        privkey = private_key()

    return (bytecode, privkey, receiver)


def _blake2b(seed):
    hashed = pysodium.crypto_generichash_blake2b_salt_personal(seed, key = "CryptapeCryptape")
    return hashed

def main():
    blake2b_ed25519 = parse_arguments().newcrypto
    print(blake2b_ed25519)
    bytecode, privkey, receiver = _params_or_default()
    data = generate_deploy_data(
        bytecode, privkey, remove_hex_0x(receiver), blake2b_ed25519)
    print("deploy code保存到../output/transaction/deploycode")
    print(data)
    save_deploy(data)


if __name__ == '__main__':
    main()
