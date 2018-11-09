# 配额价格管理

## 简述

和以太坊消耗 gas 类似，在 CITA 的 `Charge` 经济模型中发送交易，部署合约等也需要花费一定的手续费，具体的计算方法是: `手续费 = quotaUsed * quotaPrice` 。
为了更好的满足运营方的需求，我们提供了设置 `quotaPrice` 的接口，拥有权限的管理员可以通过发送交易来设置 `quotaPrice`。

## 操作示例

> 接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 命令行模式进行演示。

默认的 `quotaPrice` 默认为 1000000， 接下来演示管理员如何修改 quotaPrice。

> 0.20 版本之前的默认 `quotaPrice` 是 1

首先查询当前的 `quotaPrice`：
```bash
$ cita-cli scm PriceManager getQuotaPrice
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000000f4240"
}

```

得到 `quotaPrice` 是十六进制的默认值。

修改 `quotaPrice`， 我们把 `quotaPrice` 由 1000000  改为 2000000：

```bash
$ cita-cli scm PriceManager setQuotaPrice \
              --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
              --price 0x00000000000000000000000000000000000000000000000000000000001e8480
```

再次查询， 发现 `quotaPrice` 已更新：

```bash
$ cita-cli scm PriceManager getQuotaPrice
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x00000000000000000000000000000000000000000000000000000000001e8480"
}
```