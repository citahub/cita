# 权限管理

CITA实现了对账户的权限管理，并支持基于角色的权限管理。

CITA通过智能合约的方式来对权限进行管理。

## 账户概述

* 账户(account)： 链上唯一的标识，权限管理的主体对象。
    - 外部账户： 拥有公私钥对，可发送交易的用户。
    - 合约账户： 拥有相关的代码(code)及存储(storage)。

目前权限管理针对外部账户进行细粒度管理。CITA 默认集成了 superAdmin 账户，拥有权限管理涉及到的所有权限。在 CITA 启动前可以对 superAdmin 进行配置。
在权限系统开启时，由用户生成的外部账户，在 CITA 系统中没有任何权限，需要 superAdmin 对其进行授权。

权限管理默认未开启，配置相关信息查看[系统合约](./chain/admintool)

## 权限管理概述

权限(permission)在此系统中的定义为多个资源(resource)的集合，其中资源(resource)为一个合约地址及一个函数签名。

### 系统默认权限类型

用户可自定义权限，其中系统内置了几种权限(禁止对其进行删除操作)，如下所示：

* `sendTx`: 表示发交易的权限
* `createContract`: 表示创建合约的权限
* `newPermission`: 表示创建一个新的权限的权限
* `deletePermission`: 表示删除一个权限的权限
* `updatePermission`: 表示更新一个权限的权限
* `setAuth`: 表示对账号进行授权的权限
* `cancelAuth`: 表示对帐号取消授权的权限
* `newRole`: 表示创建一个新的角色的权限
* `deleteRole`: 表示删除一个角色的权限
* `updateRole`: 表示更新一个角色的权限
* `setRole`: 表示对账号授予角色的权限
* `cancelRole`: 表示对帐号取消授予的角色的权限
* `newGroup`: 表示创建一个新的组的权限
* `deleteGroup`: 表示删除一个组的权限
* `updateGroup`: 表示更新一个组的权限

系统内置了 superAdmin 的帐号，其拥有以上所有权限，可对其进行正常的权限管理。默认配置情况下其他普通账户也拥有以上权限，建议在初始化 CITA 系统前对权限管理进行配置。

### 权限管理合约接口

#### 操作类接口

<table>
  <tr>
    <th>名称</th>
    <th>需要权限</th>
    <th>入参</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      newPermission(name, conts, funcs) <br/>
      <strong>创建新权限</strong>
    </td>
    <td>newPermission</td>
    <td>
      name: 权限的名称
      <br/>
      conts: 权限包含的合约地址的集合
      <br/>
      funcs: 权限包含的的函数签名的集合
    </td>
    <td>新权限的地址 (Address)</td>
    <td>成功后即生成了一个新的权限类型</td>
  </tr>
  <tr>
    <td>
      deletePermission(permission) <br/>
      <strong>删除权限</strong>
    </td>
    <td>deletePermission</td>
    <td>permission: 权限地址 </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后权限不可用</td>
  </tr>
  <tr>
    <td>
      updatePermissionName(permission, name) <br/>
      <strong>更新权限名称</strong>
    </td>
    <td>updatePermission</td>
    <td>
      permission: 权限地址
      <br/>
      name: 新的权限名称
    </td>
    <td>操作是否成功 (bool)</td>
    <td>更新权限的名称，新名称可任意指定，权限之间的名称可重复</td>
  </tr>
  <tr>
    <td>
      addResources(permission, conts. funcs) <br/>
      <strong>添加资源</strong>
    </td>
    <td>updatePermission</td>
    <td>
      permission: 操作的权限对象
      <br/>
      conts: 添加的合约地址的集合
      <br/>
      funcs: 添加的函数签名的集合
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后为指定的权限添加新的资源</td>
  </tr>
  <tr>
    <td>
      deleteResources(permission, conts. funcs) <br/>
      <strong>删除资源</strong>
    </td>
    <td>updatePermission</td>
    <td>
      permission: 操作的权限对象
      <br/>
      conts: 删除的合约地址的集合
      <br/>
      funcs: 删除的函数签名的集合
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后删除指定权限的指定资源</td>
  </tr>
  <tr>
    <td>
      setAuthorization(account, permission) <br/>
      <strong>授权</strong>
    </td>
    <td>setAuth</td>
    <td>
      account: 授权的帐号对象
      <br/>
      permission: 授权的权限对象
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后帐号拥有所授予的权限</td>
  </tr>
  <tr>
    <td>
      setAuthorizations(account, permissions) <br/>
      <strong>多次授权</strong>
    </td>
    <td>setAuth</td>
    <td>
      account: 授权的帐号对象
      <br/>
      permissions: 授权的权限对象的集合
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后帐号拥有所授予的权限集合</td>
  </tr>
  <tr>
    <td>
      cancelAuthorization(account, permission) <br/>
      <strong>取消授权</strong>
    </td>
    <td>cancelAuth</td>
    <td>
      account: 取消授权的帐号对象
      <br/>
      permissions: 取消授权的权限对象
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后帐号不再拥有此权限</td>
  </tr>
  <tr>
    <td>
      cancelAuthorizations(account, permissions) <br/>
      <strong>多次取消授权</strong>
    </td>
    <td>cancelAuth</td>
    <td>
      account: 取消授权的帐号对象
      <br/>
      permissions: 取消授权的权限对象集合
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后帐号不再拥有此权限集合</td>
  </tr>
  <tr>
    <td>
      clearAuthorization(account) <br/>
      <strong>取消账户的所有授权</strong>
    </td>
    <td>cancelAuth</td>
    <td>
      account: 取消授权的帐号对象
    </td>
    <td>操作是否成功 (bool)</td>
    <td>成功后帐号不再拥有任何权限</td>
  </tr>
