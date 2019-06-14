## 相关记录：

* https://cryptape.atlassian.net/browse/CITA-1491
* https://cryptape.atlassian.net/browse/CITA-1537

## 测试目的：

CITA 对共识和交易预执行做了并行处理优化，本脚本以接口测试的方式来验证该优化的正确性。由于优化逻辑对外部不可见，所以没法验证优化的有效性。

验证 cita-executor 在收到 `SignedProposal`/`BlockWithProof` 后是否能正确返回处理结果。

## 测试方法：

* 启动 `cita-executor` 和 `cita-chain`，本脚本以 mock cita-bft 的角色运行

* 消息流图：

   ```
            ExecutedResult
   executor --------------> chain
          ^                /
           \              / RichStatus
   Messages \            /
             \          v
            box-executor
   ```

  `Messages` 是测试脚本 `box_executor` 生成的一些测试块，可能是 `SignedProposal` 或 `BlockWithProof` 结构

* 主要通过 RabbitMQ 发送消息的方式来进行测试，主要测试以下几种不同情况：

  - 发送 `Proposal`，执行完成后，再发送对应的 `BlockWithProof`，检查是否能正常执行。
  - 发送 `Proposal`，执行完成后，再发送不对应的 `BlockWithProof`，在检查是否能正常执行
  - 发送 `Proposal` A，再发送等价的 `Proposal` B（查看 `is_equivalent` 方法），~~检查是否能正常执行，~~再发送 `BlockWithProof`，检查是否能执行。
  - 发送 `Proposal` A，在发送不等价的 `Proposal` B，~~检查是否正常执行，~~在发送 `BlockWithProof`，检查是否能正常执行。
