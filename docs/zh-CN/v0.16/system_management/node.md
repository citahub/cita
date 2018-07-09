# 节点管理

CITA 中节点分为共识节点和普通节点，交易由共识节点排序并打包成块，再广播至其他节点，共识完成后即被确认为合法区块。普通节点不参与共识，只同步链上所有的原始数据。

公有链没有节点准入机制，意味着任何节点都可以接入链并同步其全部的数据，在满足一定的条件下都可以参加共识。而 CITA 对于共识节点和普通节点都进行了准入管理。对于身份验证失败的节点，即使该节点能够在网络层与其他 CITA 节点连通，这些 CITA 节点也会拒绝与之建立通讯会话，如此可避免信息泄漏。

## 普通节点管理 (白名单)

目前 CITA 对于节点的准入管理采用白名单的方式。每个节点本地保存节点白名单配置文件，其中记录着允许连接的p2p通信和数据同步的节点，包括其公钥、IP 地址、端口、对应的身份信息等。白名单由管理机构生成并分发，运维人员可对其进行维护和管理，可选择连接若干其他节点同时可配置若干普通节点，使其承担数据分析等工作。

### 相关操作

#### 添加普通节点

1. 假设目前的工作目录在 `../cita/target/install/` 下：

    ```bash
    $ pwd
    ../cita/target/install
    $ ls
    backup  bin  node0  node1  node2  node3  scripts
    ```
    backup 中保存了当前节点的公钥地址 `backup/authorities`，以及创世块信息 `backup/genesis.json`，目前地址有四个。

2. 生成新 node：

    ```bash
    $ ./bin/admintool.sh -Q 4 -l "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003,127.0.0.1:4004"
    ************************begin create node config******************************
    ************************end create node config********************************
    WARN: remember then delete all privkey files!!!

    $ ls
    backup  bin  node0  node1  node2  node3  node4  scripts
    ```
    
    - `-Q` 标识要生成的新 node 的 id
    - `-l` 列出已经存在的 node 的 ip 和端口设置，并在最后加上新 node 的 ip 及端口
    - 脚本将自动生成新的 node4，并在原有节点中 `node*/network.toml` 中插入新节点的 ip 及端口配置，同时，`backup/authorities` 中新增一行地址，对应新节点

3. 启动新节点：

    对于原来的节点，如果正在运行，那么 network.toml 修改后，将自动重新加载 p2p 网络配置，并开始尝试寻找新节点。

    新节点只需要按照正常流程启动，就可以连接入网络，并开始同步链上的块数据，**注意，此时的新节点为普通节点，不参与共识选举，即只能同步数据和接收 jsonrpc 请求**。

    ```bash
    $ ./bin/cita setup node4
    $ ./bin/cita start node4
    ```

#### 删除普通节点

到对应节点目录下，找到 `network.toml`，删除对应 `peers` 条目即可。

## 共识节点管理

CITA 作为一个面向企业级应用的区块链框架，需要保证监管方能够获得相关的权限对共识节点进行管理，包括增加、删除共识节点等操作。对于共识服务方面，需要对其提供实时读取共识节点列表的接口，而中心化管理的方式无法保证各个节点的共识节点列表的安全性及一致性。CITA 采用合约的方式来实现共识节点的管理，通过区块链上的合约可以保证共识节点的安全性及一致性。

在 CITA 初始化创世块阶段，需要初始化一个管理员地址，其拥有管理员角色，将其写入到每个节点的创世块文件中，共识节点管理合约拥有的一个固定地址也写入其中。创世块内容在初始化以后不允许被修改。区块链正常启动之后，将合约写入到创世块中。链外的操作人员可以通过调用 RPC 接口来实现对共识节点的管理。

对于共识节点的管理，包括添加、删除及获得共识节点。

* 添加操作分为发起和确认，节点先调用发起请求，申请成为共识节点，由管理员(拥有管理员角色的账号)确认才完成了添加操作;
* 删除操作只可由管理员执行;
* 共识服务可获得共识节点列表。

普通节点安装生成后申请成为共识节点，需要进行以下操作：

* 将账号地址提交给管理员;
* 节点发起一个记录其为共识节点的合约，并由管理员完成确认; 
* 和其他节点共同修改本地节点白名单;
* 等待区块数据同步完成后即可参与下一次的共识。

### 共识节点管理合约接口

