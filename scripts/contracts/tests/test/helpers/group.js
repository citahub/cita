const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { group } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/Group.abi'));

const contract = genContract(abi, group);

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
  abi,
};
