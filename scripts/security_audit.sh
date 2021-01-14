#!/usr/bin/env bash

if [[ "$(git log -n 1 --format="%s")" =~ \[skip\ audit\] ]]; then
    echo "[Info_] Skip Security Audit."
    exit 0
fi

command -v cargo-audit
ret=$?
if [ "${ret}" -ne 0 ]; then
    echo "[Info_] Install Security Audit."
    cargo install cargo-audit
fi

echo "[Info_] Run Security Audit."
# TODO: fix audit error
: "
Crate:         arc-swap
Version:       0.4.4
Title:         Dangling reference in access::Map with Constant
Date:          2020-12-10
ID:            RUSTSEC-2020-0091
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0091
Solution:      Upgrade to >=1.1.0 OR >=0.4.8
Dependency tree:
arc-swap 0.4.4

Crate:         bumpalo
Version:       3.2.0
Title:         Flaw in realloc allows reading unknown memory
Date:          2020-03-24
ID:            RUSTSEC-2020-0006
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0006
Solution:      Upgrade to >=3.2.1
Dependency tree:
bumpalo 3.2.0
└── wasm-bindgen-backend 0.2.59
    └── wasm-bindgen-macro-support 0.2.59
        └── wasm-bindgen-macro 0.2.59
            └── wasm-bindgen 0.2.59
                ├── web-sys 0.3.36
                │   └── ring 0.16.11
                │       └── tentacle-secio 0.2.2
                │           └── tentacle 0.2.7
                │               ├── tentacle-discovery 0.2.9
                │               │   └── cita-network 20.2.0
                │               └── cita-network 20.2.0
                └── js-sys 0.3.36
                    └── web-sys 0.3.36

Crate:         futures-task
Version:       0.3.5
Title:         futures_task::waker may cause a use-after-free if used on a type that isn't 'static
Date:          2020-09-04
ID:            RUSTSEC-2020-0060
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0060
Solution:      Upgrade to >=0.3.6
Dependency tree:
futures-task 0.3.5
└── futures-util 0.3.5
    ├── hyper 0.13.7
    └── h2 0.2.6

Crate:         futures-util
Version:       0.3.5
Title:         MutexGuard::map can cause a data race in safe code
Date:          2020-10-22
ID:            RUSTSEC-2020-0059
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0059
Solution:      Upgrade to >=0.3.7
Dependency tree:
futures-util 0.3.5
├── hyper 0.13.7
└── h2 0.2.6

Crate:         libsecp256k1
Version:       0.2.2
Title:         Flaw in Scalar::check_overflow allows side-channel timing attack
Date:          2019-10-14
ID:            RUSTSEC-2019-0027
URL:           https://rustsec.org/advisories/RUSTSEC-2019-0027
Solution:      Upgrade to >=0.3.1
Dependency tree:
libsecp256k1 0.2.2
├── create-genesis 0.1.0
└── cita-vm 0.2.1
    ├── create-genesis 0.1.0
    ├── core-executor 0.1.0
    │   └── cita-executor 20.2.0
    ├── common-types 0.1.0
    │   ├── core-executor 0.1.0
    │   ├── core 0.1.0
    │   │   ├── core-executor 0.1.0
    │   │   ├── cita-relayer-parser 0.1.0
    │   │   └── cita-chain 20.2.0
    │   ├── cita-executor 20.2.0
    │   └── cita-chain 20.2.0
    └── cita-executor 20.2.0