<table>
  <tr>
    <th>名称</th>
    <th>需要权限</th>
    <th>传入参数</th>
    <th>返回值</th>
    <th>详细描述</th>
  </tr>
  <tr>
    <td>newNode(address)<br/><strong>准备共识节点</strong> </td>
    <td>普通角色</td>
    <td>新增节点地址</td>
    <td>操作是否成功 (bool)</td>
    <td>成功后新节点准备成为共识节点，并将其记录在合约共识节点列表中，同时节点将处于 new 状态</td>
  </tr>
  <tr>
    <td>approveNode(address) <br/> <strong>确认共识节点</strong></td>
    <td>管理员角色</td>
    <td>新增共识节点地址</td>
    <td>操作是否成功 (bool)</td>
    <td>新节点成功准备后，可调用此方法确认节点成为共识节点，同时节点将处于 consensus 状态</td>
  </tr>
  <tr>
    <td>deleteNode(address) <br/> <strong>删除共识节点</strong></td>
    <td>管理员角色</td>
    <td>节点地址</td>
    <td>操作是否成功 (bool)</td>
    <td>成功后节点将从节点列表中删除，同时节点将处于 close 状态</td>
  </tr>
  <tr>
    <td>listNode() <br/> <strong>获取共识节点列表</strong></td>
    <td>普通角色(只读)</td>
    <td>空</td>
    <td>地址列表(address[])</td>
    <td>获取共识节点列表，即状态为 consensus 的节点</td>
  </tr>
  <tr>
    <td>getStatus(address) <br/> <strong>获得节点状态</strong></td>
    <td>普通角色(只读)</td>
    <td>节点地址</td>
    <td>
      节点的状态 (uint8):
      <ul>
        <li>0: close 状态</li>
        <li>1: new 状态</li>
        <li>2: consensus 状态</li>
      </ul>
    </td>
    <td>获取共识节点状态</td>
  </tr>
</table>

### 共识节点增加操作示例

接着上面的操作，新节点已经增加，接下来将普通节点提升为共识节点。需要用到上面两个合约方法 `newNode(address)`/`approveNode(address)`。

节点管理合约是系统合约，默认将放在创世块上，下面是节点管理合约的 hash：

```
contract address: 0x00000000000000000000000000000000013241a2
Function signatures:
    dd4c97a0: approveNode(address)
    2d4ede93: deleteNode(address)
    30ccebb5: getStatus(address)
    609df32f: listNode()
    ddad2ffe: newNode(address)
    645b8b1b: status(address)
```

#### 首先，获取当前链上的共识节点列表：

```bash
$ curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

{"jsonrpc":"2.0","id":2,"result":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000cb9480d61bf0964687c6839670f1c3e65c1ca193000000000000000000000000dd21b5f342b017a6546a3e5455be1a6e4d6e83a10000000000000000000000000bb7249753e5dcec37c4ad3b917f10c68d64bffa00000000000000000000000011f0bba536cde870fb7c733f93d9b12ecedd13a1"}

```

- to 为合约地址，即节点合约的地址
- data 为函数 hash，此为无参数合约调用方式

返回值为目前的共识节点地址，这种地址如上文所述 `install/authorities` 文件中。最后一个就是新增节点的地址，演示中，地址为 `59a316df602568957f47973332f1f85ae1e2e75e`。

#### 构造交易格式并发送

