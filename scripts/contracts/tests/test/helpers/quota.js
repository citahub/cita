/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr, admin } = config.contract.quota;

const quotaManager = web3.eth.contract(abi);
const quotaContractIns = quotaManager.at(addr);

const genTxParams = util.genTxParams;

// addAdmin
const addAdmin = function (account, _sender = sender) {
    return quotaContractIns.addAdmin.sendTransaction(
                account,
                genTxParams(_sender)
            );
};

// setBQL
const setBQL = function (value, _sender = sender) {
    return quotaContractIns.setBQL.sendTransaction(
                value,
                genTxParams(_sender)
            );
};

// setDefaultAQL
const setDefaultAQL = function (value, _sender = sender) {
    return quotaContractIns.setDefaultAQL.sendTransaction(
                value,
                genTxParams(_sender)
            );
};

// setAQL
const setAQL = function (account, value, _sender = sender) {
    return quotaContractIns.setAQL.sendTransaction(
                account,
                value,
                genTxParams(_sender)
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
