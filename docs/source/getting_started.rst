依赖
=============

系统平台要求
---------------------------

CITA的运行环境是Linux和OSX操作系统，目前不支持Windows系统。CITA是基于Ubuntu 16.04稳定版开发的，在该系统版本上运行将是正确无误的。如果在Linux系统的其他版本上运行出现问题，建议将系统版本切换到Ubuntu 16.04版本。

安装编译器及开发库
---------------------------

::

   sudo scripts/install_develop.sh
   
安装完成后，可以重新登录使Rust相关的环境变量生效，也可直接使用以下命令立即生效：
::

   source ~/.cargo/env

经过如上设置，CITA的依赖便安装完成了。

安装
=============

从Github仓库下载CITA的源代码，然后切换到CITA的源代码目录
::

  cd cita

单元测试依赖rabbitmq, 如果没有启动, 需要用以下脚本启动并配置
::

   sudo scripts/config_rabbitmq.sh

可以按照自己的需求自行选择相应的编译方式（Debug-调试模式 或 Release-发行模式）
::

  make debug

或者
::

  make release

编译成功后，其生成的可执行文件将放在 ``target/install`` 目录下，生产环境下只能看到target/install里面的内容。


配置
=============
先切换到发布件目录,并将bin目录加入到PATH环境变量中:
::

   cd target/install
   export PATH=$PWD/bin:$PATH
   
启动的公共脚本位于 ``scripts/admintool`` 目录，主要用来创建创世块配置、节点相关配置、网络连接配置、私钥配置等相关文件。  

设置节点的配置信息，该默认示例Demo中配置了4个节点，对Demo中的节点进行默认初始化的操作命令为：
::

   admintool.sh

此外，用户可以根据需要更改其中的默认配置，使用命令 ``admintool.sh -h`` 来获得详细帮助，允许自定义配置包括：

* 系统管理员账户
* 网络列表，按照 ``IP1:PORT1,IP2:PORT2,IP3:PORT3 ... IPn:PORTn`` 的格式
* 共识算法选择，可供选择的有 ``tendermint`` 、 ``raft`` 和 ``poa``
* 加密方法选择
* 出块时间间隔
* 单数据块中交易数量限制
* 累积多少历史交易量后进行重复交易的检查

节点初始化操作成功后，将在发布件目录下生成节点的配置文件，其生成的节点目录为:

* node0
* node1
* node2
* node3

可以使用 ``cita start node0`` 等命令对节点进行启动和停止等操作了。

运行
=============

启动节点的服务步骤都是相同的，以 ``node0`` 为例，其启动CITA节点的具体步骤为：

1）启动节点 ``node0`` 之前需进行初始化：

.. code-block:: none

  cita setup 0

2）启动节点 ``node0`` 的服务：

.. code-block:: none

  cita start 0

而停止节点 ``node0`` 服务只需执行以下操作：

.. code-block:: none

  cita stop 0

此外， ``cita`` 命令中还包括其他操作，具体使用可以查看相关说明：
::

  cita

除了上述的基本操作命令，为了方便用户对Demo进行相关测试，我们在目录 ``cita/tests/integreate_test`` 下提供了一些测试脚本。
例如，测试所有节点服务启动并成功出块，然后停止节点服务的操作为：
::

  ./cita_start.sh

停止所有节点服务的命令为：
::

  ./cita_stop.sh

备注：以上示例Demo的节点启动都是位于同一台机器上，如果需要部署到不同的服务器上，只需删除其他节点的配置("target/install/nodeX"),并保留自己节点的配置,然后将整个目录（即 ``target/install`` 目录）拷贝到其他服务器上运行即可。


验证
=============

- 查询节点个数

Request:
::

    curl -X POST --data '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":74}' 127.0.0.1:1337 | jq


Result:
::

    {
      "jsonrpc": "2.0",
      "id": 74,
      "result": "0x3"
    }


- 查询当前块高度。

Request:
::

    curl -X POST --data '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":83}' 127.0.0.1:1337 | jq


Result:
::

    {
      "jsonrpc": "2.0",
      "id": 83,
      "result": "0x8"
    }

返回块高度，表示节点已经开始正常出块。

更多API（如合约调用、交易查询）请参见 RPC调用_。

.. _RPC调用: rpc.html