</table>

#### 查询类接口

查询类接口的调用不需要权限。

<table>
  <tr>
    <th>名称</th>
    <th>入参</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      queryAllAccounts()<br/>
      <strong>查询所有帐号</strong>
    </td>
    <td>None</td>
    <td>所有拥有权限的账号集合</td>
    <td>查询到的账号为权限管理记录的所有帐号</td>
  </tr>
  <tr>
    <td>
      queryPermissions(account) <br/>
      <strong>查询帐号权限</strong>
    </td>
    <td>
      account: 查询的帐号
    </td>
    <td>帐号拥有的权限集合</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryAccounts(permission) <br/>
      <strong>查询拥有权限的账号</strong>
    </td>
    <td>permission: 权限地址</td>
    <td>拥有此权限的所有帐号集合</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      checkPermission(account, permission) <br/>
      <strong>检查权限</strong>
    </td>
    <td>
      account: 鉴权的帐号对象
      <br/>
      permission: 权限地址
    </td>
    <td>判断此帐号是否拥有此权限</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      checkResource(account, cont, func) <br/>
      <strong>检查资源</strong>
    </td>
    <td>
      account: 鉴权的帐号对象
      <br/>
      cont: 合约地址
      <br/>
      func: 函数签名
    </td>
    <td>判断此帐号是否拥有此资源</td>
    <td>其中合约地址及函数签名组成了一个资源</td>
  </tr>
  <tr>
    <td>
      inPermission(permission, cont, func) <br/>
      <strong>检查权限</strong>
    </td>
    <td>
      permission: 判断的权限对象
      <br/>
      cont: 合约地址
      <br/>
      func: 函数签名
    </td>
    <td>判断此权限是否拥有合约及函数</td>
    <td>其中合约地址及函数签名组成了一个资源</td>
  </tr>
  <tr>
    <td>
      queryInfo()<br/>
      <strong>查询权限信息</strong>
    </td>
    <td>None</td>
    <td>权限的所有信息</td>
    <td>信息包括名称及包含的资源集合</td>
  </tr>
  <tr>
    <td>
      queryName()<br/>
      <strong>查询权限名称</strong>
    </td>
    <td>None</td>
    <td>权限的名称</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      queryResource()<br/>
      <strong>查询权限资源</strong>
    </td>
    <td>None</td>
    <td>权限的资源</td>
    <td>None</td>
  </tr>
</table>


## 权限管理操作实例

### 修改系统配置

* 演示中 superAdmin 密钥对如下：
    - 公钥： `0x9dcd6b234e2772c5451fd4ccf7582f4283140697`
    - 私钥： `993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9`
