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

Use the script:

```
tests/integrate_test/cita_start.sh
```

Or other ways. 
Check the doc of CITA [getting_started](https://cryptape.github.io/cita/getting_started/).

### 1.1 Run the test

Use mocha framework:
[usage of mochajs](https://mochajs.org/#usage)

Some options of mocha usage: 

* `-t 60s`: set test-case timeout
* `-g <pattern>`: only run tests matching <pattern>

* To run all tests:

```
npm test
```

* To run uint test:

```
npm run-script unit_test
```

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

* To test role contract

```
npm run-script unit_role
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

* To run integrate test:
    - Should set the check_permission be true(scripts/admintool/init_data)example.json):

    ```
    "0x0000000000000000000000000000000031415926": [
        1,
        true,
        false
    ],
    ```

    - Run:

    ```
    npm run-script integrate_test
    ```

* To lint test directory:

```
npm run-script lint
```

## 2. Which contracs?

Include:

* Permission management (path: `cd ../permission_management`):
    - authorization
    - permission
    - permission_management
    - role_management

# Other directory

* `contracts` is some contracts using for test
* `doc` is some docs about test using `txtool`
