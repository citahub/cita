# prerequirement
## Build dependencies
```
sudo apt-get install --force-yes libsnappy1v5 libsnappy-dev  capnproto  libgoogle-perftools-dev  \
    libssl-dev  libudev-dev  rabbitmq-server  google-perftools jq
```

## Install rust
CITA requires Rust nightly, We recommend installing Rust through [rustup](https://www.rustup.rs/)
```
curl https://sh.rustup.rs -sSf | sh
```
after had installed rustup, Please let it become effective  
```
source ~/.cargo/env
```
switch to specify rust nightly version
```
rustup toolchain install nightly-2017-06-29
rustup show      #show the version of nightly-2017-06-29
rustup default nightly-2017-06-29-x86_64-unknown-linux-gnu
```
Speed up by mirrors
[creates](https://mirrors.ustc.edu.cn/help/rust-crates.html) 
[toolchain](https://mirrors.ustc.edu.cn/help/rust-static.html)

# Build CITA
```
make setup
```

```
please select the way of building CITA, debug or release
make debug   or   make release
```

# Setup CITA
Install admintool
```
cd admintool
./setup.sh
```
Generate node configuration with admintool
```
cd admintool
./admintool -h  #see usage
./admintool     #generate configuration for demo
```
Nodes files were generated in `admintool/release/node{...}`

# Run CITA
If Demo configuration, open four Terminal at
```
admintool/release/node0
admintool/release/node1
admintool/release/node2
admintool/release/node3
```
Operation with node
```
./cita           #see usage
./cita setup 0   #setup node0
./cita start 0   #start node0
./cita stop 0    #stop node0
```
Scripts to start/stop all Demo nodes
```
./tests/integrate_test/cita_start.sh
./tests/integrate_test/cita_start.sh debug
./tests/integrate_test/cita_stop.sh
``` 
If node configuration is customized, just copy admintool/release/node* to the customized server, then same as above.

# unit test
```
make test
```

# integrate test
```
./tests/integrate_test/cita_basic.sh
./tests/integrate_test/cita_transactiontest.sh
./tests/integrate_test/cita_byzantinetest.sh
```

# bench	
```
make bench
```

# coverage
```
make cov
```
It will open the result html file using your browser.  
