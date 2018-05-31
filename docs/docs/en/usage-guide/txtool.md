# TXTOOL

## 进入 docker 镜像

```
./env.sh
```

## 安装依赖

```
cd scripts/txtool
```

```shell
pip3 install -r requirements.txt
bash requirements_sudo.sh
```

## 用法

### check

检查 CITA 是否正常启动

```shell
python3 check.py
```

### peer_count

获取链接的节点数

```shell
python3 peer_count.py
```

### block_number

获取块的高度

```shell
python3 block_number.py
```

### generate_account

```
python3 generate_account -h
```

```
usage: generate_account.py [-h] [--newcrypto] [--no-newcrypto]

optional arguments:
  -h, --help      show this help message and exit
  --newcrypto
  --no-newcrypto
```

* 使用 secp256k1 签名算法和 sha3 hash算法

```shell
python3 generate_account.py
```

* 使用 ed25519 签名算法和blake2b hash算法

```shell
python3 generate_account.py --newcrypto
```

### compile

编译合约

```shell
python3 compile.py -h
```

```
usage: compile.py [-h] [-s SOURCE] [-f FILE] [-p PROCEDURE]

optional arguments:
  -h, --help            show this help message and exit
  -s SOURCE, --source SOURCE
                        Solidity source code
  -f FILE, --file FILE  solidity file name with full path. Like
                        ~/examplefolder/test.solc
  -p PROCEDURE, --procedure PROCEDURE
                        Solidity function name.
```

```shell
// 传入文件的绝对路径
python3 compile.py -f /home/jerry/rustproj/cita/admintool/txtool/txtool/tests/test.sol

// 或者传入源码
python3 compile.py -s "pragma solidity ^0.4.0;

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

// 合约编译的结果保存在output/compiled目录
```

获取编译合约的函数地址

```shell
$ python3 compile.py -p "get()"
0x6d4ce63c
```

### make_tx

构造交易

```
python3 make_tx -h
```

```
usage: make_tx.py [-h] [--code CODE] [--privkey PRIVKEY] [--to TO]
                  [--newcrypto] [--no-newcrypto] [--version VERSION]
                  [--chain_id CHAIN_ID]

optional arguments:
  -h, --help           show this help message and exit
  --code CODE          Compiled contract bytecode.
  --privkey PRIVKEY    private key genearted by secp256k1 alogrithm.
  --to TO              transaction to
  --newcrypto          Use ed25519 and blake2b.
  --no-newcrypto       Use ecdsa and sha3.
  --version VERSION    Tansaction version.
  --chain_id CHAIN_ID
```

使用 secp256k1 签名算法和 sha3-hash

```shell
python3 make_tx.py

python3 make_tx.py --code `contract bytecode` --privkey `privatekey` --to `transaction to`
```

使用 ed25519 签名算法和 blake2b-hash 算法

```shell
python3 make_tx.py --newcrypto

python3 make_tx.py --code `contract bytecode` --privkey `privatekey` --to `transaction to` --newcrypto
```

### send_tx

发送交易

```
python3 send_tx.py -h
```

```
usage: send_tx.py [-h] [--codes CODES [CODES ...]]

optional arguments:
  -h, --help            show this help message and exit
  --codes CODES [CODES ...]
                        send transaction params.
```

交易相关的信息保存在 output/transaction 目录

```shell
python3 send_tx.py

python3 send_tx.py `deploycode`

python3 send_tx.py --codes `deploycode1` `deploycode2` `deploycode3` ...
```

### get_tx

获取交易

交易的 hash 使用 output/transaction/hash 文件中的值

```shell
python3 get_tx.py

python3 get_tx.py --tx `transaction_hash`
```

### block_by_hash

```shell
python3 block_by_hash.py hash --detail
python3 block_by_hash.py hash --no-detail
```

### block_by_number

```shell
python3 block_by_number.py number --detail
python3 block_by_number.py number --no-detail
```

### get_receipt

```shell
python3 get_receipt.py
python3 get_receipt.py --tx `transaction_hash`
```

### tx_count

```shell
python3 tx_count.py `block_number` -a `address`
```

### get_code

```shell
python3 get_code.py `address` `number`
```

### get_logs

获取日志

```shell
python3 get_logs.py -h
```

```
usage: get_logs.py [-h] [--fromBlock FROMBLOCK] [--toBlock TOBLOCK]

optional arguments:
  -h, --help            show this help message and exit
  --fromBlock FROMBLOCK
  --toBlock TOBLOCK
```

### call

调用合约

```shell
python3 call.py `to` `data`

python3 call.py `to` `data` `block_number` --sender `option sender`

to --- contract address
data --- contract method, params encoded data.
// data 构造参考 contract ABI
```
