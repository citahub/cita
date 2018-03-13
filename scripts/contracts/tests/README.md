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

### 1.1 Run the test

Use mocha framework:
[usage of mochajs](https://mochajs.org/#usage)

Run:

```
npm test
```

Some options of mocha usage: 

* `-t 60s`: set test-case timeout
* `-g <pattern>`: only run tests matching <pattern>

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

# Notice

**Should set the check_permission true when test test/integrate**