Crate:         ordered-float
Version:       1.0.2
Title:         ordered_float:NotNan may contain NaN after panic in assignment operators
Date:          2020-12-06
ID:            RUSTSEC-2020-0082
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0082
Solution:      Upgrade to >=1.1.1, <2.0.0 OR >=2.0.1
Dependency tree:
ordered-float 1.0.2
└── serde-value 0.5.3
    └── log4rs 0.8.3
        └── cita-logger 0.1.1
            ├── util 0.6.0
            │   ├── tx_pool 0.6.0
            │   │   └── cita-auth 20.2.0
            │   ├── engine 0.6.0
            │   │   └── cita-bft 20.2.0
            │   ├── core-executor 0.1.0
            │   │   └── cita-executor 20.2.0
            │   ├── core 0.1.0
            │   │   ├── core-executor 0.1.0
            │   │   ├── cita-relayer-parser 0.1.0
            │   │   └── cita-chain 20.2.0
            │   ├── common-types 0.1.0
            │   │   ├── core-executor 0.1.0
            │   │   ├── core 0.1.0
            │   │   ├── cita-executor 20.2.0
            │   │   └── cita-chain 20.2.0
            │   ├── cita-network 20.2.0
            │   ├── cita-jsonrpc 20.2.0
            │   ├── cita-forever 1.0.0
            │   ├── cita-executor 20.2.0
            │   ├── cita-chain 20.2.0
            │   ├── cita-bft 20.2.0
            │   └── cita-auth 20.2.0
            ├── libproto 0.6.0
            │   ├── tx_pool 0.6.0
            │   ├── proof 0.6.0
            │   │   ├── jsonrpc-proto 0.1.0
            │   │   │   └── cita-jsonrpc 20.2.0
            │   │   ├── core-executor 0.1.0
            │   │   ├── core 0.1.0
            │   │   ├── common-types 0.1.0
            │   │   ├── cita-executor 20.2.0
            │   │   ├── cita-chain 20.2.0
            │   │   ├── cita-bft 20.2.0
            │   │   └── chain-executor-mock 0.1.0
            │   ├── jsonrpc-proto 0.1.0
            │   ├── create-genesis 0.1.0
            │   ├── core-executor 0.1.0
            │   ├── core 0.1.0
            │   ├── common-types 0.1.0
            │   ├── cita-relayer-parser 0.1.0
            │   ├── cita-network 20.2.0
            │   ├── cita-jsonrpc 20.2.0
            │   ├── cita-executor 20.2.0
            │   ├── cita-chain 20.2.0
            │   ├── cita-bft 20.2.0
            │   ├── cita-auth 20.2.0
            │   └── chain-executor-mock 0.1.0
            ├── jsonrpc-proto 0.1.0
            ├── core-executor 0.1.0
            ├── core 0.1.0
            ├── common-types 0.1.0
            ├── cita-relayer-parser 0.1.0
            ├── cita-network 20.2.0
            ├── cita-jsonrpc 20.2.0
            ├── cita-forever 1.0.0
            ├── cita-executor 20.2.0
            ├── cita-database 0.1.0
            │   ├── core-executor 0.1.0
            │   ├── core 0.1.0
            │   ├── common-types 0.1.0
            │   ├── cita-executor 20.2.0
            │   ├── cita-chain 20.2.0
            │   └── cita-auth 20.2.0
            ├── cita-chain 20.2.0
            ├── cita-bft 20.2.0
            ├── cita-auth 20.2.0
            └── chain-executor-mock 0.1.0

Crate:         smallvec
Version:       0.6.13
Title:         Buffer overflow in SmallVec::insert_many
Date:          2021-01-08
ID:            RUSTSEC-2021-0003
URL:           https://rustsec.org/advisories/RUSTSEC-2021-0003
Solution:      Upgrade to >=0.6.14, <1.0.0 OR >=1.6.1
Dependency tree:
smallvec 0.6.13

Crate:         smallvec
Version:       1.2.0
Title:         Buffer overflow in SmallVec::insert_many
Date:          2021-01-08
ID:            RUSTSEC-2021-0003
URL:           https://rustsec.org/advisories/RUSTSEC-2021-0003
Solution:      Upgrade to >=0.6.14, <1.0.0 OR >=1.6.1
Dependency tree:
smallvec 1.2.0

Crate:         ws
Version:       0.7.9
Title:         Insufficient size checks in outgoing buffer in ws allows remote attacker to run the process out of memory
Date:          2020-09-25
ID:            RUSTSEC-2020-0043
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0043
Solution:      No safe upgrade is available!
Dependency tree:
ws 0.7.9
└── cita-jsonrpc 20.2.0

