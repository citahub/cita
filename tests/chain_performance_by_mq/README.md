# 功能

通过MQ向Chain模块来发送Block来对Chain持续测试

## 方法

1. 修改节点目录下的chain.json，将权限检查和配额检查改成false
2. 启动Chain, Executor进程，或者启动单独一个节点
3. 在node目录下启动测试进程，这样可以正确的使用.env文件
4. 使用命令进行压力测试: ../../../debug/chain_performance_by_mq --help

## 注意

* 调整交易配额和交易数，防止block超出block quota limit
* 创建合约/更改State时，随着State状态树变大，tps会越来越越慢
* 通过查询block中最后一个交易的receipt来确认交易是否正常执行

## 测试单个合约的调用

```shell
solc Test.sol --bin-runtime --hashes

======= Test.sol:Test =======
Binary: 
606060405260043610603f576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f146044575b600080fd5b3415604e57600080fd5b60546056565b005b6001600054016000819055505600a165627a7a723058207085dc709915ad41cb41e400b83bd863a24143dfb48c10bc007a07b3a7c160cd0029
Function signatures: 
4f2be91f: add()
```

在节点的genesis.json文件中加入：

```json
"0x0000000000000000000000000000000082720029": {
    "nonce": "1",
    "code": "0x606060405260043610603f576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f146044575b600080fd5b3415604e57600080fd5b60546056565b005b6001600054016000819055505600a165627a7a723058207085dc709915ad41cb41e400b83bd863a24143dfb48c10bc007a07b3a7c160cd0029",
    "storage": {}
}
```

使用以下命令进行测试：

```shell
./target/release/chain_performance_by_mq --flag_tx_type 2
```
