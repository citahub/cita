# 快速入门

## 依赖

### 系统平台要求

CITA 是基于 `Ubuntu 18.04` 稳定版开发的，在该系统版本上运行将是正确无误的。

推荐使用 `docker` 编译和部署 `CITA`，保证环境一致。

### 安装 docker

参见 [在线资料](https://yeasy.gitbooks.io/docker_practice/content/install/)

### 获取 Docker 镜像

CITA 的 Docker 镜像托管在 [DockerHub](https://hub.docker.com/r/cita/cita-build/)

可以使用 `docker pull` 命令直接从 DockerHub 获取， 参见 [在线资料](https://yeasy.gitbooks.io/docker_practice/content/image/pull.html)。

对于内网环境，也可以通过 `docker save` 和 `docker load` 传递镜像， 参见 [在线资料](https://yeasy.gitbooks.io/docker_practice/content/image/other.html)。

### 获取源码

从 Github 仓库下载 CITA 的源代码，然后切换到 CITA 的源代码目录

```shell
git clone https://github.com/cryptape/cita.git
cd cita
git submodule init
git submodule update
```

### Docker env and daemon

在源码根目录下，我们提供了`env.sh`脚本，封装了 docker 相关的操作。

运行此脚本，以实际要运行的命令作为参数，即表示在 docker 容器环境中运行相关命令。

例如：

```shell
./env.sh make debug
```

即表示在 docker 容器环境中运行`make debug`。

不带任何参数运行`./env.sh`，将直接获取一个 docker 容器环境的 shell。

如果docker容器是被root用户创建的，后续非root用户使用`./env.sh`会出现如下错误：

```shell
$ ./env.sh
  docker container cita_run_cita_secp256k1_sha3 is already running
  error: failed switching to "user": unable to find user user: no matching entries in passwd file
```
要保证操作使用的始终是同一个系统用户。

我们还提供了`daemon.sh`，用法同`env.sh`，效果是后台运行。

如果出现 docker 相关的报错，可以执行如下命令并重试：

```shell
docker kill $(docker ps -a -q)
```

## 编译

可以按照自己的需求自行选择相应的编译方式（Debug-调试模式 或 Release-发行模式）


```shell
./env.sh make debug
```

或者

```shell
./env.sh make release
```

编译生成的文件在发布件目录`target/install`下，生产环境下只需要这个目录即可。

## 生成节点配置

先切换到发布件目录：

```shell
cd target/install
```

发布件目录中的`create_cita_config.py`工具用来生成节点配置文件，包括创世块配置、节点相关配置、网络连接配置、私钥配置等。

该工具默认生成的是本地 4 个节点的 Demo 示例配置：

```shell
./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
```

生产环境中，用户需要根据实际情况更改默认配置。

使用命令`./scripts/create_cita_config.py -h`来获得详细帮助信息，允许自定义的配置包括：

* 系统管理员账户
* 网络列表，按照`IP1:PORT1,IP2:PORT2,IP3:PORT3 ... IPn:PORTn`的格式
* 出块时间间隔
* 累积多少历史交易量后进行重复交易的检查
* 系统合约详细参数
* 共识节点地址

节点初始化操作成功后，将在发布件目录下生成节点的配置文件，其生成的节点目录为：

* test-chain/0
* test-chain/1
* test-chain/2
* test-chain/3

> ***<font color=red>注意</font>***
> 对于多服务器部署，选择一台服务器执行命令之后把相关节点目录进行拷贝。
> 不可多服务器都执行配置脚本。

## 运行节点

操作节点的命令都是相同的，以下以`test-chain/0`为例进行演示。

1.  配置节点：

    ```shell
    ./env.sh ./bin/cita setup test-chain/0
    ```

2.  启动节点：

    该命令正常情况下不会返回，因此需要后台运行。

    ```shell
    ./daemon.sh ./bin/cita start test-chain/0
    ```

3.  停止节点：

    ```shell
    ./env.sh ./bin/cita stop test-chain/0
    ```

4.  其他操作

    具体使用查看命令的帮助信息：

    ```shell
    ./env.sh ./bin/cita help
    ```

> ***<font color=red>注意</font>***
> 不可到bin目录然后执行`./cita setup/start/stop test-chain/0`。
> 虽然cita在docker中运行，但是容器并没有做网络隔离。因此请勿在一台服务器上运行多个容器。也不要同时在host系统里面运行cita以及相关的rabbitmq等软件，以免造成端口冲突。

## 测试

除了上述的基本操作命令，为了方便用户对 Demo 进行相关测试，我们在目录`cita/tests/integreate_test`下提供了一些测试脚本。

以下命令在源码根目录下运行。

1.  启动 4 个节点

    ```shell
    ./env.sh tests/integrate_test/cita_start.sh
    ```

    该命令正常情况下不会返回，需要保持 shell 不退出。或者用`daemon.sh`运行。

2.  停止 4 个节点

    上一节中的命令中止，或者执行命令：

    ```shell
    ./env.sh ./tests/integrate_test/cita_stop.sh
    ```

3.  基本功能测试

    4 个节点启动并成功出块，基本功能测试然后停止 4 个节点：

    ```shell
    ./env.sh ./tests/integrate_test/cita_basic.sh
    ```

4.  发送交易测试

    ```shell
    ./env.sh ./tests/integrate_test/cita_transactiontest.sh
    ```

5.  拜占庭测试

    模拟网络异常情况下的功能测试。

    ```shell
    ./env.sh ./tests/integrate_test/cita_byzantinetest.sh
    ```

> ***<font color=red>注意</font>***
> 必须使用`./env.sh`

## 验证

* 查询节点个数

    Request:

    ```shell
    ./env.sh curl -X POST --data '{"jsonrpc":"2.0","method":"peerCount","params":[],"id":74}' 127.0.0.1:1337
    ```

    Result:

    ```shell
    {
      "jsonrpc": "2.0",
      "id": 74,
      "result": "0x3"
    }
    ```

* 查询当前块高度。

    Request:

    ```shell
    ./env.sh curl -X POST --data '{"jsonrpc":"2.0","method":"blockNumber","params":[],"id":83}' 127.0.0.1:1337
    ```

    Result:

    ```shell
    {
      "jsonrpc": "2.0",
      "id": 83,
      "result": "0x8"
    }
    ```

    返回块高度，表示节点已经开始正常出块。

> ***<font color=red>注意</font>***
> 在发布件目录(target/install)下运行节点。
> 可选择使用`./env.sh`

更多 API（如合约调用、交易查询）请参见[RPC 调用](https://cryptape.github.io/cita/zh/usage-guide/rpc/)。
