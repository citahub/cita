const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { abi, addr } = config.contract.group;

const contract = genContract(abi, addr);

// queryInfo
const queryInfo = () => contract.methods.queryInfo().call();

// queryAccounts
const queryAccounts = () => contract.methods.queryAccounts().call();

// queryParent
const queryParent = () => contract.methods.queryParent().call();

// inGroup
const inGroup = account => contract.methods.inGroup(account).call();

module.exports = {
  queryInfo,
  queryAccounts,
  queryParent,
  inGroup,
};
