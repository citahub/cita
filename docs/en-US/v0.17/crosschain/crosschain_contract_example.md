# 跨链合约编写及操作示例

## 跨链合约编写

### 跨链合约示例

可以参照[示例合约](https://github.com/cryptape/cita/blob/develop/scripts/contracts/tests/contracts/cross_chain_token.sol)。

这是一个可以跨链转移 token 的 Token 合约。

相比普通的 Token 合约增加了 `send_to_side_chain` 和 `recv_from_side_chain` 两个函数用于跨链转 token 。

`send_to_side_chain` 只是在一条链上扣掉一部分 token 。

等交易执行之后，使用 JsonRPC 接口 `cita_getTransactionProof` 获取交易执行的证明。

将证明发送到另外一个链上的 `recv_from_side_chain`。校验证明之后解析出原始交易的内容。在这个例子里就是转账金额。

然后执行整个交易的后半段，给同样的用户增加同样数量的 token ，完成 token 的跨链转移。

### 跨链合约注意事项

`RECV_FUNC_HASHER` 是 `recv_from_side_chain` 的 function signature 。可以使用如下命令查看：

```shell
solc --hashes cross_chain_token.sol
```

`DATA_SIZE` 是跨链传递的数据的大小。即 `send_to_side_chain` 除去前两个参数（固定必须的参数）之后所有参数的总大小。

`nonce` 是为了防止跨链交易重放攻击增加的，作用同 `CITA` 交易中的 `nonce` 。

跨链交易必须严格按照交易发生的顺序在两条链之间传递，因此 `crosschain_nonce` 设计为自增的计数。

将证明发送到另外一个链上之前，先调用 `get_cross_chain_nonce` 获取当前 `nonce`。

同时有工具可以解析证明，提取证明中的 `nonce` 。只有两者相等才能发送成功，如果不相等，则说明证明已经发送过，可以丢弃；或者前序交易还未发送，还需要等待。

`send_to_side_chain` 中的 `event cross_chain(uint256 from_chain_id, uint256 to_chain_id, address dest_contract, uint256 hasher, uint256 nonce);`为跨链提供必须的信息。

请勿修改，也不要在这个函数中增加其他 `event`。

`recv_from_side_chain` 解析出原始交易的数据为 `bytes` 类型，用户需要参照 `send_to_side_chain` 自行解析成对应的类型。

## 跨链合约操作示例

### 新建、注册和启动侧链

目前，侧链使用系统合约 [ChainManager](https://github.com/cryptape/cita/blob/develop/scripts/contracts/src/system/chain_manager.sol) 进行管理。

* 生成侧链的验证节点的私钥，使用侧链的验证节点地址，在主链上使用系统合约 `ChainManager` 的方法 `newSideChain` 进行新建侧链，得到侧链的 Id 。
* 在主链上使用系统合约 `ChainManager` 的方法 `enableSideChain` 启动指定 Id 的侧链。
* 新建侧链，创世块里的系统合约 `ChainManager` 构造时，使用上一个步骤申请的侧链 Id 、主链的 Id 和主链的验证节点地址作为参数。
* 启动侧链即可。

### 部署跨链合约

在主侧链分别部署跨链合约，分别得到合约地址。

### 发送跨链交易

调用任意一条链的跨链合约的 `send_to_side_chain` 方法，
使用接收链（另一条链）的 Id 、接收链跨链合约的合约地址和转移的 token 数量作为参数，
发送跨链转移 token 交易，并得到交易 hash 。

在操作步骤中不区分主链和侧链。

### 使用 Relayer 工具发送跨链交易到目标链

使用跨链交易的交易 hash 、该交易所在链的 Id，和一个配置文件作为入参调用工具：

```shell
cita-relayer-parser -c SEND_CHAIN_ID -t TX_HASH -f relayer-parser.json
```

其中配置文件 `relayer-parser.json` 目前主要有 2 个参数：

* 工具使用的私钥。
* 所有相关链的 JsonRPC 网络地址，使用 Id 作为索引。

范例如下：

```json
{
    "private_key": "0x1111111111111111111111111111111111111111111111111111111111111111",
    "chains": [
        {
            "id": 1,
            "servers": [
                { "url": "http://127.0.0.1:11337", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11338", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11339", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:11340", "timeout": { "secs": 30, "nanos": 0 } }
            ]
        },
        {
            "id": 2,
            "servers": [
                { "url": "http://127.0.0.1:21337", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21338", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21339", "timeout": { "secs": 30, "nanos": 0 } },
                { "url": "http://127.0.0.1:21340", "timeout": { "secs": 30, "nanos": 0 } }
            ]
        }
    ]
}
```

该工具主要做的任务为：

* 根据入参，去发送链上查询跨链交易的交易证明数据。
* 根据跨链交易的交易证明数据，得到转移 token 的接收链的 Id 。
* 发送证明到接收链上，完成 token 转移。

### 验证跨链是否成功

在发送链和接收链分别使用跨链合约中的查询接口（实例合约中 `get_balance` 方法）查询当前用户的 token 数量。