* 通过以下命令生成各节点。

	``` shell
	$ ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" \
	                                         --super_admin "0x9dcd6b234e2772c5451fd4ccf7582f4283140697" \
	                                         --contract_arguments SysConfig.checkPermission=true
	```

用户生成普通账户，由super_admin账户对其进行授权，实现权限管理。使用super_admin的私钥调用的接口由管理员执行，使用用户john的私钥由用户john执行。

### 生成普通账户

用户可以自己生成公私钥对。也可以使用CITA提供的工具`create_key_addr`生成。例如用户john通过工具生成如下密钥。

根据输入的字符串"john"使用`./bin/create_key_addr /tmp/privKey "john"`命令生成账户地址:

* `/tmp/privKey`文件内容为工具生成的私钥
* `john`为当前目录下生成以之命名的文件记录账户地址。

```shell
$./bin/create_key_addr /tmp/privKey "john"
$cat john
0x6212dd3506a68d6ec231177c6cb9c46dcfd43190
$cat /tmp/privKey
a71f68fd5f0a64c0a66737357ec6e491c5bab8e001f8d7116252c22a9a4f03b4
```

### 部署合约

拥有私钥用户john需要部署如下合约，默认时用户john没有部署权限，需要由superAdmin授权。

合约内容：

```shell
pragma solidity ^0.4.16;


contract Advance {
    uint  count = 0;

    function add() {
        count += 1;
    }

    function get() constant returns(uint) {
        return count;
    }

    function reset() {
        count = 0;
    }
}
```

#### 获得合约的字节码

用户john执行如下操作：

```shell
$solc /home/king/work/doc/Advance.sol --bin
Binary:
606060405260008055341561001357600080fd5b60f2806100216000396000f3006060604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f1460585780636d4ce63c14606a578063d826f88f146090575b600080fd5b3415606257600080fd5b606860a2565b005b3415607457600080fd5b607a60b4565b6040518082815260200191505060405180910390f35b3415609a57600080fd5b60a060bd565b005b60016000808282540192505081905550565b60008054905090565b600080819055505600a165627a7a72305820906dc3fa7444ee6bea2e59c94fe33064e84166909760c82401f65dfecbd307d50029
```

得到合约字节码后，就可以将其部署到CITA链上了，部署的方法已经用python脚本封装，只需要传入私钥和字节码即可。python脚本存放的位置为`scripts/txtool/txtool`，具体安装和使用方法可以参考目录下的`README.md`文件。

```shell
######用户john部署合约
$python3 make_tx.py --privkey "a71f68fd5f0a64c0a66737357ec6e491c5bab8e001f8d7116252c22a9a4f03b4" --code "606060405260008055341561001357600080fd5b60f2806100216000396000f3006060604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f1460585780636d4ce63c14606a578063d826f88f146090575b600080fd5b3415606257600080fd5b606860a2565b005b3415607457600080fd5b607a60b4565b6040518082815260200191505060405180910390f35b3415609a57600080fd5b60a060bd565b005b60016000808282540192505081905550565b60008054905090565b600080819055505600a165627a7a72305820906dc3fa7444ee6bea2e59c94fe33064e84166909760c82401f65dfecbd307d50029"
{
    "jsonrpc":"2.0",
    "id":2,
    "result":"0x0"
}
$python3 send_tx.py
{
    "jsonrpc":"2.0",
    "id":1,
    "result":
    {
        "hash":"0x4702c79ce76e861c740601d559c1623365fcb5a281d9710ea68710080a93d571",
        "status":"Ok"
    }
}
```

其中`--privkey`为用户john的账号私钥，`--code`为`Advance.sol`的字节码。
status为OK，默认配置时表示合约已经发送成功。但此时我们已修改了默认配置，打开权限管理，首先需要获得发送交易及创建合约的权限。

这种情况获得的回执并没有合约地址(`contractAddress:null`)、并且`errorMessage:"No transaction permission."`，表示此账户还未获得发送交易的权限。回执信息如下：

```shell
$python3 get_receipt.py
{
    "contractAddress": null,
    "cumulativeGasUsed": "0x0",
    "logs": [],
    "blockHash": "0xd527e1c6a43e8096d52d83a2dfe38df7bb33e8abc29108c178c0ac713d0e82e3",
    "transactionHash": "0x4702c79ce76e861c740601d559c1623365fcb5a281d9710ea68710080a93d571",
    "root": null,
    "errorMessage": "No transaction permission.",
    "blockNumber": "0x55f9",
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "transactionIndex": "0x0",
    "gasUsed": "0x0"
}
```

