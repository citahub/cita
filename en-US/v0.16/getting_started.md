# Getting Started 

## Dependencies

### System platform requirements

CITA is developed based on the stable version of Ubuntu 16.04 and runs robustly on this version.

It is recommended to use docker to compile and deploy CITA to ensure a consistent environment.

### Install docker

see [online information](https://yeasy.gitbooks.io/docker_practice/content/install/)

### Get Docker Image

CITA docker image is hosted on [DockerHub](https://hub.docker.com/r/cita/cita-build/)

This can be obtained directly from the DockerHub using the `docker pull` command. See [online information](https://yeasy.gitbooks.io/docker_practice/content/image/pull.html)。

For intranet environments, you can also use `docker save` and `docker load` command to deliver image. See [online information](https://yeasy.gitbooks.io/docker_practice/content/image/other.html)。

### Get source code

Download CITA source code from Github repository, and switch to CITA source directory.

```shell
git clone https://github.com/cryptape/cita.git
cd cita
git submodule init
git submodule update
```

### Docker env and daemon

In the root directory of the source code, we provide `env.sh`script，which encapsulates docker-related operations.

Running this script with actual commands that you want to run inside docker environment as arguments.

For example：

```shell
./env.sh make debug
```

This means running`make debug`in docker.。

Running`./env.sh`without any arguments will directly get a shell in docker.

For users in China, speed up by using `env_cn.sh`.

We also provided`daemon.sh`, same usage as`env.sh`，but run in background.

If there are some docker-related errors, you can try again after executing the following command：

```shell
docker kill $(docker ps -a -q)
```

## Compile

You can choose the compilation method according to your needs (Debug or Release)

```shell
./env.sh make debug
```

or

```shell
./env.sh make release
```

The generated file is under the`target/install`. You only need to operate under this directory in production environment.。

## Generate node configuration

Switch to release directory at first:

```shell
cd target/install
```

The`admintool`in the release directory is used to generate the node configuration file, including the Genesis block configuration, node-related configuration, network connection configuration, and private key configuration.

The tool defaults to generate a Demo with 4 local nodes:

```shell
./env.sh ./bin/admintool.sh
```

In the production environment, user needs to change the default configuration according to the actual situation.

Use`admintool.sh -h`to get detailed help information, allowing custom configurations to include:

* System administrator account
* Network list, in the format of`IP1:PORT1,IP2:PORT2,IP3:PORT3 ... IPn:PORTn`
* Consensus algorithm. There are three choices: `cita-bft`、`raft` 和`poa`
* Encryption method
* Blocking interval
* Limit number of transactions in a single block
* Check for repeated transactions after accumulating a certain amount of historical transactions

After the node initialization, the node configuration file will be generated in the release directory. The generated node directory is:

* node0
* node1
* node2
* node3

## Run nodes

The commands of operation the nodes are the same. Take`node0`as an example.

1. Configure the node:

    ```shell
    ./env.sh ./bin/cita setup node0
    ```

2. Start the node：

    This command does not return normally, so it needs to run in the background.
	
    ```shell
    ./daemon.sh ./bin/cita start node0
    ```	

3. Stop the node：

    ```shell
    ./env.sh ./bin/cita stop node0
    ```

4. Other operations

    use help for detailed information：

    ```shell
    ./env.sh ./bin/cita help
    ```

## Build test environment

There are two ways to set up test environment.

- You can start 4 nodes one by one as mentioned in previous section. When you do not need to use them, close them one by one.
- You can also start and shut down nodes in batches by using the following script.
	
    The following commands run in the source root directory.
	
	- Start 4 nodes
		
        ```shell
        ./env.sh tests/integrate_test/cita_start.sh
        ```
		
        This command does not return normally and you need to keep the shell from exiting. Or run with`daemon.sh`.

	- Stop 4 nodes
		
        ```shell
        ./env.sh ./tests/integrate_test/cita_stop.sh
        ```


## Test

***Need to be executed after the test environment is set up***

In addition to the above basic operation commands, in order to facilitate user to test Demo, we provide some test scripts under the`cita/tests/integreate_test`.

The following command is run in the source root directory.

1.  Basic function test

    4 nodes runn and generate blocks successfully. After basic function tests, stop 4 nodes.

    ```shell
    ./env.sh ./tests/integrate_test/cita_basic.sh
    ```

2.  Transaction test

    ```shell
    ./env.sh ./tests/integrate_test/cita_transactiontest.sh
    ```

3.  Byzantine test

    Functional tests under abnormal network conditions.

    ```shell
    ./env.sh ./tests/integrate_test/cita_byzantinetest.sh
    ```

## Verification

***Need to be executed after the test environment is set up***


- Query the number of nodes.

    Request:

    ```shell
    ./env.sh curl -X POST --data '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":74}' 127.0.0.1:1337
    ```

    Result:

    ```shell
    {
        "jsonrpc": "2.0",
        "id": 74,
        "result": "0x3"
    }
    ```

- Query the current block height.

    Request:

    ```shell
    ./env.sh curl -X POST --data '{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":83}' 127.0.0.1:1337
    ```

    Result:

    ```shell
    {
        "jsonrpc": "2.0",
        "id": 83,
        "result": "0x8"
    }
    ```

    Return the block height, indicating that the node has started to block out normally.

More APIs (such as contract calls, transaction queries),please check[RPC calls](./usage-guide/rpc)。
