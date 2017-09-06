# 帮助使用CITA的新用户了解操作的流程

### 安装需要的依赖

```
$ sudo add-apt-repository ppa:ethereum/ethereum
$ sudo apt-get update
$ sudo apt-get install solc
```

```
$ pip install -r requirements.txt
$ bash requirements_sudo.sh
```

### 检查CITA是否正常启动
```
$ python check.py
```

### net_peerCount

```
$ python peer_count.py
```

### cita_blockNumber

```
$ python block_number.py
```

### 生成账户信息(账户信息保存在output/accounts目录)

使用secp256k1签名算法和sha3 hash

```
$ python generate_account.py
```

使用ed25519签名算法和blake2b hash

```
$ python generate_account.py --newcrypto
```

### 编译合约

```
传入文件的绝对路径
$ python compile.py -f /home/jerry/rustproj/cita/admintool/txtool/txtool/tests/test.sol

或者传入源码
$ python compile.py -s "pragma solidity ^0.4.0;

contract SimpleStorage {
    uint storedData;
    event Init(address, uint);
    event Set(address, uint);

    function SimpleStorage() {
        storedData = 100;
        Init(msg.sender, 100);
    }

    event Stored(uint);

    function set(uint x)  {
        Stored(x);
        storedData = x;
        Set(msg.sender, x);
    }

    function get() constant returns (uint) {
        return storedData;
    }
}"

合约编译的结果保存在output/compiled目录
```

获取编译合约的函数地址
```
$ python compile.py -p "get()"
0x6d4ce63c
```

### 构造交易

使用secp256k1签名算法和sha3 hash
```
$ python make_tx.py

$ python make_tx.py --bytecode `contract bytecode` --privkey `privatekey` --receiver `transaction to`
```
使用ed25519签名算法和blake2b hash

```
$ python make_tx.py --newcrypto

$ python make_tx.py --bytecode `contract bytecode` --privkey `privatekey` --receiver `transaction to` --newcrypto
```


### 发送交易
交易相关的信息保存在output/transaction目录

```
$ python send_tx.py

$ python send_tx.py `deploycode`

$ python send_tx.py --codes `deploycode1` `deploycode2` `deploycode3` ...
```

### 获取交易
交易的hash使用output/transaction/hash文件中的值

```
$ python get_tx.py

$python get_tx.py --tx `transaction_hash`
```

### cita_getBlockByHash

```
$ python block_by_hash.py hash --detail
$ python block_by_hash.py hash --no-detail
```

### cita_getBlockByNumber

```
$ python block_by_number.py number --detail
$ python block_by_number.py number --no-detail
```

### 获取receipt

```
$ python get_receipt.py
$ python get_receipt.py --tx `transaction_hash`
```

### eth_getTransactionCount

```
$ python tx_count.py `block_number` -a `address`
```

### eth_getCode
```
$ python get_code.py `address` `number`
```
### 获取Logs

```
$ python get_logs.py
```

### 调用合约
```
$ python call.py `from` `to` `data` `block_number`
from或to没有的使用空字符串
data构造参考contract ABI
```
