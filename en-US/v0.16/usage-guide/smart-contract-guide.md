# 智能合约指南

## 智能合约简介

智能合约是存储在区块链上的一段代码，它们可以被区块链上的交易所触发，被触发之后，这段代码可以从区块链上读取数据或者向区块链上写入数据。

从用户角度来讲，智能合约通常被认为是一个自动担保账户，当特定的条件满足时，程序就会释放和转移资金。

从技术角度来讲，智能合约被认为是网络服务器，只是这些服务器并不是使用 IP 地址架设在互联网上，而是架设在区块链上，从而可以在其上面运行特定的合约程序。

智能合约是编程在区块链上的汇编语言，我们通常使用更高级的语言例如 Solidity 等专用语言来编写合约程序，然后将它编译成区块链可以识别的字节码。合约代码被触发后将是自动执行的，要么成功执行，要么所有的状态变化都撤消，这就避免了合约部分执行的情况。

## 编写智能合约

智能合约是由一组代码和数据组成的程序，位于以太坊区块链上的一个特殊地址中。我们以 Solidity 编写智能合约为例，简单介绍一下如何编写一个规范的合约程序，合约 HelloWorld.sol 的代码如下：

```solidity
pragma solidity ^0.4.19;

contract HelloWorld {
    uint balance;

    function update(uint amount) returns (address, uint) {
        balance += amount;
        return (msg.sender, balance);
    }
}
```

说明：上述示例中 `uint balance` 表示声明了一个状态变量，类型为 `uint`（即 256 位的无符号整数）。 `function update(uint amount) returns (address, uint)` 表示一个函数，其中参数为 `amount` ，返回值为一个元组 `(address, unit)` ，函数中返回值可以只给出类型，而无需具体的变量名。该函数的功能是更新 `HelloWord` 合约的 `balance` ，即调用合约成功执行后，将增加 `balance` 余额。

编写 Solidity 合约代码的详细说明可参考[Solidity 官方开发指南](http://solidity.readthedocs.io/en/develop/)。

## 编译合约

首先我们需要安装 Solidity 编译器，安装编译器有多种不同的方法，我们推荐安装 `solc` 编译器稳定版，具体操作如下：

```bash
sudo add-apt-repository ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install solc
```

其他安装编译器的方法可以参考[Solidity 编译安装介绍](https://solidity.readthedocs.io/en/latest/installing-solidity.html#building-from-source)。

然后使用 `sloc` 编译器命令将合约程序编译成字节码：

```bash
solc HelloWorld.sol --bin
```

智能合约编译完成后，将输出如下：

```bash
======= HelloWorld.sol:HelloWorld =======
Binary:
6060604052341561000f57600080fd5b5b60f08061001e6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806382ab890a14603d575b600080fd5b3415604757600080fd5b605b600480803590602001909190505060a4565b604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390f35b60008082600080828254019250508190555033600054915091505b9150915600a165627a7a72305820b88062f2366288fcc87eb44079fcc26956e4c806546ad0e75b6927d14de63d950029
```

生成智能合约函数调用的 Hash 值的命令如下：

```bash
solc HelloWorld.sol --hashes
```

其输出如下：

```bash
======= HelloWorld.sol:HelloWorld =======
Function signatures:
82ab890a: update(uint256)
```

## 创建合约

在 CITA 中是使用 `JSON-PRC` 来发送交易的，发送创建合约交易之前，首先需要构造创建合约的 JSON 配置文件 `config_create.json`，其内容如下：

```json
{
  "category": 1,
  "ipandport": ["127.0.0.1:1340"],
  "txnum": 1,
  "threads": 1,
  "code":
    "6060604052341561000f57600080fd5b5b60f08061001e6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806382ab890a14603d575b600080fd5b3415604757600080fd5b605b600480803590602001909190505060a4565b604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390f35b60008082600080828254019250508190555033600054915091505b9150915600a165627a7a72305820b88062f2366288fcc87eb44079fcc26956e4c806546ad0e75b6927d14de63d950029"
}
```

其中 `category` 只有两种取值，等于 1 时表示创建合约，等于 2 时表示调用合约； `ipandport` 表示接收该交易的节点 IP 地址和端口； `txnum` 表示发送的交易个数，该配置是为了以后兼容批量交易而预留的扩展接口； `threads` 表示执行该交易启动的线程个数；而 `code` 表示编译合约后生成的字节码，即上述编译合约输出的 `Binary` 。

然后使用以下命令创建智能合约：

```bash
./bin/trans_evm --config=config_create.json
```

执行成功后，便会经过 CITA 系统的处理，然后返回创建合约交易的执行结果。

## 调用合约

同理，为了使用 `JSON-PRC` 调用合约，需要构造调用合约的 JSON 配置文件 `config_call.json`，其内容如下：

```json
{
  "category": 2,
  "ipandport": ["127.0.0.1:1340"],
  "txnum": 1,
  "threads": 1,
  "code":
    "82ab890a0000000000000000000000000000000000000000000000000000000012345678"
}
```

其中需要注意的是 `category` 等于 2 表示调用合约，而 `code` 一般是由合约函数名 Hash 和它对应的参数值组成的，如果该函数没有参数，则仅仅使用函数名 Hash 即可。例如上述示例中合约函数名 Hash 即输出 `Function signatures` 下的 `update(uint256)` 对应的 Hash 值，占前面 32 比特，其中函数名为 `update` ，参数类型为 `uint256`，而参数部分是将参数按位数补齐并序列化，占后面 256 比特。如果函数包含多个参数，每个参数都占 256 比特，且参数按照函数的参数顺序排列。

然后使用以下命令调用智能合约：

```bash
./bin/trans_evm --config=config_call.json
```

执行成功后，便会经过 CITA 系统的处理，然后返回调用合约交易的执行结果。
