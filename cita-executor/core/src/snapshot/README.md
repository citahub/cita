# 概述

快照含三部分: block chunks, state chunks, 及manifest.

每个chunk是snappy压缩后的hash. 压缩前的chunks大小为`CHUNK_SIZE`,目前默认4MB. 

这里的数据结构都需进行RLP编码。

# 清单(Manifest)

如下结构的rlp list:
```
[
    state_hashes: [hash_1: B_32, hash_2: B_32, ...], // list: state chunks （snappy压缩后的hash）
    block_hashes: [hash_1: B_32, hash_2: B_32, ...], // list: block chunks （snappy压缩后的hash）
    state_root: B_32, // 用于恢复state trie的root
    block_number: P, // 快照对应的区块号，state也与其对应
    block_hash: B_32, // 快照对应的区块hash
]
```

# 区块chunks(Block chunks)

区块chunks包含原始区块数据: blocks及交易receipts. blocks格式"abridged block"(简称 `AB`), receipts格式list: `[receipt_1: P, receipt_2: P, ...]` (简称`RC`).

每个block chunk如下结构的list:
```
[
    number: P, // chunk中的第一个区块号
    hash: B_32, // chunk中的第一个区块hash
    [abridged_1: AB, receipts_1: RC], // 第一个block及receipts的RLP编码（区块连续）
    [abridged_2: AB, receipts_2: RC], // 第二个block及receipts的RLP编码
    [abridged_3: AB, receipts_3: RC], // ... 
    ...
]
```

# 状态chunks(State Chunks)

State chunks存储给定区块的状态. 

每个chunk由list集合构成，每个list含两项：地址的`sha3` hash，及相应的账户结构(ACC).

`[ [hash1: B_32, acc_1: P],  [hash_2: B_32, acc_2: P], ... ]`.

## 账户(Account)

如下格式的RLP编码list:
```
[
    nonce: B_32,
    code: P,
    storage: [[keyhash1: B_32, val1: B_32], [keyhash2: B_32, val2: B_32], ...]
]
```
`storage` 为账户storage的RLP list, 每个元素含两项：`sha3(key)`, 及storage值. 