#### 部署合约

由于用户john的账户没有部署权限，需要通过superAdmin对其授sendTx发送交易及createContract创建合约权限。

* 授予发送交易权限

	```shell
	######调用系统合约setAuthorization对用户john授予sendTx权限，createContract对应的ID为0x1，ID作为setAuthorization的第二个参数
	$python3 make_tx.py --privkey "993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9" --to "ffffffffffffffffffffffffffffffffff020004" --code "0f5aa9f30000000000000000000000006212dd3506a68d6ec231177c6cb9c46dcfd431900000000000000000000000000000000000000000000000000000000000000001"
	$python3 send_tx.py
	$python3 get_receipt.py
	```

其中`--to`为权限管理的系统合约地址，`--code`前`0x0f5aa9f3`为系统合约的setAuthorization接口，其后紧跟需要授权地址和所授权限。`0x1`表示sendTx发送权限。

* 授予创建合约权限

	```shell
	######调用系统合约setAuthorization对用户john授予createContract权限，createContract对应的ID为0x2，ID作为setAuthorization的第二个参数
	$python3 make_tx.py --privkey "993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9" --to "ffffffffffffffffffffffffffffffffff020004" --code "0f5aa9f30000000000000000000000006212dd3506a68d6ec231177c6cb9c46dcfd431900000000000000000000000000000000000000000000000000000000000000002"
	$python3 send_tx.py
	$python3 get_receipt.py
	```

其中`--to`为权限管理的系统合约地址，`--code`前`0x0f5aa9f3`为系统合约的setAuthorization接口，其后紧跟需要授权地址和所授权限。0x2表示createContract创建合约权限。

* 再次部署合约

权限授予成功后用户john重做第一步，成功获得合约地址，表示合约部署成功。成功回执信息如下：

```shell
######再次部署合约
$python3 make_tx.py --privkey "a71f68fd5f0a64c0a66737357ec6e491c5bab8e001f8d7116252c22a9a4f03b4" --code "606060405260008055341561001357600080fd5b60f2806100216000396000f3006060604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680634f2be91f1460585780636d4ce63c14606a578063d826f88f146090575b600080fd5b3415606257600080fd5b606860a2565b005b3415607457600080fd5b607a60b4565b6040518082815260200191505060405180910390f35b3415609a57600080fd5b60a060bd565b005b60016000808282540192505081905550565b60008054905090565b600080819055505600a165627a7a72305820906dc3fa7444ee6bea2e59c94fe33064e84166909760c82401f65dfecbd307d50029"
$python3 send_tx.py
#python3 get_receipt.py
{
    "contractAddress": "0x47113fea5720d201b31ecf82a7da5ea3ed150255",
    "cumulativeGasUsed": "0xd160",
    "logs": [],
    "blockHash": "0x84c43701a3eefdde2a3f3ff8ab823faea8fa2fd59cd691c0f7a704791947e86f",
    "transactionHash": "0x3593ac7424d1f98f77a705e6c6b1a2e3db99438df0f771ad82ff4a7b00d9865b",
    "root": null,
    "errorMessage": null,
    "blockNumber": "0x564c",
    "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "transactionIndex": "0x0",
    "gasUsed": "0xd160"
}
```

回执`errorMessage`返回值为null，此时Advance合约由用户john成功部署到CITA链上。但是此时的合约还没有用户能够使用(superAdmin除外)，用户john需要调用Advance合约还需要superAdmin用户给予授权。

### 生成新的权限

通过superAdmin授予用户john的账户newPermission权限，回执中`errorMessage`返回值为`null`，表示成功生成新权限。