调用合约遵循 [abi](https://solidity.readthedocs.io/en/develop/abi-spec.html), 提供工具 `make_tx.py`：

1. 构造 newNode 交易信息

    ```bash
    $ cd script/txtool/txtool

    $ python make_tx.py --to "00000000000000000000000000000000013241a2" --code "ddad2ffe00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
    ```

    - privkey 是私钥，用来签证，确认交易信息合法，系统默认的几个私钥可以看 [系统合约相关](./usage-guide/admintool)
    - code 前 8 位是函数 hash 值，即 newNode 对应的 hash，后面 64 位是函数的参数 address 的值，即节点地址，不足 64 位用 0 补齐。

    生成的交易信息存放在 `../output/transaction/deploycode` 中

2. 发送交易

    ```bash
    $ python send_tx.py 
    --> {"params": ["0a5b0a283030303030303030303030303030303030303030303030303030303030303030303133323431613212013018fface20420dc012a24ddad2ffe00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e1241bc58c97ad8979f429bac343157fd8ecb193edb8255ca256ca077d352c24161e31ad634214f5443ea27ac95a3fe0b2ef2efc2a991b26c043f193325ea12033e7400"], "jsonrpc": "2.0", "method": "cita_sendTransaction", "id": 1}
    <-- {"jsonrpc":"2.0","id":1,"result":{"hash":"0xdacbbb3697085eec3bfb0321d5142b86266a88eeaf5fba7ff40552a8350f4323","status":"OK"}} (200 OK)
    ```

3. 接收回执

    ```bash
    $ python get_receipt.py
    {
      "contractAddress": null,
      "cumulativeGasUsed": "0x5615",
      "logs": [
        {
          "blockHash": "0xe5f58cbe8d4817adabec30c93662610fd4859cf87eecc2f3a4d483d74f9b256d",
          "transactionHash": "0xdacbbb3697085eec3bfb0321d5142b86266a88eeaf5fba7ff40552a8350f4323",
          "transactionIndex": "0x0",
          "topics": [
            "0xfd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c1",
            "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
          ],
          "blockNumber": "0x89",
          "address": "0x00000000000000000000000000000000013241a2",
          "transactionLogIndex": "0x0",
          "logIndex": "0x0",
          "data": "0x"
        }
      ],
      "blockHash": "0xe5f58cbe8d4817adabec30c93662610fd4859cf87eecc2f3a4d483d74f9b256d",
      "transactionHash": "0xdacbbb3697085eec3bfb0321d5142b86266a88eeaf5fba7ff40552a8350f4323",
      "root": null,
      "errorMessage": null,
      "blockNumber": "0x89",
      "logsBloom": "0x00000000000000020040000000000000000000000000000000000000000000200000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000208000000000000000000000000000",
      "transactionIndex": "0x0",
      "gasUsed": "0x5615"
    }
    ```
    这里如果交易还没有被处理，则会发生错误，多试几次，得到回执，如果 `errorMassage` 为 null，即表示正常，继续下一步

4. 构造 approveNode 交易信息

    ```bash
    $ python make_tx.py --to "00000000000000000000000000000000013241a2" --code "dd4c97a000000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
    ```
    可以看出，只是 code 中的函数 hash 换了一下而已。

5. 发送交易

    ```bash
    $ python send_tx.py
    --> {"params": ["0a5b0a283030303030303030303030303030303030303030303030303030303030303030303133323431613212013118fface20420ef012a24dd4c97a000000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e124177a025eaafcda1f28f4b2eedd1c8ecb0d339b141e452a3bd8736cd9abc45e7387af7ab41045df5646aa411e7cac1b3a8b78e7efc81b356877afcf4a2080c06d500"], "jsonrpc": "2.0", "method": "cita_sendTransaction", "id": 1}
    <-- {"jsonrpc":"2.0","id":1,"result":{"hash":"0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4","status":"OK"}} (200 OK)
    ```

6. 接收回执

    ```bash
    $ python get_receipt.py 
    {
      "contractAddress": null,
      "cumulativeGasUsed": "0xcf15",
      "logs": [
        {
          "blockHash": "0x17b24208a3f9af4d8ef65aa385116fc35e789477026bdc7a0fbef162047bdf98",
          "transactionHash": "0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4",
          "transactionIndex": "0x0",
          "topics": [
            "0x5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb",
            "0x00000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"
          ],
          "blockNumber": "0x9b",
          "address": "0x00000000000000000000000000000000013241a2",
          "transactionLogIndex": "0x0",
          "logIndex": "0x0",
          "data": "0x"
        }
      ],
      "blockHash": "0x17b24208a3f9af4d8ef65aa385116fc35e789477026bdc7a0fbef162047bdf98",
      "transactionHash": "0xd6b38b125efcacb8d59379eef9394e3d9d4f7bb4151e53f0c2c50682f9f037b4",
      "root": null,
      "errorMessage": null,
      "blockNumber": "0x9b",
      "logsBloom": "0x00000000000000020040000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000040000000000000000000000000000000000000000000010000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000",
      "transactionIndex": "0x0",
      "gasUsed": "0xcf15"
    }
    ```

7. 查看当前的共识节点数

    ```bash
    $ curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

    {"jsonrpc":"2.0","id":2,"result":"0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005000000000000000000000000cb9480d61bf0964687c6839670f1c3e65c1ca193000000000000000000000000dd21b5f342b017a6546a3e5455be1a6e4d6e83a10000000000000000000000000bb7249753e5dcec37c4ad3b917f10c68d64bffa00000000000000000000000011f0bba536cde870fb7c733f93d9b12ecedd13a100000000000000000000000059a316df602568957f47973332f1f85ae1e2e75e"}
    ```

    可以看到，返回的 result 中在最后增加了一个地址，即当前新增的节点已经成为共识节点。

> 以上代码的返回值有所删减，实际操作会略有不同
