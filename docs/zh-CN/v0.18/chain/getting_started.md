# 快速入门

CITA 是一个开源的区块链内核，任何人都可以基于 CITA 来搭建属于自己的一条区块链，在本文档中我们将带你搭建一条简单的链并运行其中的节点。

> * 如果你想一键搭建属于你自己的链，你可以选择租用 CITA 的云服务。只需根据您的需求，在云服务平台选择适合自己的方案直接租用，帮你省去准备服务器以及部署 CITA 的一系列操作。具体请参考[云服务支持](https://docs.nervos.org/nervos-appchain-docs/#/quick-start/deploy-appchain)。
> * 如果你想在 CITA 上直接开发您的应用，我们建议你使用我们已经搭好的 [AppChain 测试链](https://docs.nervos.org/nervos-appchain-docs/#/quick-start/deploy-appchain), 也可以使用万云提供的 [BaaS服务](https://docs.nervos.org/nervos-appchain-docs/#/quick-start/deploy-appchain)。

## 依赖

### 系统平台要求

系统需支持 Docker 的安装。

CITA 的 Docker 镜像托管在 [DockerHub](https://hub.docker.com/r/cita/cita-build/)
。
因为 CITA 是基于 Ubuntu 18.04 稳定版开发的，因此该镜像中封装了 Ubuntu 18.04 还有其他一些 CITA 运行所需要的配置和文件。

### 安装 Docker

参见 [在线资料](https://yeasy.gitbooks.io/docker_practice/content/install/)。

可使用下面的命令来检查 Docker 是否已经成功安装：

```
$ sudo docker run hello-world
```

### 获取 Docker 镜像

CITA 的 Docker 镜像托管在 [DockerHub](https://hub.docker.com/r/cita/cita-build/)。

可以使用 `docker pull` 命令直接从 DockerHub 获取， 参见 [在线资料](https://yeasy.gitbooks.io/docker_practice/content/image/pull.html)。

对于内网环境，也可以通过 `docker save` 和 `docker load` 传递镜像， 参见 [在线资料](https://yeasy.gitbooks.io/docker_practice/content/image/other.html)。

## 编译 CITA

>下面的操作步骤是带你获取最新的源码进行编译，若你想直接下载编译好的发布包，可前往 Github 查看目前所有的 [CITA 正式发布版本](https://github.com/cryptape/cita/releases)，直接下载你想要的版本发布包然后部署即可。

### 获取源码

从 Github 仓库下载 CITA 的源代码，然后切换到 CITA 的源代码目录

```shell
$ git clone https://github.com/cryptape/cita.git
$ cd cita
$ git submodule init
$ git submodule update
```

### 编译源代码

可以按照自己的需求自行选择相应的编译方式（Debug-调试模式 或 Release-发行模式）

```shell
$ ./env.sh make debug
```

或者

```shell
$ ./env.sh make release
```

编译生成的文件在发布件目录 `target/install` 下，生产环境下只需要这个目录即可。

> **Docker env 和 daemon 使用说明**
> 
> * 在源码根目录下，我们提供了 `env.sh` 脚本，封装了 Docker 相关的操作。
运行此脚本，以实际要运行的命令作为参数，即表示在 Docker 环境中运行相关命令。
例如：
> 
>   ```shell
>   $ ./env.sh make debug
>   ```
>
>   即表示在 Docker 环境中运行 `make debug`。
> * 不带任何参数运行 `./env.sh`，将直接获取一个 Docker 环境的 shell。
> * 国内用户请使用 `env_cn.sh`，提供了编译时的国内镜像加速。
> * 还提供了 `daemon.sh`，用法同 `env.sh`，效果是后台运行。

> **Notice**
>
> * 如果 Docker 容器是被 root 用户创建的，后续非 root 用户使用 `./env.sh` 会出现如下错误：
>
>   ```shell
>   $ ./env.sh
>   docker container cita_run_cita_secp256k1_sha3 is already running
>   error: failed switching to "user": unable to find user user: no matching entries in passwd file
>   ``` 
>   因此要保证操作使用的始终是同一个系统用户。
> * 如果出现 Docker 相关的报错，可以执行如下命令并重试：  
>   ```shell
>   docker kill $(docker ps -a -q)
>   ```

## 部署CITA

### 配置节点

* 先切换到发布件目录

  * 如果之前选择从源码开始编译：

    ```shell
    $ cd target/install
    ```
  * 如果之前选择下载编译好的发布包：
   
    ```shell
    $ cd cita_secp256k1_sha3/
    ```
     
* 使用发布件目录中的 `create_cita_config.py` 工具用来生成节点配置文件，包括创世块配置、节点相关配置、网络连接配置、私钥配置等。执行以下命令行可使用该工具生成默认的本地 4 个节点的 Demo 示例配置：

  ```shell
  $ ./env.sh ./scripts/create_cita_config.py create --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
  ```

  节点初始化操作成功后，将在发布件目录下生成节点的配置文件，其生成的节点目录为：

  * test-chain/0
  * test-chain/1
  * test-chain/2
  * test-chain/3

* 执行以下命令依次配置四个节点

  ```shell
  $ ./env.sh ./bin/cita setup test-chain/0
  $ ./env.sh ./bin/cita setup test-chain/1
  $ ./env.sh ./bin/cita setup test-chain/2
  $ ./env.sh ./bin/cita setup test-chain/3
  ```

> **Note** 
>
> * 生产环境中，用户需要根据实际情况更改默认配置。使用命令 `./scripts/create_cita_config.py -h` 来获得详细帮助信息，允许自定义的配置包括：
>   * 系统管理员账户
>   * 出块时间间隔
>   * 累积多少历史交易量后进行重复交易的检查
>   * 系统合约详细参数
>   * 共识节点地址
>
>   该工具更详细的使用说明请参考 [Config Tool](./chain/config_tool)。
> * 对于多服务器部署一条链，选择一台服务器执行命令之后把相关节点目录进行拷贝。不可多服务器都执行配置脚本。
> * 在不同服务器部署多条链主要规划相关端口配置，参见 [Config_Tool的功能和用法](./chain/config_tool)。在同一台服务器上部署多条链，除了规划端口配置外，由于 `RabbitMQ` 系统服务限制，多条链只能在一个Docker里运行。基于上面 test-chain 链所在的目录，生成一条新链：
>
>   ```shell
>   $ ./env.sh ./scripts/create_cita_config.py create --chain_name test2-chain --jsonrpc_port 2337 --ws_port 5337 --grpc_port 6000 --nodes "127.0.0.1:8000,127.0.0.1:8001,127.0.0.1:8002,127.0.0.1:8003"
>   ```
>
>   运行 test2-chain 方式与上面 test-chain 一致，并且只能在同一个Docker 里运行。

### 启动节点

执行以下命令依次启动四个节点，该命令正常情况下不会返回，节点后台运行。

```shell
$ ./daemon.sh ./bin/cita start test-chain/0
$ ./daemon.sh ./bin/cita start test-chain/1
$ ./daemon.sh ./bin/cita start test-chain/2
$ ./daemon.sh ./bin/cita start test-chain/3
```

### 停止节点

以“0”节点为例，执行以下命令即可停止“0”节点：

```shell
$ ./env.sh ./bin/cita stop test-chain/0
```

### 其他操作

更多其他操作使用以下命令查看帮助信息：

```shell
$ ./env.sh ./bin/cita help
```

>**Notice**
>
> * 请不要先进到 bin 目录，再执行以上的部署操作，错误示范：
>
>   ```shell
>   $ cd bin
>   $ ./env.sh .cita setup test-chain/0
>   ```
>
> * 请勿在一台服务器上运行多个容器。因为虽然 CITA 在 Docker 中运行，但是容器并没有做网络隔离。
> * 请不要同时在 host 系统里面运行 CITA 以及相关的 RabbitMQ 等软件，以免造成端口冲突


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

!> 必须使用`./env.sh`

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

!> 在发布件目录(target/install)下运行节点时，可选择使用`./env.sh`

更多 API（如合约调用、交易查询）请参见[RPC 调用](./rpc_guide/rpc)。
