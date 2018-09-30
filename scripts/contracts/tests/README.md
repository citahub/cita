# Test contract using web3js

## 0. Setup

### 0.0. Install node

***use latest version (require version > 8.0.0)***

Use nvm to manage the node version:

```
wget -qO- https://raw.github.com/creationix/nvm/v0.4.0/install.sh | sh
```

Then install the latest version:

```
nvm install --latest-npm
```

### 0.1. Install node modules

* [Install yarn](https://yarnpkg.com/lang/en/docs/install/)
    - configure the repository:
        ```
        curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
        echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
        ```
    - run:
        ```
        sudo apt-get update && sudo apt-get install yarn
        ```
* cd the `tests` dir, then run:

```
yarn install
```

## 1. Test

### 1.0 Run the cita

Check the doc of CITA [getting_started](https://docs.nervos.org/cita/#/chain/getting_started),

### 1.1 Run the test

Use mocha framework:
[usage of mochajs](https://mochajs.org/#usage)

Some options of mocha usage:

* `-t 60s`: set test-case timeout
* `-g <pattern>`: only run tests matching <pattern>

Some test command:

* To test permission contract:

```
npm run-script unit_permission
```

* To test permission management contract:

```
npm run-script unit_pm
```

* To test group contract

```
npm run-script unit_group
```

* To test group management contract

```
npm run-script unit_gm
```

* To test role management contract

```
npm run-script unit_rm
```

* To test authorization contract

```
npm run-script unit_auth
```

* To test node manager contract

```
npm run-script unit_node
```

* To test quota manager contract

```
npm run-script unit_quota
```

* To test chain manager contract

```
npm run-script unit_chain
```

* To test abi

```
npm run-script abi
```

* To test admin

```
npm run-script unit_admin
```

* To test uint8 op of solidity

```
npm run-script uint8
```

* To test integrate quota

*Should set the checkQuota be true. [config_tool](https://docs.nervos.org/cita/#/chain/config_tool?id=%E4%B8%BB%E8%A6%81%E5%8A%9F%E8%83%BD)*

```
npm run-script integrate_quota
```

* To test amend abi

```
npm run-script abi
```

* To test store

```
npm run-script store
```

* To test batch_tx

```
npm run-script batch_tx
```

* To test integrate permission

*Should set the checkPermission be true. [config_tool](https://docs.nervos.org/cita/#/chain/config_tool?id=%E4%B8%BB%E8%A6%81%E5%8A%9F%E8%83%BD)*

```
npm run-script permission
```

* To lint test directory:

```
npm run-script lint
```

* To fix lint of test directory:

```
npm run-script lint-fix
```

## 2. Which contracts?

Include:

* Permission management (path: `../src/permission_management`):
    - permission
    - authorization
    - permission_management
    - permission_creator

* Role management (path: `../src/role_management`):
    - role
    - role_auth
    - role_management
    - role_creator

* User management (path: `../src/user_management`):
    - group
    - group_creator
    - group_management

* Quota manager (path: `../src/system/quota_manager.sol`):

* Node manager (path: `../src/system/node_manager.sol`):

* chain manager (path: `../src/system/chain_manager.sol`):

* sys config (path: `../src/system/sys_config.sol`):

* batch tx (path: `../src/system/batch_tx.sol`):

# Other directory

* `contracts` is some contracts used for test
* `doc` is some docs about test using `txtool`
