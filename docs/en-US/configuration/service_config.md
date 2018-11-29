# Microservice Configuration

The most notable feature of CITA is the microservice architecture, that is, in CITA, functionalities of a node are decoupled into six microservices, including RPC, Auth, Consensus, Chain, Executor, Network. These six microservices coordinating with each other via message queue to complete the node's task. 

Let's take a look at the configuration files which are .toml files under the `test-chain/*/` path. (`test-chain` is the default name of the chain).

```bash
$ ./env.sh ./scripts/create_cita_config.py create --super_admin "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523" --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003"
$ ls test—chain/
  0 1 2 3 template
$ ls 0
  auth.toml executor.toml jsonrpc.toml chain.toml forever.toml logs
  consensus.toml network.toml genesis.json privkey
```

The following sections will show you how to config each microservice.

## Auth

Auth.toml is the configuration file for the Auth microservice, as follows:

```bash

count_per_batch = 30
buffer_duration = 30
tx_verify_thread_num = 4
tx_verify_cache_size = 100000
tx_pool_limit = 0
wal_enable = false
prof_start = 0
prof_duration = 0

```

* `count_per_batch` : threshold of batch processing 
* `buffer_duration`: timeout period (when the batch quantity is reached or the timeout period is up, the transaction processing flow starts)
* `tx_verify_thread_num` : number of transaction verification threads
* `tx_verify_cache_size` : size of transaction verification cache (reduce double counting)
* `tx_pool_limit` : the maximum number of transactions in trading pools (default is 0, indicating no limit)
* `wal_enable` : Transaction persistence switch. If `Wal_enable = true`, transactions  would be persisted, which means these transactions would not be lost after node restart
* `prof_start`: Performance sampling parameter, indicating how long after starting to perform the performance sampling, in seconds
* `prof_duration`: Performance sampling parameter, indicating the sampling duration, in seconds. `Prof_duration = 0` indicating no sampling

## Consensus

Consensus.toml is the configuration file for the Consensus microservice, as follows:

```bash

[ntp_config]
enabled = true
threshold = 1000
address = "0.pool.ntp.org:123"

```

* `enabled` : `Enabled = true` means to enable ntp
* `threshold` : threshold of the time offset
* `address` : address of the ntp server

## Chain

Chain.toml is the configuration file for the Chain microservice, as follows:

```bash

prooftype = 2

```

* `prooftype`: type of consensus algorithm (CITA only supports the CITA-BFT algorithm in current)

## Executor

Executor.toml is the configuration file for the Executor microservice as follows:

```bash

journaldb_type = "archive"
prooftype = 2
grpc_port = 5000
statedb_cache_size = 5242880

```

* `journaldb_type` : type of JournalDB algorithm. There are 4 types, including "archive", "light", "fast" and "basic". The default is `archive`
* `prooftype` : type of consensus algorithm, (CITA only supports the CITA-BFT algorithm in current)
* `grpc_port` : grpc port
* `statedb_cache_size`:  size of global cache in StateDB, which is used to save account and code. The default is 5242880, that is, 5M.

## RPC

Jsonrpc.toml is the configuration file for the  RPC microservice. CITA supports JsonRpc and Websocket communication protocols. 

```shell
backlog_capacity = 1000

[profile_config]
flag_prof_start = 0
enable = false
flag_prof_duration = 0

[http_config]
allow_origin = "*"
timeout = 3
enable = true
listen_port = "1337"
listen_ip = "0.0.0.0"

[ws_config]
panic_on_internal = true
fragments_grow = true
panic_on_protocol = false
enable = true
in_buffer_capacity = 2048
panic_on_queue = false
fragment_size = 65535
panic_on_timeout = false
method_strict = false
thread_number = 2
panic_on_capacity = false
masking_strict = false
key_strict = false
max_connections = 800
listen_ip = "0.0.0.0"
listen_port = "4337"
queue_size = 200
fragments_capacity = 100
tcp_nodelay = false
shutdown_on_interrupt = true
out_buffer_grow = true
panic_on_io = false
panic_on_new_connection = false
out_buffer_capacity = 2048
encrypt_server = false
in_buffer_grow = true
panic_on_shutdown = false
panic_on_encoding = false

[new_tx_flow_config]
buffer_duration = 30000000
count_per_batch = 30
```

* `backlog_capacity`: connection capacity
* `profile_config`: performance sampling analysis
  - `flag_prof_start`: how long after starting to perform the performance sampling, in seconds 
  - `enable`: switch
  - `flag_prof_duration`: sampling duration, in seconds. `Prof_duration = 0` indicating no sampling
