# 紧急制动

## 简述

超级管理员在极端情况下的维护手段，开启紧急制动模式后，链上只接收超级管理员发送的交易，其他交易全部拒绝。

可能需要这个合约的场景：

- 运营方需要对系统合约进行 `amend` 操作，这个操作是风险性很大的，需要拒绝任何其他人交易带来的意外影响
- 链正常运行期间，进行一些升级，维护等操作，不希望有其他人的干扰

### 合约信息

合约地址： `0xffffffffffffffffffffffffffffffffff02000f`

接口签名如下：

```
======= emergency_brake.sol:EmergencyBrake =======
ac9f0222: setState(bool)
c19d93fb: state()
```

初始默认值为 `false`，超级管理员可以通过发交易的方式修改状态值，当状态为 `true` 时，进入紧急制动模式。

### 操作示例

*首先需要启动一条链，具体方法见快速入门部分*

接下来的测试，用 [cita-cli](https://github.com/cryptape/cita-cli) 命令行模式（与交互式模式的命令是一致的）进行演示。

- 首先查询链当前状态：

```bash
cita-cli scm EmergencyBrake state --url http://127.0.0.1:1337
```

输出：

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000000"
}
```
可以看到，当前链未开启紧急制动模式

- 开启紧急制动模式并确认：

```bash
cita-cli scm EmergencyBrake setState \
    --state true \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

输出：
```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x45d1436927f3fe013c8e35481283317dda23f4c17b8aa4f5b4c42ecb2e81c817",
    "status": "OK"
  }
}
```

```bash
cita-cli scm EmergencyBrake state --url http://127.0.0.1:1337
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```
可以确认，当前状态已经改为紧急制动模式

- 用随机私钥发送转账交易，确认紧急制动功能正常：

```bash
cita-cli key create
```

输出：
```json
{
    "address": "0xdd7342f637100daac32dc42823e111bcfc90943d",
    "private": "0xf2c9b7ebd64c079928e6873f6b2f0551ecedf87d4a1cab30851b8592aa4b2396",
    "public": "0x24ff15c562d4cd61c8d041fa960bd6ee88313ad5eb5359fa0f66cac787b3010c8bb2d508ccf218f0ac58b9c318d7ae90508486ad568bf538562831db2da3faea"
}
```

任意私钥发送交易被拒绝
```bash
cita-cli transfer \
    --address 0x23e2aef1f034f4e2db0ede35bfd92999a4b081d9 \
    --private-key 0xf2c9b7ebd64c079928e6873f6b2f0551ecedf87d4a1cab30851b8592aa4b2396 \
    --value 0x10 \
    --url http://127.0.0.1:1337
```

输出：
```json
{
  "error": {
    "code": -32006,
    "message": "Forbidden"
  },
  "id": 3,
  "jsonrpc": "2.0"
}
```

超管发送交易，正常执行
```bash
$ cita-cli transfer \
      --address 0xdd7342f637100daac32dc42823e111bcfc90943d \
      --private-key 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
      --value 0x10 \
      --url http://127.0.0.1:1337
```

输出：
```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x52c40ad205d15357cbd3ecab07a68d6886ea3d0064f76394272ae26d955ad231",
    "status": "OK"
  }
}
```

- 取消紧急制动模式：

```bash
cita-cli scm EmergencyBrake setState \
    --state false \
    --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
    --url http://127.0.0.1:1337
```

输出：
```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x4118bcf5db08dc1649019dcf8f0b777c03a9ca88fac84eef5a4dc734be0c8253",
    "status": "OK"
  }
}
```

```bash
cita-cli scm EmergencyBrake state --url http://127.0.0.1:1337
```

输出：
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000000"
}
```

- 确认已经取消紧急制动：

```bash
cita-cli transfer \
    --address 0x23e2aef1f034f4e2db0ede35bfd92999a4b081d9 \
    --private-key 0xf2c9b7ebd64c079928e6873f6b2f0551ecedf87d4a1cab30851b8592aa4b2396 \
    --value 0x10 \
    --url http://127.0.0.1:1337
```

输出：
```json
{
  "id": 3,
  "jsonrpc": "2.0",
  "result": {
    "hash": "0x99d7efd6932f43a041512ac88cbfe2997b1a286c288c84d42c22109b4a55c819",
    "status": "OK"
  }
}
```
