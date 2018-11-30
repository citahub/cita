# Configuration

In this chapter, we will illustrate how to config the chain itself and the microservices under each node.

[Chain Configuration](./configuration/chain_config) means the configuration of some attributes of the chain itself, system contracts, RPC interfaces, network connections between nodes, etc.. And please notice that many configurable items can only be changed before starting the chain.
This document will introduce the configurable items of the chain;
Then through a specific operation example, demonstrate how to initialize the chain before starting it;
And take you in detail to understand the directory structure of the file after the initial configuration;
Finally, an operation example will be used to demonstrate how to modify some particular configurations after starting the chain.
Though this document, you will be able to customize a chain that meets your needs.

[Microservice Configuration](./configuration/service_config) mainly refers to the configuration of each microservices. In CITA, functionalities of a blockchain node are decoupled into six microservices, including RPC, Auth, Consensus, Chain, Executor，Network. These six microservices coordinate with eath other via a message queue. 
It is flexible for operation and maintenance personnel to perform configuration adjustments based on system operation (microservice load conditions, etc.) to optimize performance.
Currently, only network microservices can be adjusted in the runtime while other modules need to be operated after stopping the chain. Automatic refresh is supported when the configuration is modified.
In this document, we will illustrate in detail about the configurable items of each microservice.
