# 权限合约接口

<h2 class="hover-list">Permission</h2>

* [inPermission](#inPermission)
* [queryInfo](#queryInfo)
* [queryName](#queryName)
* [queryResource](#queryResource)

***

### inPermission

检查资源是否在 permission 中。

* Parameters

    `address` - The contract address of the resource

    `bytes4` -  The function signature of the resource

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm Permission inPermission \
        --contract 0x1e041ec9a18590924d84a1f011eb0749c03fc41a \
        --function-hash 0x60fe47b1 \
        --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee \
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

### queryInfo

* Parameters

    None

* Returns

    `bytes32 permission` - The permission name

    `address[] cont` - The contract address of the resource

    `bytes4[] func` - The function signature of the resource

* Example

```shell
$ scm Permission queryInfo --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000060fe47b2000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000020000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc0000000000000000000000001e041ec9a18590924d84a1f011eb0749c03fc41a000000000000000000000000000000000000000000000000000000000000000260fe47b10000000000000000000000000000000000000000000000000000000060fe47b100000000000000000000000000000000000000000000000000000000"
}
```

### queryName

* Parameters

    None

* Returns

    `bytes32` - The permission name

* Example

```shell
$ scm Permission queryName --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x0000000000000000000000000000000000000000000000000000000060fe47b2"
}
```

### queryResource

* Parameters

    None

* Returns

    `bool` - True, if successfully, otherwise false.

* Example

```shell
$ scm Permission queryResource --permission 0xca645d2b0d2e4c451a2dd546dbd7ab8c29c3dcee
```

```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000020000000000000000000000005839153e0efe76efe0c974b728c4f49ca7ed75cc0000000000000000000000001e041ec9a18590924d84a1f011eb0749c03fc41a000000000000000000000000000000000000000000000000000000000000000260fe47b10000000000000000000000000000000000000000000000000000000060fe47b100000000000000000000000000000000000000000000000000000000"
}

```