Crate:         failure
Version:       0.1.7
Warning:       unmaintained
Title:         failure is officially deprecated/unmaintained
Date:          2020-05-02
ID:            RUSTSEC-2020-0036
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0036
Dependency tree:
failure 0.1.7
└── dotenv 0.13.0
    ├── pubsub 0.6.0
    │   ├── core 0.1.0
    │   │   ├── core-executor 0.1.0
    │   │   │   └── cita-executor 20.2.0
    │   │   ├── cita-relayer-parser 0.1.0
    │   │   └── cita-chain 20.2.0
    │   ├── cita-network 20.2.0
    │   ├── cita-jsonrpc 20.2.0
    │   ├── cita-executor 20.2.0
    │   ├── cita-chain 20.2.0
    │   ├── cita-bft 20.2.0
    │   ├── cita-auth 20.2.0
    │   └── chain-executor-mock 0.1.0
    ├── cita-network 20.2.0
    ├── cita-jsonrpc 20.2.0
    ├── cita-executor 20.2.0
    ├── cita-chain 20.2.0
    ├── cita-bft 20.2.0
    ├── cita-auth 20.2.0
    └── chain-executor-mock 0.1.0

Crate:         net2
Version:       0.2.33
Warning:       unmaintained
Title:         net2 crate has been deprecated; use socket2 instead
Date:          2020-05-01
ID:            RUSTSEC-2020-0016
URL:           https://rustsec.org/advisories/RUSTSEC-2020-0016
Dependency tree:
net2 0.2.33
├── miow 0.2.1
├── mio 0.6.21
│   ├── ws 0.7.9
│   │   └── cita-jsonrpc 20.2.0
│   ├── tokio-uds 0.2.6
│   │   └── tokio 0.1.22
│   ├── tokio-udp 0.1.6
│   │   └── tokio 0.1.22
│   ├── tokio-tcp 0.1.4
│   │   ├── tokio 0.1.22
│   │   └── hyper 0.12.35
│   ├── tokio-reactor 0.1.12
│   │   ├── tokio-uds 0.2.6
│   │   ├── tokio-udp 0.1.6
│   │   ├── tokio-tcp 0.1.4
│   │   ├── tokio-core 0.1.17
│   │   │   └── cita-jsonrpc 20.2.0
│   │   ├── tokio 0.1.22
│   │   └── hyper 0.12.35
│   ├── tokio-core 0.1.17
│   ├── tokio 0.2.22
│   ├── tokio 0.1.22
│   ├── notify 4.0.15
│   │   └── cita-network 20.2.0
│   ├── mio-uds 0.6.7
│   │   ├── tokio-uds 0.2.6
│   │   └── tokio 0.2.22
│   ├── mio-named-pipes 0.1.7
│   │   └── tokio 0.2.22
│   └── mio-extras 2.0.6
│       ├── ws 0.7.9
│       └── notify 4.0.15
├── hyper 0.12.35
└── cita-jsonrpc 20.2.0

Crate:         rust-crypto
Version:       0.2.36
Warning:       unmaintained
Title:         rust-crypto is unmaintained; switch to a modern alternative
Date:          2016-09-06
ID:            RUSTSEC-2016-0005
URL:           https://rustsec.org/advisories/RUSTSEC-2016-0005
Dependency tree:
rust-crypto 0.2.36
└── core-executor 0.1.0
    └── cita-executor 20.2.0

Crate:         tempdir
Version:       0.3.7
Warning:       unmaintained
Title:         tempdir crate has been deprecated; use tempfile instead
Date:          2018-02-13
ID:            RUSTSEC-2018-0017
URL:           https://rustsec.org/advisories/RUSTSEC-2018-0017
Dependency tree:
tempdir 0.3.7
├── core-executor 0.1.0
│   └── cita-executor 20.2.0
├── core 0.1.0
│   ├── core-executor 0.1.0
│   ├── cita-relayer-parser 0.1.0
│   └── cita-chain 20.2.0
├── cita-executor 20.2.0
└── cita-auth 20.2.0

Crate:         arc-swap
Version:       0.4.4
Warning:       yanked