* `http_config`:
  - `allow_origin`: reponse header. `*` indicate allowing all origins
  - `timeout`: timeout value
  - `enable`: switch
  - `listen_port`: listener port
  - `listen_ip`: listener IP address
* `ws_config`: 
  - `panic_on_internal`: whether to exit when an internal error occurs. True means to exit
  - `fragments_grow`: whether to reassign when fragments_capacity is reached. True means to reassign.
  - `panic_on_protocol`: whether to exit when a protocol error occurs. Fause means not to exit
  - `enable`: switch
  - `in_buffer_capacity`: input cache size without the dynamic increase. The default is 2048
  - `panic_on_queue`: whether to exit when a queue error occurs. The default is false, which means not to exit
  - `fragment_size`: the max limit of frame fragment length, after which it is truncated into fragments, default 65535
  - `panic_on_timeout`: whether to exit when a timeout occurs. The default is false, which means to exit
  - `method_strict`: whether to check the handshake request. The default is false, which means not to check
  - `thread_number`: number of threads. The default is 2
  - `panic_on_capacity`: whether to exit when capacity is reached. The default is false
  - `masking_strict`: frame security check. The default is false
  - `key_strict`: whether the client checks the key value returned by the server. The default is false.
  - `max_connections`: maximum allowable connections of WebSocket. The default is 800
  - `listen_ip`: listening address, default 0.0.0.0
  - `listen_port`: listening port, default 4337
  - `queue_size`: event queue size for a single connection. The default is 200
  - `fragments_capacity`: the maximum number of fragments that the connection can handle without dynamic addition. The default is 100
  - `tcp_nodelay`: whether TCP socket accumulates packets to a certain number, and send together. The default is false
  - `shutdown_on_interrupt`: whether the event listener is turned off when the interrupt occurs. The default is true
  - `out_buffer_grow`: whether it is dynamically incremented when the output buffer reaches out_buffer_capacity. The default is true
  - `panic_on_io`: Whether to exit when an IO error occurs. The default is false
  - `panic_on_new_connection`: Whether to exit after a TCP connection failure. The default is false.
  - `out_buffer_capacity`: Output cache size without the dynamic increase. The default is 2048
  - `encrypt_server`: whether the server accepts the connections with SSL encryption. The default is false
  - `in_buffer_grow`: whether it is dynamically incremented when the input buffer reaches in_buffer_capacity. The default is true
  - `panic_on_shutdown`: Whether to exit when receiving a WebSocket stop request. The default is false
  - `panic_on_encoding`: Whether to exit when the encoding problem occurs. The default is false
* `new_tx_flow_config`:
  - `buffer_duration`: timeout period
  - `count_per_batch`: threshold of batch processing 

## Network

Network.toml is the configuration file for Network Microservices. The file records the total number of nodes, the local node port, and the IP and port of other nodes. The user can add nodes by adding node information and support hot update. Copy the modified file directly to cover older one, and it will take effect without restarting the process. 

```shell
# Current node ip is 127.0.0.1
id_card = 0
port = 4000
[[peers]]
id_card = 1
ip = "127.0.0.1"
port = 4001

[[peers]]
id_card = 2
ip = "127.0.0.1"
port = 4002

[[peers]]
id_card = 3
ip = "127.0.0.1"
port = 4003

```

## Forever

Forever.toml is the daemon's configuration file. Each process corresponds to a microservice, and `respawn` indicates the number of wakeups.

```
name="cita-forever"
command = "cita-forever"
pidfile = ".cita-forever.pid"

[[process]]
name = "cita-auth"
command = "cita-auth"
args = ["-c","auth.toml"]
pidfile = ".cita-auth.pid"
respawn = 3

[[process]]
name = "cita-network"
command = "cita-network"
args = ["-c","network.toml"]
pidfile = ".cita-network.pid"
respawn = 3

[[process]]
name = "cita-bft"
command = "cita-bft"
args = ["-c","consensus.toml","-p","privkey"]
pidfile = ".cita-bft.pid"
respawn = 3

[[process]]
name = "cita-jsonrpc"
command = "cita-jsonrpc"
args = ["-c","jsonrpc.toml"]
pidfile = ".cita-jsonrpc.pid"
respawn = 3


[[process]]
name = "cita-chain"
command = "cita-chain"
args = ["-c","chain.toml"]
pidfile = ".cita-chain.pid"
respawn = 3

[[process]]
name = "cita-executor"
command = "cita-executor"
args = ["-g","genesis.json","-c","executor.toml"]
pidfile = ".cita-executor.pid"
respawn = 3
```