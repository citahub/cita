/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr, admin } = config.contract.quota;

const quotaManager = web3.eth.contract(abi);
const quotaContractIns = quotaManager.at(addr);

const quota = util.quota;
const blockLimit = util.blockLimit;

const tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: web3.eth.blockNumber + blockLimit,
    from: sender.address
};

// addAdmin
const addAdmin = function (account, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.addAdmin.sendTransaction(
            account,
            tx_params
        );
};

// setBlockGasLimit
const setBlockGasLimit = function (value, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.setBlockGasLimit.sendTransaction(
            value,
            tx_params
        );
};

// setGlobalAccountGasLimit
const setGlobalAccountGasLimit = function (value, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.setGlobalAccountGasLimit.sendTransaction(
            value,
            tx_params
        );
};

// setAccountGasLimit
const setAccountGasLimit = function (account, value, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.setAccountGasLimit.sendTransaction(
            account,
            value,
            tx_params
        );
};

// isAdmin
const isAdmin = function (account) {
    return quotaContractIns.isAdmin.call(account);
};

// getSpecialUsers
const getSpecialUsers = function () {
    return quotaContractIns.getSpecialUsers.call();
};

// getUsersQuota
const getUsersQuota = function () {
    return quotaContractIns.getUsersQuota.call();
};

// getBlockGasLimit
const getBlockGasLimit = function () {
    return quotaContractIns.getblockGasLimit.call();
};

// getAccountGasLimit
const getAccountGasLimit = function () {
    return quotaContractIns.getAccountGasLimit.call();
};

// getAccountQuota
const getAccountQuota = function (account) {
    return quotaContractIns.getAccountQuota.call(account);
};

module.exports = {
    admin,
    addAdmin,
    setBlockGasLimit,
    setGlobalAccountGasLimit,
    setAccountGasLimit,
    isAdmin,
    getSpecialUsers,
    getUsersQuota,
    getBlockGasLimit,
    getAccountGasLimit,
    getAccountQuota
};