```shell
######调用系统合约的newPermission生成新的权限
$python3 make_tx.py --privkey "993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9" --to "ffffffffffffffffffffffffffffffffff020004" --code "fc4a089c416476616e63655f66756e6374696f6e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000100000000000000000000000047113fea5720d201b31ecf82a7da5ea3ed15025500000000000000000000000000000000000000000000000000000000000000014f2be91f00000000000000000000000000000000000000000000000000000000"
$python3 send_tx.py
$python3 get_receipt.py
{
  "contractAddress": null,
  "cumulativeGasUsed": "0xc5a22",
  "logs": [
    {
      "blockHash": "0xf994c2748ec0d09b1233fc25b194872acec15d3fe45dcfd5f632d12bfb3503b1",
      "transactionHash": "0xfbfb3cdb1d4fa1ca203cc4feaa869e1a0447d035dfac6e48a3e1e71c83f3ca58",
      "transactionIndex": "0x0",
      "topics": [
        "0xb533e8b79dc7485ba7e4435e3395df911c1a3c767225941003d88a7812d216f7"
      ],
      "blockNumber": "0x56af",
      "address": "0x3682affa243cb9536cb6989307c54e388198e709",
      "transactionLogIndex": "0x0",
      "logIndex": "0x0",
      "data": "0x00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000100000000000000000000000047113fea5720d201b31ecf82a7da5ea3ed15025500000000000000000000000000000000000000000000000000000000000000014f2be91f00000000000000000000000000000000000000000000000000000000"
    },
    {
      "blockHash": "0xf994c2748ec0d09b1233fc25b194872acec15d3fe45dcfd5f632d12bfb3503b1",
      "transactionHash": "0xfbfb3cdb1d4fa1ca203cc4feaa869e1a0447d035dfac6e48a3e1e71c83f3ca58",
      "transactionIndex": "0x0",
      "topics": [
        "0x792f7322d94960c6e90863b5aef39075ca54620cfa13a822081d733f79c48f91",
        "0x0000000000000000000000003682affa243cb9536cb6989307c54e388198e709",
        "0x416476616e63655f66756e6374696f6e00000000000000000000000000000000"
      ],
      "blockNumber": "0x56af",
      "address": "0xffffffffffffffffffffffffffffffffff020005",
      "transactionLogIndex": "0x1",
      "logIndex": "0x1",
      "data": "0x00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000100000000000000000000000047113fea5720d201b31ecf82a7da5ea3ed15025500000000000000000000000000000000000000000000000000000000000000014f2be91f00000000000000000000000000000000000000000000000000000000"
    }
  ],
  "blockHash": "0xf994c2748ec0d09b1233fc25b194872acec15d3fe45dcfd5f632d12bfb3503b1",
  "transactionHash": "0xfbfb3cdb1d4fa1ca203cc4feaa869e1a0447d035dfac6e48a3e1e71c83f3ca58",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x56af",
  "logsBloom": "0x00000000000000000000000100000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000200000000000000000000080000000100000000000020000000000000000000000000800000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000800000000000000000000000000000000000000000100000000000010000004000000000000000000000000000000000000002000001000000100000000100000",
  "transactionIndex": "0x0",
  "gasUsed": "0xc5a22"
}
```

其中

* `--privkey` 为superAdmin的私钥
* `--to` 为权限管理的合约地址
* `--code` 前`0xfc4a089c`为newPermission接口hash值，后面为接口输入的参数
  ("Advance_function",["0x47113fea5720d201b31ecf82a7da5ea3ed150255"],["0x4f2be91f"])，
  `0x47113fea5720d201b31ecf82a7da5ea3ed150255`为部署合约时生成的合约地址，`0x4f2be91f`为合约的add()方法。
  执行成功从get_receipt.py回执logsi[0].address获得新的权限地址`0x3682affa243cb9536cb6989307c54e388198e709`。目前newPermission方法新生成的权限地址记录在`logs[0].address`中。

### 使用新权限

给用户john授予新的权限使其可以调用Advance.sol合约中的add方法。

```shell
######调用系统合约setAuthorization对用户john授予newPermission新生成的权限
$python3 make_tx.py --privkey "993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9" --to "ffffffffffffffffffffffffffffffffff020004" --code "0f5aa9f30000000000000000000000006212dd3506a68d6ec231177c6cb9c46dcfd431900000000000000000000000003682affa243cb9536cb6989307c54e388198e709"
$python3 send_tx.py
$python3 get_receipt.py
```

用户john获得权限后就能使用Advance.add方法，`errorMessage:null`表示成功调用。

