const util = require('./util');
const config = require('../config');

const { genContract } = util;

const {
  abi, addr,
} = config.contract.authorization;

const contract = genContract(abi, addr);

// queryPermissions
const queryPermissions = account => contract.methods.queryPermissions(account).call();

// queryAccounts
const queryAccounts = perm => contract.methods.queryAccounts(perm).call();

// checkResource
const checkResource = (account, cont, func) => contract.methods.checkResource(
  account,
  cont,
  func,
).call();

// queryAllAccounts
const queryAllAccounts = () => contract.methods.queryAllAccounts().call();

module.exports = {
  queryPermissions,
  queryAccounts,
  checkResource,
  queryAllAccounts,
};
