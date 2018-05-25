const util = require('./util');
const config = require('../config');

const { web3, genTxParams } = util;

const sender = config.contract.authorization.superAdmin;
const { abi, addr, admin } = config.contract.quota;

const quotaManager = web3.eth.contract(abi);
const quotaContractIns = quotaManager.at(addr);

// addAdmin
const addAdmin = function addAdmin(account, _sender = sender) {
  return quotaContractIns.addAdmin.sendTransaction(
    account,
    genTxParams(_sender),
  );
};

// setBQL
const setBQL = function setBQL(value, _sender = sender) {
  return quotaContractIns.setBQL.sendTransaction(
    value,
    genTxParams(_sender),
  );
};

// setDefaultAQL
const setDefaultAQL = function setDefaultAQL(value, _sender = sender) {
  return quotaContractIns.setDefaultAQL.sendTransaction(
    value,
    genTxParams(_sender),
  );
};

// setAQL
const setAQL = function setAQL(account, value, _sender = sender) {
  return quotaContractIns.setAQL.sendTransaction(
    account,
    value,
    genTxParams(_sender),
  );
};

// isAdmin
const isAdmin = function isAdmin(account) {
  return quotaContractIns.isAdmin.call(account);
};

// getAccounts
const getAccounts = function getAccounts() {
  return quotaContractIns.getAccounts.call();
};

// getQuotas
const getQuotas = function getQuotas() {
  return quotaContractIns.getQuotas.call();
};

// getBQL
const getBQL = function getBQL() {
  return quotaContractIns.getBQL.call();
};

// getDefaultAQL
const getDefaultAQL = function getDefaultAQL() {
  return quotaContractIns.getDefaultAQL.call();
};

// getAQL
const getAQL = function getAQL(account) {
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
  getAQL,
};
