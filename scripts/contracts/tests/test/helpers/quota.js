const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const sender = config.superAdmin;
const { abi, addr } = config.contract.quota;

const contract = genContract(abi, addr);

// setBQL
const setBQL = async (value, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.setBQL(value).send(param);
};

// setDefaultAQL
const setDefaultAQL = async (value, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.setDefaultAQL(value).send(param);
};

// setAQL
const setAQL = async (account, value, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.setAQL(account, value).send(param);
};

// getAccounts
const getAccounts = () => contract.methods.getAccounts().call();

// getQuotas
const getQuotas = () => contract.methods.getQuotas().call();

// getBQL
const getBQL = () => contract.methods.getBQL().call();

// getDefaultAQL
const getDefaultAQL = () => contract.methods.getDefaultAQL().call();

// getAQL
const getAQL = account => contract.methods.getAQL(account).call();

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
