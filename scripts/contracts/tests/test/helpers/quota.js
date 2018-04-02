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

// setBQL
const setBQL = function (value, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.setBQL.sendTransaction(
            value,
            tx_params
        );
};

// setDefaultAQL
const setDefaultAQL = function (value, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.setDefaultAQL.sendTransaction(
            value,
            tx_params
        );
};

// setAQL
const setAQL = function (account, value, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return quotaContractIns.setAQL.sendTransaction(
            account,
            value,
            tx_params
        );
};

// isAdmin
const isAdmin = function (account) {
    return quotaContractIns.isAdmin.call(account);
};

// getAccounts
const getAccounts = function () {
    return quotaContractIns.getAccounts.call();
};

// getQuotas
const getQuotas = function () {
    return quotaContractIns.getQuotas.call();
};

// getBQL
const getBQL = function () {
    return quotaContractIns.getBQL.call();
};

// getDefaultAQL
const getDefaultAQL = function () {
    return quotaContractIns.getDefaultAQL.call();
};

// getAQL
const getAQL = function (account) {
    return quotaContractIns.getAQL.call(account);
};

module.exports = {
    admin,
    addAdmin,
    setBQL,
    setDefaultAQL,
    setAQL,
    isAdmin,
    getAccounts,
    getQuotas,
    getBQL,
    getDefaultAQL,
    getAQL
};
