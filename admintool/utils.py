from bitcoin import privtopub
import sys
import rlp
from rlp.utils import decode_hex, encode_hex as _encode_hex, ascii_chr, str_to_bytes
import secp256k1
try:
    from Crypto.Hash import keccak
    sha3_256 = lambda x: keccak.new(digest_bits=256, data=x).digest()
except ImportError:
    import sha3 as _sha3
    sha3_256 = lambda x: _sha3.keccak_256(x).digest()

def privtoaddr(x, extended=False):
    if len(x) > 32:
        x = decode_hex(x)
    o = sha3(privtopub(x)[1:])[12:]
    return add_checksum(o) if extended else o

def add_checksum(x):
    if len(x) in (40, 48):
        x = decode_hex(x)
    if len(x) == 24:
        return x
    return x + sha3(x)[:4]

sha3_count = [0]


def sha3(seed):
    sha3_count[0] += 1
    return sha3_256(to_string(seed))

if sys.version_info.major == 2:
    is_string = lambda x: isinstance(x, (str, unicode))

    def to_string(value):
        return str(value)

    encode_hex = _encode_hex

else:
    is_string = lambda x: isinstance(x, bytes)

    def to_string(value):
        if isinstance(value, bytes):
            return value
        if isinstance(value, str):
            return bytes(value, 'utf-8')
        if isinstance(value, int):
            return bytes(str(value), 'utf-8')

    # returns encode_hex behaviour back to pre rlp-0.4.7 behaviour
    # detect if this is necessary (i.e. what rlp version is running)
    if isinstance(_encode_hex(b''), bytes):
        encode_hex = _encode_hex
    else:
        # if using a newer version of rlp, wrap the encode so it
        # returns a byte string
        def encode_hex(b):
            return _encode_hex(b).encode('utf-8')
