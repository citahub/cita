const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { abi, addr } = config.contract.group;

const contract = genContract(abi, addr);

// queryInfo
const queryInfo = () => contract.methods.queryInfo().call('pending');

// queryAccounts
const queryAccounts = () => contract.methods.queryAccounts().call('pending');

// queryParent
const queryParent = () => contract.methods.queryParent().call('pending');

// inGroup
const inGroup = account => contract.methods.inGroup(account).call('pending');

module.exports = {
  queryInfo,
  queryAccounts,
  queryParent,
  inGroup,
};
