# CITA Interfaces

CITA RPC specifications and tests.

## 目的

主要是为了测试API接口的所有类型能正常工作，对于EVM内不同情况的但是RPC接口一致的情况，不属于本测试覆盖范围，部分EVM指令的测试需要EVM内部构造测试用例。

## 分类

- cita_blockNumber

  正确参数：[正确结果[包含交易内容，不包含交易内容]，错误结果]

  错误参数：[个数错误，类型错误]

- cita_sendTransaction 暂时不处理

- cita_getBlockByHash

  正确参数：

  - 参数：[hash, latest, earliest, 创世块]
  - 返回：[有结果[有交易，无交易]，无结果]

  错误参数：...

- cita_getBlockByNumber

  正确参数：...

  错误参数：...

- eth_getTransactionReceipt

  正确参数:

  - 有结果
    - 交易正常处理
    - 交易处理失败：ReceiptError的类型参见ReceiptError，对于权限类别的验证，这里不覆盖。
  - 无结果

  错误参数：

- eth_getLogs

  正确参数：几种组合...

  错误参数：...

- eth_call

  正确参数: [...]

  错误参数: ...

- cita_getTransaction...

- eth_getTransactionCount ...

- eth_getCode ...

- eth_getAbi...

- eth_newFilter ...

- eth_newBlockFilter 暂时不测试

- eth_uninstallFilter 暂时不测试

- eth_getFilterChanges 暂时不测试

- eth_getFilterLogs 暂时不测试

- cita_getTransactionProof 暂时不测试

## 原则

根据以上测试数据构造数据和测试用例