Crate:         blake2b
Version:       0.1.0
Warning:       yanked
Dependency tree:
blake2b 0.1.0
└── hashable 0.1.0
    ├── proof 0.6.0
    │   ├── jsonrpc-proto 0.1.0
    │   │   └── cita-jsonrpc 20.2.0
    │   ├── core-executor 0.1.0
    │   │   └── cita-executor 20.2.0
    │   ├── core 0.1.0
    │   │   ├── core-executor 0.1.0
    │   │   ├── cita-relayer-parser 0.1.0
    │   │   └── cita-chain 20.2.0
    │   ├── common-types 0.1.0
    │   │   ├── core-executor 0.1.0
    │   │   ├── core 0.1.0
    │   │   ├── cita-executor 20.2.0
    │   │   └── cita-chain 20.2.0
    │   ├── cita-executor 20.2.0
    │   ├── cita-chain 20.2.0
    │   ├── cita-bft 20.2.0
    │   └── chain-executor-mock 0.1.0
    ├── libproto 0.6.0
    │   ├── tx_pool 0.6.0
    │   │   └── cita-auth 20.2.0
    │   ├── proof 0.6.0
    │   ├── jsonrpc-proto 0.1.0
    │   ├── create-genesis 0.1.0
    │   ├── core-executor 0.1.0
    │   ├── core 0.1.0
    │   ├── common-types 0.1.0
    │   ├── cita-relayer-parser 0.1.0
    │   ├── cita-network 20.2.0
    │   ├── cita-jsonrpc 20.2.0
    │   ├── cita-executor 20.2.0
    │   ├── cita-chain 20.2.0
    │   ├── cita-bft 20.2.0
    │   ├── cita-auth 20.2.0
    │   └── chain-executor-mock 0.1.0
    ├── create-key-addr 0.1.0
    ├── core-executor 0.1.0
    ├── core 0.1.0
    ├── common-types 0.1.0
    ├── cita-sm2 0.1.0
    │   └── cita-crypto 0.1.0
    │       ├── tx_pool 0.6.0
    │       ├── proof 0.6.0
    │       ├── libproto 0.6.0
    │       ├── engine 0.6.0
    │       │   └── cita-bft 20.2.0
    │       ├── create-key-addr 0.1.0
    │       ├── core-executor 0.1.0
    │       ├── core 0.1.0
    │       ├── common-types 0.1.0
    │       ├── cita-relayer-parser 0.1.0
    │       ├── cita-executor 20.2.0
    │       ├── cita-bft 20.2.0
    │       ├── cita-auth 20.2.0
    │       └── chain-executor-mock 0.1.0
    ├── cita-secp256k1 0.6.0
    │   ├── core-executor 0.1.0
    │   ├── core 0.1.0
    │   └── cita-crypto 0.1.0
    ├── cita-merklehash 0.1.0
    │   ├── libproto 0.6.0
    │   ├── core-executor 0.1.0
    │   └── core 0.1.0
    ├── cita-executor 20.2.0
    ├── cita-ed25519 0.6.0
    │   ├── core-executor 0.1.0
    │   ├── core 0.1.0
    │   ├── common-types 0.1.0
    │   ├── cita-executor 20.2.0
    │   └── cita-crypto 0.1.0
    ├── cita-bft 20.2.0
    ├── cita-auth 20.2.0
    └── chain-executor-mock 0.1.0

Crate:         bumpalo
Version:       3.2.0
Warning:       yanked

Crate:         miow
Version:       0.2.1
Warning:       yanked
Dependency tree:
miow 0.2.1

Crate:         miow
Version:       0.3.5
Warning:       yanked
Dependency tree:
miow 0.3.5

Crate:         net2
Version:       0.2.33
Warning:       yanked

Crate:         socket2
Version:       0.3.12
Warning:       yanked
Dependency tree:
socket2 0.3.12
├── miow 0.3.5
└── hyper 0.13.7

Crate:         tentacle-discovery
Version:       0.2.9
Warning:       yanked
Dependency tree:
tentacle-discovery 0.2.9
└── cita-network 20.2.0

error: 9 vulnerabilities found!
warning: 12 allowed warnings found
"

cargo audit --ignore RUSTSEC-2020-0091 \
            --ignore RUSTSEC-2020-0006 \
            --ignore RUSTSEC-2020-0060 \
            --ignore RUSTSEC-2020-0059 \
            --ignore RUSTSEC-2019-0027 \
            --ignore RUSTSEC-2020-0082 \
            --ignore RUSTSEC-2021-0003 \
            --ignore RUSTSEC-2020-0043
