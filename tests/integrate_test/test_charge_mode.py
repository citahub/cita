#!/usr/bin/env python3

"""
Test cases of transfer and check balances in charge economical mode.
"""

import functools
import subprocess
import argparse
import time
import binascii

import sha3
from ecdsa import SigningKey, SECP256k1
from jsonrpcclient.http_client import HTTPClient


def send_tx(privkey, to_addr, value=0, quota=30000, code=""):
    """
    Send a transfer transaction to a node

        python3 make_tx.py \
        --value 20000 \
        --quota 100000\
        --code "" \
        --privkey 101c286e965ddf8176dd6c0793e9ad5f3d745105fab744eea6ffdae6a98d0553 \
        --to 0xc94bcce78b4e618c6a259d4eb8e7bf45e145f0d0 \
        --no-newcrypto

        python3 send_tx.py

    """
    kwargs = {
        '--privkey': privkey,
        '--to': to_addr,
        '--code': code,
        '--value': str(value),
        '--quota': str(quota),
    }
    args = functools.reduce(
        lambda lst, kv: lst + list(kv),
        kwargs.items(),
        [],
    )
    print(['python3', 'make_tx.py', *args, '--no-newcrypto'])
    subprocess.call(['python3', 'make_tx.py', *args, '--no-newcrypto'])
    subprocess.call(['python3', 'send_tx.py'])
    with open('../output/transaction/hash') as fobj:
        return fobj.read().strip()


def rpc_request(method, params):
    """ Send a jsonrpc request to default url. """
    client = HTTPClient('http://127.0.0.1:1337')
    return client.request(method, params)


def get_balance(addr):
    """ Get the balance of an address """
    return int(rpc_request('getBalance', [addr, 'pending']), 16)

def get_receipt(tx_hash, retry=8):
    """ Get receipt of a transaction """
    while retry > 0:
        receipt = rpc_request('getTransactionReceipt', [tx_hash])
        if receipt is not None:
            return receipt
        time.sleep(4)
        retry -= 1

def test_transfer(
        sender_privkey, receiver_addr, value,
        sender_is_miner=False):
    """ Transfer and check balances """
    sender_addr = key_address(sender_privkey)
    sender_balance_old = get_balance(sender_addr)
    receiver_balance_old = get_balance(receiver_addr)
    assert sender_balance_old > 0, \
        'Sender balance not enough: address={}'.format(sender_addr)

    tx_hash = send_tx(sender_privkey, receiver_addr, value)
    receipt = get_receipt(tx_hash)
    assert receipt and receipt['errorMessage'] is None, \
        'Send transaction failed: receipt={}'.format(receipt)

    sender_balance_new = get_balance(sender_addr)
    receiver_balance_new = get_balance(receiver_addr)
    assert receiver_balance_old + value == receiver_balance_new, \
        'Invalid receiver balance: {} + {} == {}'.format(
            receiver_balance_old, value, receiver_balance_new)
    if not sender_is_miner:
        assert sender_balance_old - value > sender_balance_new, \
            'Invalid sender balance: {} - {} > {}'.format(
                sender_balance_old, value, sender_balance_new
            )
    print('> [Sender({}).balance]: {}'.format(sender_addr, sender_balance_new))
    print('> [Receiver({}).balance]: {}'.format(receiver_addr, receiver_balance_new))


def get_miner_with_balance(miner_privkeys):
    """ Select a miner with non-zero balance """
    retry = 15
    while retry > 0:
        for privkey in miner_privkeys:
            address = key_address(privkey)
            try:
                if get_balance(address) > 0:
                    return privkey
            except Exception as ex:
                print('Get balance error: {}', ex)
        time.sleep(4)
        retry -= 1
    raise Exception('Get miner with balance timeout(60)')


def key_address(privkey):
    """ Get the address of a privkey """
    hash_obj = sha3.keccak_256()
    pubkey = SigningKey \
        .from_string(binascii.unhexlify(privkey[2:]), curve=SECP256k1) \
        .get_verifying_key() \
        .to_string()
    hash_obj.update(pubkey)
    return '0x{}'.format(hash_obj.hexdigest()[24:])


def main():
    """ Run the tests. """
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--miner-privkeys',
        required=True,
        metavar='PRIVKEY',
        nargs='+',
        help='Private key list of all miners(authorities/nodes)'
    )
    args = parser.parse_args()

    miner_privkey = get_miner_with_balance(args.miner_privkeys)

    alice_privkey = '0xb5d6f7a1bf4493af95afc96f5bf116a3236038fae25e0287ac847623d4e183e6'
    alice_address = key_address(alice_privkey)
    print('[Alice.address]: {}'.format(alice_address))

    bob_privkey = '0x9b9464a30a57702fbfc29cc4afbc676d3dcad1811db3a36ea79b0bde94e10dd9'
    bob_address = key_address(bob_privkey)
    print('[Bob.address]: {}'.format(bob_address))

    # Send 10 * 10000 from miner to alice
    test_transfer(miner_privkey, alice_address, 10 * 10000,
                  sender_is_miner=True)
    assert get_balance(alice_address) == 10 * 10000, \
        'Alice({}) should have 10 * 10000 now'.format(alice_address)
    # Send 50000 from alice to bob
    test_transfer(alice_privkey, bob_address, 30000)
    assert get_balance(bob_address) == 30000, \
        'Bob({}) should have 30000 now'.format(bob_address)

    # Bob send an invalid transaction to chain (Error=NotEnoughCash)
    tx_hash = send_tx(bob_privkey, "", quota=29000, code="")
    # Wait the transaction receipt then check the balance
    get_receipt(tx_hash)
    bob_balance = get_balance(bob_address)
    # Because base_quota_required=21000 (30000 - 21000 = 9000)
    assert bob_balance == 9000, \
        'Bob({}) should have 9000 now (got: {})'.format(bob_address, bob_balance)

    print('>>> Charge Mode test successfully!')


if __name__ == '__main__':
    main()
