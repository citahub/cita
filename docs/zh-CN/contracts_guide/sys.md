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

* 参数

    `String` - 待设置的链的名称

* 返回值

    空

* 示例

```shell
$ scm SysConfig setChainName \
        --chain-name "AAA" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setOperator

设置运营方名称。

* 参数

    `String` - 链运营方名称

* 返回值

    空

* 实例

```shell
 $ scm SysConfig setOperator \
        --operator "CITA" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### setWebsite

设置运营方网站。

* 参数

    `String` - 运营方网站

* 返回值

    空

* 示例

```shell
$ scm SysConfig setWebsite \
        --website "https://github.com/cryptape" \
        --admin-private 0x5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6 \
```

### getPermissionCheck

查询调用合约权限检查是否开启， 默认关闭。

* 参数

    空

* 返回值

    `bool` - 如果开启返回真，反之则反

* 示例

```shell
$ scm SysConfig getPermissionCheck
```

### getSendTxPermissionCheck

查询发送交易权限检查是否开启, 默认关闭。

* 参数

    空

* 返回值

    `bool` - 如果开启返回真，反之则反

* 示例

```shell
$ scm SysConfig getSendTxPermissionCheck
```

### getCreateContractPermissionCheck

查询创建合约权限检查是否开启, 默认关闭。

* 参数

    空

* 返回值

    `bool` - 如果开启返回真，反之则反

* 示例

```shell
$ scm SysConfig getCreateContractPermissionCheck
```

### getQuotaCheck

查询配额检查是否开启， 默认关闭。

* 参数

    空

* 返回值

    `bool` - 如果开启返回真，反之则反

* 示例

```shell
$ scm SysConfig getQuotaCheck
```

### getFeeBackPlatformCheck

查询出块激励返回开关是否开启， 默认关闭。

* 参数

    空

* 返回值

    `bool` - 如果开启返回真，反之则反

* 示例

```shell
$ scm SysConfig getFeeBackPlatformCheck
```

### getChainOwner

查询链的持有者地址。

* 参数

    空

* 返回值

    `address` - 链运营方地址

* 示例

```shell
$ scm SysConfig getChainOwner
```
