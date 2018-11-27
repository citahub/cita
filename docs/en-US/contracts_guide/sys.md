# 系统配置合约接口

<h2 class="hover-list">System Config</h2>

* [setChainName](#setChainName)
* [setOperator](#setOperator)
* [setWebsite](#setWebsite)
* [getPermissionCheck](#getPermissionCheck)
* [getSendTxPermissionCheck](#getSendTxPermissionCheck)
* [getCreateContractPermissionCheck](#getCreateContractPermissionCheck)
* [getQuotaCheck](#getQuotaCheck)
* [getFeeBackPlatformCheck](#getFeeBackPlatformCheck)
* [getChainOwner](#getChainOwner)

***

### setChainName

设置链名称。

* Parameters

    `String chainName` - The Chain name

* Returns

    `None`

* Example

```shell
$ scm SysConfig setChainName \
        --chain-name "AAA" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setOperator

设置运营方。

* Parameters

    `String operator` - The Chain operator

* Returns

    `None`

* Example

```shell
 $ scm SysConfig setOperator \
        --operator "CITA" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setWebsite

设置运营方地址。

* Parameters

    `String website` - The Operator website

* Returns

    `None`

* Example

```shell
$ scm SysConfig setWebsite \
        --website "https://github.com/cryptape" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### getPermissionCheck

查询调用合约权限是否开启， 默认关闭。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getPermissionCheck
```

### getSendTxPermissionCheck

查询发送交易权限是否开启, 默认关闭。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getSendTxPermissionCheck
```

### getCreateContractPermissionCheck

查询创建合约权限是否开启, 默认关闭。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getCreateContractPermissionCheck
```

### getQuotaCheck

查询 quota 检查是否开启， 默认关闭。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getQuotaCheck
```

### getFeeBackPlatformCheck

查询出块激励返回开关是否开启， 默认关闭。

* Parameters

    `height(Optional)`

* Returns

    `bool` - True, if permission check, otherwise false.

* Example

```shell
$ scm SysConfig getFeeBackPlatformCheck
```

### getChainOwner

查询链的持有者地址。

* Parameters

    `height(Optional)`

* Returns

    `address` - The chain owner's address

* Example

```shell
$ scm SysConfig getChainOwner
```
