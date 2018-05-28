const util = require('./util');
const config = require('../config');

const { web3 } = util;

const { gABI, gAddr } = config.contract.group;

const group = web3.eth.contract(gABI);
const gContractInstance = group.at(gAddr);

// queryInfo
const queryInfo = function queryInfo() {
  return gContractInstance.queryInfo.call();
};

// queryAccounts
const queryAccounts = function queryAccounts() {
  return gContractInstance.queryAccounts.call();
};

// queryParent
const queryParent = function queryParent() {
  return gContractInstance.queryParent.call();
};

// inGroup
const inGroup = function inGroup(account) {
  return gContractInstance.inGroup.call(account);
};

module.exports = {
  group,
  queryInfo,
  queryAccounts,
  queryParent,
  inGroup,
};