```shell
#####用户john调用Advance合约的add方法
$python3 make_tx.py --privkey "a71f68fd5f0a64c0a66737357ec6e491c5bab8e001f8d7116252c22a9a4f03b4" --to "47113fea5720d201b31ecf82a7da5ea3ed150255" --code "4f2be91f"
$python3 send_tx.py
$python3 get_receipt.py
{
  "contractAddress": null,
  "cumulativeGasUsed": "0x4f57",
  "logs": [],
  "blockHash": "0xdfab49202b8cd8c85fafab92b0f4e0002162ecfa6a4414c1e5b78ea8b7522796",
  "transactionHash": "0x7224604ecc51c1f4801ad73123b08001625d1e35c3511a9eb16cced4fc15b58e",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x5848",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0x4f57"
}
```

用户john通过合约的get方法可以获得add方法的结果。

```shell
$curl -X POST --data '{"jsonrpc":"2.0","method":"call","params":[{"to":"0x47113fea5720d201b31ecf82a7da5ea3ed150255","data":"0x6d4ce63c"}, "latest"],"id":2}' 127.0.0.1:1337|jq
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
}
```

其中`to`参数为john之前部署成功后的合约地址、`data`为合约的get方法。result成功返回调用add方法的结果。

致此用户john获得sendTx、createContract以及0x3682affa243cb9536cb6989307c54e388198e709新权限。操作类接口的其它接口如添加删除资源等参照以上调用方式使用。

### 查询账户权限

查询账户权限非发送交易方式，调用合约内容直接调用jsonrpc中的`call`接口。

```shell
######调用系统合约queryPermissions方法查询用户john权限
$curl -X POST --data '{"jsonrpc":"2.0","method":"call","params":[{"to":"0xffffffffffffffffffffffffffffffffff020006","data":"0x945a25550000000000000000000000006212dd3506a68d6ec231177c6cb9c46dcfd43190"}, "latest"],"id":2}' 127.0.0.1:1337|jq
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000020000000000000000000000003682affa243cb9536cb6989307c54e388198e7090000000000000000000000000000000000000000000000000000000000000001"
}
```

其中params中:

* `to`参数，为queryPermissions所在系统合约地址；
* `data`为调用`call`的输入参数，由queryPermissions的hash和queryPermissions参数组成。

result返回给用户john的所有权限，包括sendTx、createContract以及0x3682affa243cb9536cb6989307c54e388198e709新权限。

```shell
######调用系统合约queryResource方法查询新权限0x3682affa243cb9536cb6989307c54e388198e709内容
$curl -X POST --data '{"jsonrpc":"2.0","method":"call","params":[{"to":"0x3682affa243cb9536cb6989307c54e388198e709","data":"0x53f4a519"}, "latest"],"id":2}' 127.0.0.1:1337|jq
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": "0x00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000100000000000000000000000047113fea5720d201b31ecf82a7da5ea3ed15025500000000000000000000000000000000000000000000000000000000000000014f2be91f00000000000000000000000000000000000000000000000000000000"
}
```

result返回新权限包括john部署的合约地址及相应add方法。

查询类接口的其它接口参照以上调用方式使用。


## 角色管理概述

在权限之上封装了一层更贴近于现实生活中的角色类型，角色包含多种权限。可对用户赋予角色，则用户拥有角色内的所有权限。

* 角色的增删改等相关操作独立于权限管理。操作需要权限管理赋予相应权限，不会造成权限管理的变动。
* 关于角色的授权操作： 授予角色时会调用权限管理的授权接口，所以会造成权限管理的变动。 ***建议角色的授权与权限的授权二者选其一，应该尽量避免同时使用***
* 关于角色的鉴权： 鉴权是在底层操作，底层没有角色的概念，鉴权与权限管理统一。

用户可自定义角色。

### 角色管理合约接口

合约接口调用方式与权限管理方式一致。

#### 操作类接口

