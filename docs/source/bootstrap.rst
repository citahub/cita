===========
 Bootstrap
===========

With steps described in this article, we can quickly build our federo chain ``CITA`` to develop and test.


Deploy nodes
============

To simplify deployment on multiple machine and quickly build run&develop environment, it is recommended to deploy ``CITA`` with docker.

**install docker**


install ``docker`` and ``docker-compose``

::

    sudo apt-get install docker docker-compose  docker.io


For user in china, please use mirror of ``hub.docker.com`` to speed up image pulling：

::

    sudo cat > /etc/docker/daemon.json  << EOF
    {
       "registry-mirror": [ "https://registry.docker-cn.com" ]
    }
    EOF


**fetch CITA image**



There are two way to fetch ``CITA`` image:

1. Directly pull from ``hub.docker.com`` 

::

   sudo docker pull cryptape/play

2. Alternatively, build from release artifact

- build from binary artifacts 

::

    wget https://github.com/cryptape/cita/releases/download/v0.10/cita.tar.bz2
    tar xf cita-v0.1.0.tar.bz2
    cd cita
    scripts/build_image_from_binary.sh

Check ``cryptape/play`` image from output of command ``docker images`` when it finished.

::

   $ docker images
   REPOSITORY            TAG                 IMAGE ID            CREATED             SIZE
   cryptape/play         <none>              6c12ed009d90        2 weeks ago         1.85 GB

- build from source

::

    git clone http://github.com/cryptape/cita
    cd cita
    scripts/build_image_from_source.sh


**deploy CITA with docker-compose**


- preparation

::

    mkdir cita
    cd cita
    wget https://raw.githubusercontent.com/cryptape/cita/develop/scripts/docker-compose.yaml


- generate configuration

::

    sudo docker-compose run admin


- start nodes

::

    # start single node
    sudo docker-compose start node0
    # start all nodes
    sudo docker-compose up node0 node1 node2 node3


- stop nodes

::

    # stop single node
    sudo docker-compose stop node0
    # stop all nodes
    sudo docker-compose down


Deploy smart contract
=====================

``solidity`` is the recommended smart contract language, and ``CITA``
is complete compatible with it. Below we will write and test smart
contract with ``solidity``


**compile contract source to get contract deployment code**

contract source can be compiled with multiple way.

1. access from online web ide <https://remix.ethereum.org/>
2. alternatively, install ``solc`` compiler and compile contract source from command line

please refer to here  <http://www.ethdocs.org/en/latest/ethereum-clients/cpp-ethereum/index.html> for installation.

After installation finished, check result from shell using ``solc --version``:

::

   $ solc --version
   solc, the solidity compiler commandline interface
   Version: 0.4.15-develop.2017.9.28+commit.9fb91ee6.mod.Linux.g++

If output displayed as above, we can compiled ``solidity`` contract source using ``solc`` command

Docker image ``cryptape/play`` supplied smart contract demo at ``/cita/scripts/contracts/tests``.

``test_example.sol`` is a simple contract source, only provide ``get`` and ``set`` method. To check deployment and call of contract, we can store a value using ``set`` method, then read it using ``get`` method.

If compiled successfully, it will output contract deployment code.
::

    solc test_example.sol --bin


**construct transaction data**
``--code`` refer to deployment code from previous step, and ``--privkey`` refer to private key.

::

   cd scripts/txtool/txtool
   python make_tx.py --privkey "352416e1c910e413768c51390dfd791b414212b7b4fe6b1a18f58007fa894214" --code     "606060405234156100105760006000fd5b610015565b60e0806100236000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604b5780636d4ce63c14606c576045565b60006000fd5b341560565760006000fd5b606a60048080359060200190919050506093565b005b341560775760006000fd5b607d60a3565b6040518082815260200191505060405180910390f35b8060006000508190909055505b50565b6000600060005054905060b1565b905600a165627a7a72305820942223976c6dd48a3aa1d4749f45ad270915cfacd9c0bf3583c018d4c86f9da20029"



**send transation**
::

   python send_tx.py

If it executed successully, it will output as below.
::

    {
        "jsonrpc":"2.0",
        "id":1,
        "result":
        {
            "hash":"0xa02fa9a94de11d288449ccbe8c5de5916116433b167eaec37455e402e1ab53d3",
            "status":"Ok"
        }
    }


``status`` with value ``OK``, means transation with contract deployment have sent to nodes.

**get receipt**

::

   python get_receipt.py

If it executed successully, it will output as below.
::

    {
        "contractAddress": "0x73552bc4e960a1d53013b40074569ea05b950b4d",          
        "cumulativeGasUsed": "0xafc8",
        "logs": [],
        "blockHash": "0x14a311d9f026ab592e6d156f2ac6244b153816eeec18717802ee9e675f0bfbbd",
        "transactionHash": "0x61854d356645ab5aacd24616e59d76ac639c5a5c2ec79292f8e8fb409b42177b",
        "root": null,
        "errorMessage": null,
        "blockNumber": "0x6",                                                     
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "transactionIndex": "0x0",
        "gasUsed": "0xafc8"
    }

remark:  ``contractAddress`` and  ``blockNumber`` will be used below steps.


Call smart contract method
==========================
**Call method set**

***get contract method's hash***

Contract method hash can be obtained from ``solc --hash``.

::

   solc test_example.sol --hash

Result as below:
::

    ======= example.sol:SimpleStorage =======
    Function signatures:
    6d4ce63c: get()                         
    60fe47b1: set(uint256)                  


Here ``get`` and ``set`` method ``hash`` is unique id in contract ``SimpleStorage``, and will be used in next step

***construct transaction data***

::

    python make_tx.py --privkey "352416e1c910e413768c51390dfd791b414212b7b4fe6b1a18f58007fa894214" --to "73552bc4e960a1d53013b40074569ea05b950b4d" --code "60fe47b10000000000000000000000000000000000000000000000000000000000000001"


``--privkey`` means your private key, ``--to`` means contract address(from contractAddress in previous steps), ``--code`` means method hash(``set``) and parameters. 

***send transation***

::

   python sen_tx.py

Output displayed as below:   
::

    {
        "jsonrpc":"2.0",
        "id":1,
        "result":
        {
            "hash":"0xf29935d0221cd8ef2cb6a265e0a963ca172aca4f6e43728d2ccae3127631d590",
            "status":"Ok"
        }
    }

***get receipt***

::

   python get_receipt.py

Output displayed as below:   
::

    {
        "contractAddress": null,
        "cumulativeGasUsed": "0x4f2d",
        "logs": [],
        "blockHash": "0x2a10ae38be9e1816487dbfb34bce7f440d60035e8978146caef5d14608bb222c",
        "transactionHash": "0xf29935d0221cd8ef2cb6a265e0a963ca172aca4f6e43728d2ccae3127631d590",
        "root": null,
        "errorMessage": null,
        "blockNumber": "0x15",  
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "transactionIndex": "0x0",
        "gasUsed": "0x4f2d"
    }


**call method get, check previous value setting**

::

    curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x73552bc4e960a1d53013b40074569ea05b950b4d", "data":"0x6d4ce63c"}, "0x15"],"id":2}' 127.0.0.1:1337

Here ``to`` means contract address, ``data``means method(``get``) hash, and ``0x15`` means block number from previous step.

Result displayed as below:
::

    {
        "jsonrpc":"2.0",
        "id":2,
        "result":"0x0000000000000000000000000000000000000000000000000000000000000001"
    }


Here ``result`` is ``1``, same with previous setting to ``1``.

