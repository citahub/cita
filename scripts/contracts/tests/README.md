# Test permission system

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

cd the `tests` dir, then run:

```
npm install
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

# Other directory

* `contracts` is some contracts using for test
* `doc` is some docs about test using `txtool`
