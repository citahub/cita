# 配额价格管理合约接口

<h2 class="hover-list">Price Management</h2>

* [setQuotaPrice](#setQuotaPrice)
* [getQuotaPrice](#getQuotaPrice)

***

### setQuotaPrice

设置 `quota price`，默认为 1。

* Parameters

    `uint` - The setting quota price

* Returns

    `bool` - True if success,other false.

* Example

```shell
$ cita-cli scm PriceManager setQuotaPrice \
              --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
              --price 0x0000000000000000000000000000000000000000000000000000000000000002
```

### getQuotaPrice

查询当前链 quota price。

* Parameters

    None

* Returns

    `uint` - The quota price

* Example

```shell
$ cita-cli scm PriceManager getQuotaPrice
```

output:

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000002"
}
```
