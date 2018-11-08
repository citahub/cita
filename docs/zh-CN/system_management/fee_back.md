# 出块奖励返回

## 简述
CITA 中存在两种经济模型：`Quota` 和 `Charge` 模型，默认经济模型 `Quota` ，没有余额概念。
在具有余额的 `Charge` 经济模型中，出块奖励默认返还给共识节点，但运营方可以通过设置 `checkFeeBackPlatform` 和 `chainOwner`，将出块奖励返还给自己。

出块奖励 = quotaUsed * quotaPrice, 其中 `quotaPrice` 默认为 1000000，`quotaUsed` 在交易回执中可以获取。

> 0.20 版本之前的默认 `quotaPrice` 是 1

### 操作示例

首先配置链的时候要注意三点：
- 配置经济模型
- 设置奖励返回开关
- 设置运营方地址

```bash
$ ./env.sh ./scripts/create_cita_config.py create \
        --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" \
        --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
        --contract_arguments "SysConfig.checkFeeBackPlatform=true" \
        --contract_arguments "SysConfig.chainOwner=0x36a60d575b0dee0423abb6a57dbc6ca60bf47545" \  
        --contract_arguments "SysConfig.economicalModel=1"
```

*接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 交互模式进行演示*。

查看管理员和运营方地址余额

```bash
$ rpc getBalance --address "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"
```

管理员余额输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0xffffffffffffffffffffffffff"
}
```

```bash
$ rpc getBalance --address "0x36a60d575b0dee0423abb6a57dbc6ca60bf47545"
```

运营方余额输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0"
}
```

让我们来发一笔交易并获取回执，来看看余额的变化吧。

```bash
$ rpc sendRawTransaction \
    --code "0x606060405260008055341561001357600080fd5b60f2806100216000396000f3006060604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f1460585780636d4ce63c14606a578063d826f88f146090575b600080fd5b3415606257600080fd5b606860a2565b005b3415607457600080fd5b607a60b4565b6040518082815260200191505060405180910390f35b3415609a57600080fd5b60a060bd565b005b60016000808282540192505081905550565b60008054905090565b600080819055505600a165627a7a72305820906dc3fa7444ee6bea2e59c94fe33064e84166909760c82401f65dfecbd307d50029" \
    --private-key "0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6" \
```

```bash
$ rpc getTransactionReceipt --hash "0x39c4cd332892fb5db11c250275b9a130bf3c087ebdf47b6504d65347ec349406"
```

回执输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0x72d1eb886dda61bc5b58f024d5edcf920b15f2e5978ab55f034941b18beb56a8",
    "blockNumber": "0x1b",
    "contractAddress": "0x27ec3678e4d61534ab8a87cf8feb8ac110ddeda5",
    "cumulativeQuotaUsed": "0x1a004",
    "errorMessage": null,
    "logs": [
    ],
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "quotaUsed": "0x1a004",
    "root": null,
    "transactionHash": "0x59df5370e52c4a6af60c869c35222ae7e32b6259e901e94e89be4810dfe7e711",
    "transactionIndex": "0x0"
  }
}

```

再来查一下余额：

```bash
$ rpc getBalance --address "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"
```

管理员余额输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0xffffffffffffffffe7341af6ff"
}
```

```bash
$ rpc getBalance --address "0x36a60d575b0dee0423abb6a57dbc6ca60bf47545"
```

运营方余额输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x18cbe50900"
}
```

可以看到余额发生了变化，运营方的账户余额从 0 变成了 4e3b29200(十进制 106500000000 )， 查看交易回执中的 QuotaUsed 为 106500 (0x1a004)。