<table>
  <tr>
    <th>名称</th>
    <th>需要权限</th>
    <th>入参</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      newRole(name, permissions)<br/>
      <strong>新建角色</strong>
    </td>
    <td>newRole</td>
    <td>
      name: 角色名称
      <br/>
      permissions: 权限集合
    </td>
    <td>新建的角色的地址</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      deleteRole(role) <br/>
      <strong>删除角色</strong>
    </td>
    <td>deleteRole</td>
    <td>
      role: 角色地址
    </td>
    <td>删除是否成功 (bool)</td>
    <td>如果角色已经被授予帐号则需要cancelAuthorization，否则则不需要权限</td>
  </tr>
  <tr>
    <td>
      updateRoleName(role, name) <br/>
      <strong>更新角色名称</strong>
    </td>
    <td>updateRole</td>
    <td>
      role: 更新的角色
      <br/>
      name: 更新的新的角色的名称
    </td>
    <td>更新是否成功 (bool)</td>
    <td>None</td>
  </tr>
  <tr>
    <td>
      addPermissions(role, permissions) <br/>
      <strong>添加角色权限</strong>
    </td>
    <td>updateRole</td>
    <td>
      role: 角色
      <br/>
      permissions: 添加的权限集合
    </td>
    <td>添加是否成功 (bool)</td>
    <td>如果角色已经被授予帐号则需要调用setAuthorization</td>
  </tr>
  <tr>
    <td>
      deletePermissions(role, permissions) <br/>
      <strong>删除角色权限</strong>
    </td>
    <td>updateRole</td>
    <td>
      role: 角色
      <br/>
      permissions: 删除的权限集合
    </td>
    <td>删除是否成功 (bool)</td>
    <td>如果角色已经被授予帐号则需要调用cancelAuthorization</td>
  </tr>
  <tr>
    <td>
      setRole(account, role) <br/>
      <strong>设置角色</strong>
    </td>
    <td>setRole</td>
    <td>
      account: 设置角色的帐号对象
      <br/>
      role: 设置的角色
    </td>
    <td>设置是否成功 (bool)</td>
    <td>调用权限管理，把role内的所有permission依次授予account</td>
  </tr>
  <tr>
    <td>
      cancelRole(account, role) <br/>
      <strong>取消设置角色</strong>
    </td>
    <td>cancelRole</td>
    <td>
      account: 取消设置角色的帐号对象
      <br/>
      role: 取消设置的角色
    </td>
    <td>取消设置是否成功 (bool)</td>
    <td>调用权限管理，把role内的所有permission依次取消授予account</td>
  </tr>
  <tr>
    <td>
      clearRole(account) <br/>
      <strong>取消帐号所有的角色</strong>
    </td>
    <td>cancelRole</td>
    <td>
      account: 取消设置角色的帐号对象
    </td>
    <td>取消设置是否成功 (bool)</td>
    <td>调用权限管理，把account所有的role内的所有permission依次取消授予account</td>
  </tr>
</table>

#### 查询类接口

<table>
  <tr>
    <th>名称</th>
    <th>入参</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>
      queryPermissions(role)<br/>
      <strong>查询角色权限</strong>
    </td>
    <td>
      role: 查询的角色
    </td>
    <td>角色的权限集合</td>
    <td>调用role_management合约</td>
  </tr>
  <tr>
    <td>
      queryRoles(account)<br/>
      <strong>查询帐号所有的角色</strong>
    </td>
    <td>account: 查询的帐号</td>
    <td>所有账号拥有的角色集合</td>
    <td>调用role_management合约</td>
  </tr>
  <tr>
    <td>
      queryAccounts(role) <br/>
      <strong>查询拥有此角色的所有帐号</strong>
    </td>
    <td>
      role: 查询的角色
    </td>
    <td>帐号集合</td>
    <td>调用role_management合约</td>
  </tr>
  <tr>
    <td>
      queryRole() <br/>
      <strong>查询角色信息</strong>
    </td>
    <td>None</td>
    <td>角色的信息集合</td>
    <td>调用相应role合约，信息包括角色名称和权限列表</td>
  </tr>
  <tr>
    <td>
      queryName()<br/>
      <strong>查询角色名称</strong>
    </td>
    <td>None</td>
    <td>角色的名称</td>
    <td>调用相应role合约</td>
  </tr>
  <tr>
    <td>
      queryPermissions()<br/>
      <strong>查询角色权限</strong>
    </td>
    <td>None</td>
    <td>角色的权限集合</td>
    <td>调用相应role合约</td>
  </tr>
</table>
