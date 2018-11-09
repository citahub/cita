const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { quota } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/QuotaManager.abi'));

const contract = genContract(abi, quota);

// setBQL
const setBQL = async (value, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.setBQL(value).send(param);
};

// setDefaultAQL
const setDefaultAQL = async (value, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.setDefaultAQL(value).send(param);
};

// setAQL
const setAQL = async (account, value, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.setAQL(account, value).send(param);
};

// getAccounts
const getAccounts = () => contract.methods.getAccounts().call('pending');

// getQuotas
const getQuotas = () => contract.methods.getQuotas().call('pending');

// getBQL
const getBQL = () => contract.methods.getBQL().call('pending');

// getDefaultAQL
const getDefaultAQL = () => contract.methods.getDefaultAQL().call('pending');

// getAQL
const getAQL = account => contract.methods.getAQL(account).call('pending');

module.exports = {
  setBQL,
  setDefaultAQL,
  setAQL,
  getAccounts,
  getQuotas,
  getBQL,
  getDefaultAQL,
  getAQL,
};
